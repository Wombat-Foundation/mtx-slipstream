//! Minimal typed-endpoint machinery: requests, responses and HTTP conversion.
//!
//! An endpoint is a request type and a response type described once with
//! [`endpoint!`](macro@crate::endpoint). The macro implements [`EndpointRequest`] and
//! [`EndpointResponse`]; blanket impls then provide the ruma-style
//! `OutgoingRequest`, `IncomingResponse`, `IncomingRequest` and
//! `OutgoingResponse` traits in both directions.

use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use core::fmt::{self, Write as _};

use bytes::BufMut;

use crate::{
	api::{
		client::error::{Error, ErrorBody, ErrorKind},
		error::IntoHttpError,
	},
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

/// Matrix specification versions an endpoint may be available in.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatrixVersion {
	V1_0,
	V1_1,
	V1_2,
	V1_3,
	V1_4,
	V1_5,
	V1_6,
	V1_7,
	V1_8,
	V1_9,
	V1_10,
	V1_11,
	V1_12,
	V1_13,
	V1_14,
	V1_15,
}

/// How an access token is attached to an outgoing request.
#[derive(Clone, Copy, Debug)]
pub enum SendAccessToken<'a> {
	IfRequired(&'a str),
	Always(&'a str),
	Appservice(&'a str),
	None,
}

impl<'a> SendAccessToken<'a> {
	/// The token to send, given whether the endpoint requires authentication.
	#[must_use]
	pub fn get_required_for_endpoint(self) -> Option<&'a str> {
		match self {
			Self::IfRequired(token) | Self::Always(token) | Self::Appservice(token) => {
				Some(token)
			}
			Self::None => None,
		}
	}
}

/// How an endpoint authenticates its caller.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AuthScheme {
	/// No credentials.
	None,
	/// A user access token.
	AccessToken,
	/// A user access token if present, otherwise unauthenticated.
	AccessTokenOptional,
	/// An appservice access token.
	AppserviceToken,
	/// An `X-Matrix` federation signature.
	ServerSignatures,
}

impl AuthScheme {
	/// Whether a request carrying no credentials is let through to the handler.
	///
	/// `None`, `AccessTokenOptional` and `AppserviceToken` admit anonymous callers (the
	/// last because `/register` and `/login` are reachable without a token);
	/// `AccessToken` and `ServerSignatures` do not.
	#[must_use]
	pub const fn allows_anonymous(self) -> bool {
		matches!(self, Self::None | Self::AccessTokenOptional | Self::AppserviceToken)
	}
}

#[path = "endpoint_auth.rs"]
mod auth_table;

#[cfg(test)]
#[path = "endpoint_auth_spec.rs"]
mod auth_spec;

/// The declared authentication of `method` + `path`, if the table has it.
///
/// Placeholders (`{name}`) match each other regardless of name.
#[must_use]
pub const fn lookup_auth(method: &str, path: &str) -> Option<AuthScheme> {
	if let Some(scheme) = lookup_sorted(EXTRA_AUTH, method, path) {
		return Some(scheme);
	}
	lookup_sorted(auth_table::AUTH_TABLE, method, path)
}

/// Hand-reviewed entries that take precedence over the generated table.
///
/// - Push rules: ruma only knew the `global` scope; ours takes any `{scope}`.
/// - Antispam: requests we *send* to spam checkers; the router never serves them.
/// - `event_relationships` (MSC2836): federation, signed.
/// - Federation `query` and `exchange_third_party_invite`: ruwuma declared
///   `AccessToken`, but both are signed federation requests in the spec and in synapse.
const EXTRA_AUTH: &[(&str, &str, AuthScheme)] = &[
	("DELETE", "/_matrix/client/v3/pushrules/{}/{}/{}", AuthScheme::AccessToken),
	("GET", "/_matrix/client/v3/pushrules/{}/{}/{}", AuthScheme::AccessToken),
	("PUT", "/_matrix/client/v3/pushrules/{}/{}/{}", AuthScheme::AccessToken),
	("GET", "/_matrix/client/v3/pushrules/{}/{}/{}/actions", AuthScheme::AccessToken),
	("PUT", "/_matrix/client/v3/pushrules/{}/{}/{}/actions", AuthScheme::AccessToken),
	("GET", "/_matrix/client/v3/pushrules/{}/{}/{}/enabled", AuthScheme::AccessToken),
	("PUT", "/_matrix/client/v3/pushrules/{}/{}/{}/enabled", AuthScheme::AccessToken),
	("POST", "/_matrix/federation/unstable/event_relationships", AuthScheme::ServerSignatures),
	(
		"PUT",
		"/_matrix/federation/v1/exchange_third_party_invite/{}",
		AuthScheme::ServerSignatures,
	),
	("GET", "/_matrix/federation/v1/query/{}", AuthScheme::ServerSignatures),
	("POST", "/_meowlnir/antispam/{}/accept_make_join", AuthScheme::None),
	("POST", "/_meowlnir/antispam/{}/user_may_invite", AuthScheme::None),
	("POST", "/_meowlnir/antispam/{}/user_may_join_room", AuthScheme::None),
	("POST", "/api/1/spam_check/user_may_invite", AuthScheme::None),
	("POST", "/api/1/spam_check/user_may_join_room", AuthScheme::None),
];

/// Binary search over a table sorted by `(path, method)`.
///
/// Every endpoint evaluates this during const evaluation, so it must stay
/// logarithmic: a linear scan made the whole crate's check time quadratic in the
/// number of endpoints.
const fn lookup_sorted(
	table: &[(&str, &str, AuthScheme)],
	method: &str,
	path: &str,
) -> Option<AuthScheme> {
	let (mut lo, mut hi) = (0, table.len());
	while lo < hi {
		let mid = lo.saturating_add(hi.saturating_sub(lo) / 2);
		let (m, p, scheme) = table[mid];
		let mut ord = template_cmp(p.as_bytes(), path.as_bytes());
		if ord == 0 {
			ord = bytes_cmp(m.as_bytes(), method.as_bytes());
		}
		if ord == 0 {
			return Some(scheme);
		} else if ord < 0 {
			lo = mid.saturating_add(1);
		} else {
			hi = mid;
		}
	}
	None
}

/// Whether `table` is strictly sorted by `(path, method)`, as `lookup_sorted` requires.
const fn is_sorted(table: &[(&str, &str, AuthScheme)]) -> bool {
	let mut i = 1;
	while i < table.len() {
		let (pm, pp, _) = table[i.saturating_sub(1)];
		let (m, p, _) = table[i];
		let mut ord = template_cmp(pp.as_bytes(), p.as_bytes());
		if ord == 0 {
			ord = bytes_cmp(pm.as_bytes(), m.as_bytes());
		}
		if ord >= 0 {
			return false;
		}
		i = i.saturating_add(1);
	}
	true
}

const _: () =
	assert!(is_sorted(auth_table::AUTH_TABLE), "AUTH_TABLE must be sorted by (path, method)");

const fn bytes_cmp(a: &[u8], b: &[u8]) -> i8 {
	let mut i = 0;
	while i < a.len() && i < b.len() {
		if a[i] != b[i] {
			return if a[i] < b[i] {
				-1
			} else {
				1
			};
		}
		i = i.saturating_add(1);
	}
	if a.len() == b.len() {
		0
	} else if a.len() < b.len() {
		-1
	} else {
		1
	}
}

/// Orders path templates, treating every `{name}` placeholder as `{}`.
const fn template_cmp(a: &[u8], b: &[u8]) -> i8 {
	let (mut i, mut j) = (0, 0);
	while i < a.len() && j < b.len() {
		if a[i] == b'{' && b[j] == b'{' {
			while i < a.len() && a[i] != b'}' {
				i = i.saturating_add(1);
			}
			while j < b.len() && b[j] != b'}' {
				j = j.saturating_add(1);
			}
		} else if a[i] != b[j] {
			return if a[i] < b[j] {
				-1
			} else {
				1
			};
		}
		i = i.saturating_add(1);
		j = j.saturating_add(1);
	}
	if i >= a.len() && j >= b.len() {
		0
	} else if i >= a.len() {
		-1
	} else {
		1
	}
}

/// Static description of an endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Metadata {
	pub method: &'static str,
	pub path: &'static str,
	pub authentication: AuthScheme,
	/// Further paths the same endpoint is served under (for example the stable
	/// path of an endpoint whose canonical path is still the unstable one).
	pub aliases: &'static [&'static str],
}

impl Metadata {
	/// Adds alias paths. Each must have an authentication entry for the same
	/// method, like the canonical path.
	///
	/// # Panics
	///
	/// Panics (at compile time in const contexts) if an alias has no declared
	/// authentication.
	#[must_use]
	pub const fn with_aliases(mut self, aliases: &'static [&'static str]) -> Self {
		let mut i = 0;
		while i < aliases.len() {
			assert!(
				lookup_auth(self.method, aliases[i]).is_some(),
				"no authentication declared for an endpoint alias; add it to endpoint_auth.rs"
			);
			i = i.saturating_add(1);
		}
		self.aliases = aliases;
		self
	}

	/// Describes an endpoint, looking up its authentication.
	///
	/// There is deliberately no default: an endpoint missing from the table fails to
	/// compile (const evaluation panics) instead of silently picking a policy.
	///
	/// # Panics
	///
	/// Panics (at compile time in const contexts) if no authentication is declared
	/// for `method` and `path`.
	#[must_use]
	pub const fn new(method: &'static str, path: &'static str) -> Self {
		let Some(authentication) = lookup_auth(method, path) else {
			panic!("no authentication declared for this endpoint; add it to endpoint_auth.rs");
		};
		Self {
			method,
			path,
			authentication,
			aliases: &[],
		}
	}
}

/// An error returned by the remote endpoint, parsed from an HTTP response.
pub trait EndpointError: Sized + fmt::Debug + fmt::Display {
	fn from_http_response<T: AsRef<[u8]>>(response: http::Response<T>) -> Self;
}

impl EndpointError for Error {
	fn from_http_response<T: AsRef<[u8]>>(response: http::Response<T>) -> Self {
		let status_code = response.status();
		let value = core::str::from_utf8(response.body().as_ref())
			.ok()
			.and_then(|text| Value::parse(text).ok());
		let body = value.as_ref().and_then(Value::as_object).map_or(ErrorBody::Other, |object| {
			let errcode = object.get("errcode").and_then(Value::as_str).unwrap_or("M_UNKNOWN");
			let message = object.get("error").and_then(Value::as_str).unwrap_or_default();
			ErrorBody::Standard {
				kind: ErrorKind::from_errcode(errcode),
				message: message.to_string(),
			}
		});
		Self {
			status_code,
			body,
		}
	}
}

/// Error converting an HTTP response into a typed response.
#[derive(Debug)]
pub enum FromHttpResponseError<E> {
	/// The server returned an error response.
	Server(E),
	/// The body did not match the expected shape.
	Deserialization(DeError),
}

impl<E: fmt::Display> fmt::Display for FromHttpResponseError<E> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Server(e) => e.fmt(f),
			Self::Deserialization(e) => write!(f, "deserialization failed: {e}"),
		}
	}
}
impl<E: fmt::Debug + fmt::Display> core::error::Error for FromHttpResponseError<E> {}

impl<E> From<DeError> for FromHttpResponseError<E> {
	fn from(e: DeError) -> Self {
		Self::Deserialization(e)
	}
}

/// Error converting an HTTP request into a typed request.
#[derive(Debug)]
pub enum FromHttpRequestError {
	Deserialization(DeError),
	MethodMismatch,
}

impl fmt::Display for FromHttpRequestError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Deserialization(e) => write!(f, "deserialization failed: {e}"),
			Self::MethodMismatch => f.write_str("HTTP method mismatch"),
		}
	}
}
impl core::error::Error for FromHttpRequestError {}
impl From<DeError> for FromHttpRequestError {
	fn from(e: DeError) -> Self {
		Self::Deserialization(e)
	}
}

/// A request that can be sent to a remote server.
pub trait OutgoingRequest: Sized {
	const METADATA: Metadata;
	type EndpointError: EndpointError;
	type IncomingResponse: IncomingResponse<EndpointError = Self::EndpointError>;

	/// # Errors
	///
	/// Returns an error if the request cannot be encoded.
	fn try_into_http_request<B: Default + BufMut>(
		self,
		base_url: &str,
		access_token: SendAccessToken<'_>,
		considering_versions: &[MatrixVersion],
	) -> Result<http::Request<B>, IntoHttpError>;
}

/// A response received from a remote server.
pub trait IncomingResponse: Sized {
	type EndpointError: EndpointError;

	/// # Errors
	///
	/// Returns an error for a non-success status or a malformed body.
	fn try_from_http_response<T: AsRef<[u8]>>(
		response: http::Response<T>,
	) -> Result<Self, FromHttpResponseError<Self::EndpointError>>;
}

/// A request received from a remote client or server.
pub trait IncomingRequest: Sized {
	type EndpointError: EndpointError;
	type OutgoingResponse: OutgoingResponse;

	/// # Errors
	///
	/// Returns an error if the request does not match the endpoint.
	fn try_from_http_request<B: AsRef<[u8]>, S: AsRef<str>>(
		request: http::Request<B>,
		path_args: &[S],
	) -> Result<Self, FromHttpRequestError>;
}

/// A response that can be sent back to a remote caller.
pub trait OutgoingResponse {
	/// # Errors
	///
	/// Returns an error if the response cannot be encoded.
	fn try_into_http_response<B: Default + BufMut>(
		self,
	) -> Result<http::Response<B>, IntoHttpError>;
}

/// Request half of an endpoint, implemented by [`endpoint!`](macro@crate::endpoint).
pub trait EndpointRequest: Sized {
	type Response: IncomingResponse<EndpointError = Error> + OutgoingResponse;
	const METADATA: Metadata;

	/// Values for each `{placeholder}` in the path, in order.
	fn path_args(&self) -> Vec<String>;
	fn query(&self) -> Vec<(String, String)>;
	fn body(&self) -> Option<Value>;
	/// # Errors
	///
	/// Returns an error if the request cannot be encoded.
	fn try_into_http_request_raw<B: Default + BufMut>(
		self,
		base_url: &str,
		access_token: SendAccessToken<'_>,
		considering_versions: &[MatrixVersion],
	) -> Result<http::Request<B>, IntoHttpError> {
		let mut url = alloc::format!(
			"{}{}",
			base_url.trim_end_matches('/'),
			build_path(Self::METADATA.path, &self.path_args())
		);
		let query = self.query();
		if !query.is_empty() {
			let pairs: Vec<String> = query
				.iter()
				.map(|(k, v)| alloc::format!("{}={}", percent_encode(k), percent_encode(v)))
				.collect();
			url.push('?');
			url.push_str(&pairs.join("&"));
		}
		let mut builder = http::Request::builder().method(Self::METADATA.method).uri(url);
		if let Some(token) = access_token.get_required_for_endpoint() {
			builder =
				builder.header(http::header::AUTHORIZATION, alloc::format!("Bearer {token}"));
		}
		let mut buf = B::default();
		if let Some(body) = self.body() {
			builder = builder.header(http::header::CONTENT_TYPE, "application/json");
			buf.put_slice(crate::codec::to_string(&body).as_bytes());
		}
		let _ = considering_versions;
		builder.body(buf).map_err(|e| IntoHttpError(e.to_string()))
	}
	/// # Errors
	///
	/// Returns an error if a required value is missing or malformed.
	fn from_parts(
		path: &[String],
		query: &[(String, String)],
		body: Option<&Value>,
	) -> Result<Self, DeError>;

	/// Parses an incoming request body. Endpoints with non-JSON bodies may override this.
	///
	/// # Errors
	///
	/// Returns an error if the method, body, or endpoint parameters are invalid.
	fn from_http_parts<B: AsRef<[u8]>, S: AsRef<str>>(
		request: &http::Request<B>,
		path_args: &[S],
	) -> Result<Self, FromHttpRequestError> {
		if request.method().as_str() != Self::METADATA.method {
			return Err(FromHttpRequestError::MethodMismatch);
		}
		let path: Vec<String> = path_args.iter().map(|s| s.as_ref().to_string()).collect();
		let query = parse_query(request.uri());
		let bytes = request.body().as_ref();
		let body = if bytes.is_empty() {
			None
		} else {
			let text = core::str::from_utf8(bytes).map_err(|e| DeError(e.to_string()))?;
			Some(Value::parse(text).map_err(|e| DeError(e.to_string()))?)
		};
		Ok(Self::from_parts(&path, &query, body.as_ref())?)
	}
}

/// Response half of an endpoint, implemented by [`endpoint!`](macro@crate::endpoint).
pub trait EndpointResponse: Sized {
	/// Whether a response body with a repeated object key, at any depth, is
	/// rejected while it is first parsed. Parsing keeps the last duplicate, so
	/// by the time `from_body` sees the value the duplicate is already gone.
	/// Responses whose signatures cover the body (MSC4499 key responses) opt in.
	const REJECT_DUPLICATE_KEYS: bool = false;

	fn to_body(&self) -> Value;
	/// # Errors
	///
	/// Returns an error if `body` does not match the response shape.
	fn from_body(body: &Value) -> Result<Self, DeError>;
}

/// Renders a value as a path or query parameter.
#[must_use]
pub fn to_param<T: Serialize>(value: &T) -> Option<String> {
	match value.to_json() {
		Value::Null => None,
		Value::String(s) => Some(s),
		other => crate::json::write_string_value(&other).ok(),
	}
}

/// Parses a path or query parameter.
///
/// # Errors
///
/// Returns an error if the parameter does not match `T`.
pub fn from_param<T: Deserialize>(param: Option<&str>) -> Result<T, DeError> {
	let Some(param) = param else {
		return T::from_json(&Value::Null);
	};
	T::from_json(&Value::String(param.to_string()))
		.or_else(|_| T::from_json(&Value::parse(param).unwrap_or(Value::Null)))
}

fn percent_encode(input: &str) -> String {
	let mut out = String::with_capacity(input.len());
	for byte in input.bytes() {
		if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
			out.push(char::from(byte));
		} else {
			let _ = write!(out, "%{byte:02X}");
		}
	}
	out
}

fn build_path(template: &str, args: &[String]) -> String {
	let mut out = String::new();
	let mut args = args.iter();
	let mut chars = template.chars();
	while let Some(c) = chars.next() {
		if c == '{' {
			for c in chars.by_ref() {
				if c == '}' {
					break;
				}
			}
			out.push_str(&percent_encode(args.next().map_or("", String::as_str)));
		} else {
			out.push(c);
		}
	}
	out
}

/// Builds the request URL: the path template filled with `args`, then the query.
#[must_use]
pub fn request_url(
	base_url: &str,
	template: &str,
	args: &[String],
	query: &[(String, String)],
) -> String {
	let mut url =
		alloc::format!("{}{}", base_url.trim_end_matches('/'), build_path(template, args));
	if !query.is_empty() {
		let pairs: Vec<String> = query
			.iter()
			.map(|(k, v)| alloc::format!("{}={}", percent_encode(k), percent_encode(v)))
			.collect();
		url.push('?');
		url.push_str(&pairs.join("&"));
	}
	url
}

/// Decodes `%XX` escapes; malformed escapes are kept literally.
#[must_use]
pub fn percent_decode(input: &str) -> String {
	let bytes = input.as_bytes();
	let mut out = Vec::with_capacity(bytes.len());
	let mut i = 0;
	while let Some(&byte) = bytes.get(i) {
		if byte == b'+' {
			out.push(b' ');
			i = i.saturating_add(1);
			continue;
		}
		let escaped = (byte == b'%')
			.then(|| bytes.get(i.saturating_add(1)..i.saturating_add(3)))
			.flatten()
			.and_then(|hex| core::str::from_utf8(hex).ok())
			.and_then(|hex| u8::from_str_radix(hex, 16).ok());
		if let Some(decoded) = escaped {
			out.push(decoded);
			i = i.saturating_add(3);
		} else {
			out.push(byte);
			i = i.saturating_add(1);
		}
	}
	String::from_utf8_lossy(&out).into_owned()
}

/// The decoded key/value pairs of a request URI's query string.
#[must_use]
pub fn parse_query(uri: &http::Uri) -> Vec<(String, String)> {
	uri.query()
		.unwrap_or_default()
		.split('&')
		.filter(|pair| !pair.is_empty())
		.map(|pair| {
			let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
			(percent_decode(k), percent_decode(v))
		})
		.collect()
}

impl<T: EndpointRequest> OutgoingRequest for T {
	const METADATA: Metadata = T::METADATA;
	type EndpointError = Error;
	type IncomingResponse = T::Response;

	fn try_into_http_request<B: Default + BufMut>(
		self,
		base_url: &str,
		access_token: SendAccessToken<'_>,
		considering_versions: &[MatrixVersion],
	) -> Result<http::Request<B>, IntoHttpError> {
		EndpointRequest::try_into_http_request_raw(
			self,
			base_url,
			access_token,
			considering_versions,
		)
		/*
		let mut url = alloc::format!(
			"{}{}",
			base_url.trim_end_matches('/'),
			build_path(T::METADATA.path, &self.path_args())
		);
		let query = self.query();
		if !query.is_empty() {
			let pairs: Vec<String> = query
				.iter()
				.map(|(k, v)| alloc::format!("{}={}", percent_encode(k), percent_encode(v)))
				.collect();
			url.push('?');
			url.push_str(&pairs.join("&"));
		}
		let mut builder = http::Request::builder().method(T::METADATA.method).uri(url);
		if let Some(token) = access_token.get_required_for_endpoint() {
			builder =
				builder.header(http::header::AUTHORIZATION, alloc::format!("Bearer {token}"));
		}
		let mut buf = B::default();
		if let Some(body) = self.body() {
			builder = builder.header(http::header::CONTENT_TYPE, "application/json");
			buf.put_slice(crate::codec::to_string(&body).as_bytes());
		}
		builder.body(buf).map_err(|e| IntoHttpError(e.to_string()))*/
	}
}

impl<T: EndpointResponse> IncomingResponse for T {
	type EndpointError = Error;

	fn try_from_http_response<B: AsRef<[u8]>>(
		response: http::Response<B>,
	) -> Result<Self, FromHttpResponseError<Error>> {
		if !response.status().is_success() {
			return Err(FromHttpResponseError::Server(Error::from_http_response(response)));
		}
		let text =
			core::str::from_utf8(response.body().as_ref()).map_err(|e| DeError(e.to_string()))?;
		let value = if T::REJECT_DUPLICATE_KEYS {
			Value::parse_strict(text)
		} else {
			Value::parse(text)
		}
		.map_err(|e| DeError(e.to_string()))?;
		Ok(T::from_body(&value)?)
	}
}

impl<T: EndpointRequest> IncomingRequest for T {
	type EndpointError = Error;
	type OutgoingResponse = T::Response;

	fn try_from_http_request<B: AsRef<[u8]>, S: AsRef<str>>(
		request: http::Request<B>,
		path_args: &[S],
	) -> Result<Self, FromHttpRequestError> {
		T::from_http_parts(&request, path_args)
	}
}

impl<T: EndpointResponse> OutgoingResponse for T {
	fn try_into_http_response<B: Default + BufMut>(
		self,
	) -> Result<http::Response<B>, IntoHttpError> {
		let mut buf = B::default();
		buf.put_slice(crate::codec::to_string(&self.to_body()).as_bytes());
		http::Response::builder()
			.status(http::StatusCode::OK)
			.header(http::header::CONTENT_TYPE, "application/json")
			.body(buf)
			.map_err(|e| IntoHttpError(e.to_string()))
	}
}

/// Reads an optional field of a JSON object.
///
/// # Errors
///
/// Returns an error if the field is present but has the wrong shape.
pub fn body_field<T: Deserialize>(body: Option<&Value>, name: &str) -> Result<T, DeError> {
	let null = Value::Null;
	let value = body.and_then(Value::as_object).and_then(|o| o.get(name)).unwrap_or(&null);
	T::from_json(value)
}

/// Builds a JSON object from named values, omitting nulls.
#[must_use]
pub fn object_from(mut fields: Vec<(&str, Value)>) -> Object {
	object_from_mut(&mut fields)
}

/// Builds a JSON object from named values, omitting nulls and moving the rest out.
#[doc(hidden)]
#[must_use]
pub fn object_from_mut(fields: &mut [(&str, Value)]) -> Object {
	fields
		.iter_mut()
		.filter(|(_, value)| !value.is_null())
		.map(|(name, value)| ((*name).to_string(), core::mem::replace(value, Value::Null)))
		.collect()
}

/// Flattens named values into query pairs: nulls are dropped and arrays repeat the key.
#[must_use]
pub fn query_pairs(mut params: Vec<(&str, Value)>) -> Vec<(String, String)> {
	query_pairs_mut(&mut params)
}

/// Flattens named values into query pairs, moving the values out.
#[doc(hidden)]
#[must_use]
pub fn query_pairs_mut(params: &mut [(&str, Value)]) -> Pairs {
	let mut pairs = Vec::new();
	for (name, value) in params {
		let items = match core::mem::replace(value, Value::Null) {
			Value::Array(items) => items,
			other => alloc::vec![other],
		};
		for item in items {
			let rendered = match item {
				Value::Null => continue,
				Value::String(text) => text,
				other => crate::json::write_string_value(&other).unwrap_or_default(),
			};
			pairs.push(((*name).to_string(), rendered));
		}
	}
	pairs
}

/// A request body of named fields; an empty `GET` body is omitted.
#[doc(hidden)]
#[must_use]
pub fn body_value(method: &str, fields: &mut [(&str, Value)]) -> Option<Value> {
	let object = object_from_mut(fields);
	if object.is_empty() && method == "GET" {
		None
	} else {
		Some(Value::Object(object))
	}
}

/// A response body of named fields.
#[doc(hidden)]
#[must_use]
pub fn body_object(fields: &mut [(&str, Value)]) -> Value {
	Value::Object(object_from_mut(fields))
}

/// A path argument rendered for the URL; null renders empty.
#[doc(hidden)]
#[must_use]
pub fn path_param<T: Serialize>(value: &T) -> String {
	to_param(value).unwrap_or_default()
}

/// Collects rendered path arguments, moving the strings out.
#[doc(hidden)]
#[must_use]
pub fn path_args_from(args: &mut [String]) -> Strs {
	args.iter_mut().map(core::mem::take).collect()
}

/// A field value for a generated encoder.
#[doc(hidden)]
#[must_use]
pub fn enc<T: Serialize + ?Sized>(value: &T) -> Value {
	value.to_json()
}

/// Opaque `Debug` for generated request and response types.
#[doc(hidden)]
pub fn opaque_debug(f: &mut Fmt<'_>, name: &str) -> FmtResult {
	f.write_str(name)
}

// Short names keep the generated impls small.
#[doc(hidden)]
pub type Str = String;
#[doc(hidden)]
pub type Strs = Vec<String>;
#[doc(hidden)]
pub type Pair = (String, String);
#[doc(hidden)]
pub type Pairs = Vec<(String, String)>;
#[doc(hidden)]
pub type Parsed<T> = Result<T, DeError>;
#[doc(hidden)]
pub type Fmt<'a> = fmt::Formatter<'a>;
#[doc(hidden)]
pub type FmtResult = fmt::Result;

/// Reads typed values out of a request or response being decoded.
pub struct Input<'a> {
	path: &'a [String],
	query: &'a [(String, String)],
	body: Option<&'a Value>,
	next: core::cell::Cell<usize>,
}

impl<'a> Input<'a> {
	#[must_use]
	pub fn new(
		path: &'a [String],
		query: &'a [(String, String)],
		body: Option<&'a Value>,
	) -> Self {
		Self {
			path,
			query,
			body,
			next: core::cell::Cell::new(0),
		}
	}

	/// An input with only a body, as for a response.
	#[doc(hidden)]
	#[must_use]
	pub fn body_only(body: &'a Value) -> Self {
		Self::new(&[], &[], Some(body))
	}

	/// The next path argument.
	///
	/// # Errors
	///
	/// Returns an error if the argument is missing or malformed.
	pub fn path<T: Deserialize>(&self) -> Result<T, DeError> {
		let index = self.next.get();
		self.next.set(index.saturating_add(1));
		from_param(self.path.get(index).map(String::as_str))
	}

	/// The named query parameter.
	///
	/// # Errors
	///
	/// Returns an error if the parameter is malformed or required and absent.
	pub fn query<T: Deserialize>(&self, name: &str) -> Result<T, DeError> {
		let values: Vec<&str> = self
			.query
			.iter()
			.filter(|(key, _)| key == name)
			.map(|(_, value)| value.as_str())
			.collect();
		from_param::<T>(values.first().copied()).or_else(|single| {
			let items = values.iter().map(|value| Value::String((*value).to_string())).collect();
			T::from_json(&Value::Array(items)).map_err(|_| single)
		})
	}

	/// The named query parameter, or `default` when it is not present.
	///
	/// # Errors
	///
	/// Returns an error if the parameter is present but malformed.
	pub fn query_or<T: Deserialize>(&self, name: &str, default: T) -> Result<T, DeError> {
		if self.query.iter().any(|(key, _)| key == name) {
			self.query(name)
		} else {
			Ok(default)
		}
	}

	/// The named field of the JSON body.
	///
	/// # Errors
	///
	/// Returns an error if the field is malformed or required and absent.
	pub fn body<T: Deserialize>(&self, name: &str) -> Result<T, DeError> {
		body_field(self.body, name)
	}

	/// The named field of the JSON body, or its default when absent or null.
	///
	/// # Errors
	///
	/// Returns an error if the field is present but malformed.
	pub fn body_or_default<T: Deserialize + Default>(&self, name: &str) -> Result<T, DeError> {
		self.body_or(name, T::default())
	}

	/// The named field of the JSON body, or the supplied default when absent or null.
	///
	/// # Errors
	///
	/// Returns an error if the field is present but malformed.
	pub fn body_or<T: Deserialize>(&self, name: &str, default: T) -> Result<T, DeError> {
		if let Some(body) = self.body
			&& !body.is_null()
			&& body.as_object().is_none()
		{
			return Err(DeError::expected("object"));
		}
		let present = self
			.body
			.and_then(Value::as_object)
			.and_then(|object| object.get(name))
			.is_some_and(|value| !value.is_null());
		if present {
			self.body(name)
		} else {
			Ok(default)
		}
	}

	/// Checks that every path argument was consumed.
	///
	/// # Errors
	///
	/// Returns an error if the request carries unexpected path arguments.
	pub fn finish(self) -> Result<(), DeError> {
		if self.next.get() >= self.path.len() {
			Ok(())
		} else {
			Err(DeError("unexpected path arguments".to_string()))
		}
	}
}

/// Declares an endpoint's request and response types.
///
/// ```text
/// endpoint! {
///     method: "GET", path: "/_matrix/federation/v1/event/{event_id}",
///     request {
///         path { event_id: OwnedEventId }
///         query { include_unredacted_content: Option<bool> }
///         body {}
///     }
///     response { origin: OwnedServerName }
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! endpoint_body_field {
	($input:ident, $name:expr, $ty:ty, default) => {
		$input.body_or_default($name)?
	};
	($input:ident, $name:expr, $ty:ty, $default:expr) => {
		$input.body_or($name, $default)?
	};
	($input:ident, $name:expr, $ty:ty) => {
		$input.body($name)?
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! endpoint_query_field {
	($input:ident, $name:expr, $ty:ty, $default:expr) => {
		$input.query_or($name, $default)?
	};
	($input:ident, $name:expr, $ty:ty) => {
		$input.query($name)?
	};
}

#[macro_export]
macro_rules! endpoint_request {
	// Body fields in this form support a Rust-name/wire-name override and a
	// default when the wire field is absent or null.
	(
		method: $method:literal, path: $path:literal,
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty),* $(,)? }
			body { $($body_field_name:ident => $body_wire_name:literal : $bt:ty = default),* $(,)? }
		}
	) => {
		pub struct Request {
			$(pub $path_field: $pt,)*
			$(pub $query_field: $qt,)*
			$(pub $body_field_name: $bt,)*
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Request")
			}
		}

		const _: $crate::endpoint::Metadata =
			<Request as $crate::endpoint::EndpointRequest>::METADATA;
		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata::new($method, $path);

			fn path_args(&self) -> $crate::endpoint::Strs {
				$crate::endpoint::path_args_from(&mut [
					$($crate::endpoint::path_param(&self.$path_field)),*
				])
			}

			fn query(&self) -> $crate::endpoint::Pairs {
				$crate::endpoint::query_pairs_mut(&mut [
					$((stringify!($query_field), $crate::endpoint::enc(&self.$query_field))),*
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				$crate::endpoint::body_value(
					<Self as $crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [$(($body_wire_name, $crate::endpoint::enc(&self.$body_field_name))),*],
				)
			}

			fn from_parts(
				path: &[$crate::endpoint::Str],
				query: &[$crate::endpoint::Pair],
				body: Option<&$crate::json::Value>,
			) -> $crate::endpoint::Parsed<Self> {
				let input = $crate::endpoint::Input::new(path, query, body);
				let value = Self {
					$($path_field: input.path()?,)*
					$($query_field: input.query(stringify!($query_field))?,)*
					$($body_field_name: input.body_or_default($body_wire_name)?,)*
				};
				input.finish()?;
				Ok(value)
			}
		}
	};
	(
		method: $method:literal, path: $path:literal,
		$(aliases: [$($alias:literal),* $(,)?],)?
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty $(= $query_default:expr)?),* $(,)? }
			body { $($body_field_name:ident : $bt:ty $(= $body_default:expr)?),* $(,)? }
		}
	) => {
		pub struct Request {
			$(pub $path_field: $pt,)*
			$(pub $query_field: $qt,)*
			$(pub $body_field_name: $bt,)*
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Request")
			}
		}

		const _: $crate::endpoint::Metadata =
				<Request as $crate::endpoint::EndpointRequest>::METADATA;
		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata::new($method, $path) $(.with_aliases(&[$($alias),*]))?;

			fn path_args(&self) -> $crate::endpoint::Strs {
				$crate::endpoint::path_args_from(&mut [
					$($crate::endpoint::path_param(&self.$path_field)),*
				])
			}

			fn query(&self) -> $crate::endpoint::Pairs {
				$crate::endpoint::query_pairs_mut(&mut [
					$((stringify!($query_field), $crate::endpoint::enc(&self.$query_field))),*
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				$crate::endpoint::body_value(
					<Self as $crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [$((stringify!($body_field_name), $crate::endpoint::enc(&self.$body_field_name))),*],
				)
			}

			fn from_parts(
				path: &[$crate::endpoint::Str],
				query: &[$crate::endpoint::Pair],
				body: Option<&$crate::json::Value>,
			) -> $crate::endpoint::Parsed<Self> {
				let input = $crate::endpoint::Input::new(path, query, body);
				let value = Self {
					$($path_field: input.path()?,)*
					$($query_field: $crate::endpoint_query_field!(input, stringify!($query_field), $qt $(, $query_default)?),)*
					$($body_field_name: $crate::endpoint_body_field!(input, stringify!($body_field_name), $bt $(, $body_default)?),)*
				};
				input.finish()?;
				Ok(value)
			}
		}
	};
}

/// Declares an endpoint's response type; see [`endpoint!`](macro@crate::endpoint).
#[macro_export]
macro_rules! endpoint_response {
	(response { $($resp_field:ident : $rt:ty),* $(,)? }) => {
		pub struct Response {
			$(pub $resp_field: $rt,)*
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Response")
			}
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::endpoint::body_object(&mut [
					$((stringify!($resp_field), $crate::endpoint::enc(&self.$resp_field))),*
				])
			}

			fn from_body(body: &$crate::json::Value) -> $crate::endpoint::Parsed<Self> {
				let _input = $crate::endpoint::Input::body_only(body);
				Ok(Self {
					$($resp_field: _input.body(stringify!($resp_field))?,)*
				})
			}
		}
	};
}

#[macro_export]
macro_rules! endpoint {
	(
		method: $method:literal, path: $path:literal,
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty),* $(,)? }
			body { $($body_field_name:ident : $bt:ty = default),* $(,)? }
		}
		response { $($resp_field:ident : $rt:ty),* $(,)? }
	) => {
		$crate::endpoint_request! {
			method: $method, path: $path,
			request {
				path { $($path_field : $pt),* }
				query { $($query_field : $qt),* }
				body { $($body_field_name : $bt = ::core::default::Default::default()),* }
			}
		}
		$crate::endpoint_response! { response { $($resp_field : $rt),* } }
	};
	(
		method: $method:literal, path: $path:literal,
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty),* $(,)? }
			body { $($body_field_name:ident => $body_wire_name:literal : $bt:ty = default),* $(,)? }
		}
		response { $($resp_field:ident : $rt:ty),* $(,)? }
	) => {
		$crate::endpoint_request! {
			method: $method, path: $path,
			request {
				path { $($path_field : $pt),* }
				query { $($query_field : $qt),* }
				body { $($body_field_name => $body_wire_name : $bt = default),* }
			}
		}
		$crate::endpoint_response! { response { $($resp_field : $rt),* } }
	};
	(
		method: $method:literal, path: $path:literal,
		$(aliases: [$($alias:literal),* $(,)?],)?
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty $(= $query_default:expr)?),* $(,)? }
			body { $($body_field_name:ident : $bt:ty $(= $body_default:expr)?),* $(,)? }
		}
		response { $($resp_field:ident : $rt:ty),* $(,)? }
	) => {
		$crate::endpoint_request! {
			method: $method, path: $path,
			$(aliases: [$($alias),*],)?
			request {
				path { $($path_field : $pt),* }
				query { $($query_field : $qt $(= $query_default)?),* }
				body { $($body_field_name : $bt $(= $body_default)?),* }
			}
		}
		$crate::endpoint_response! { response { $($resp_field : $rt),* } }
	};
}

/// Declares a request whose whole JSON body is a single value.
///
/// Used for endpoints such as `send_join`, where the body is the event itself.
#[macro_export]
macro_rules! endpoint_request_raw {
	(
		method: $method:literal, path: $path:literal,
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty),* $(,)? }
			raw_body { $body_field:ident : $bt:ty }
		}
	) => {
		pub struct Request {
			$(pub $path_field: $pt,)*
			$(pub $query_field: $qt,)*
			pub $body_field: $bt,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Request")
			}
		}

		const _: $crate::endpoint::Metadata =
				<Request as $crate::endpoint::EndpointRequest>::METADATA;
		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata::new($method, $path);

			fn path_args(&self) -> $crate::endpoint::Strs {
				$crate::endpoint::path_args_from(&mut [
					$($crate::endpoint::path_param(&self.$path_field)),*
				])
			}

			fn query(&self) -> $crate::endpoint::Pairs {
				$crate::endpoint::query_pairs_mut(&mut [
					$((stringify!($query_field), $crate::endpoint::enc(&self.$query_field))),*
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				Some($crate::endpoint::enc(&self.$body_field))
			}

			fn from_parts(
				path: &[$crate::endpoint::Str],
				query: &[$crate::endpoint::Pair],
				body: Option<&$crate::json::Value>,
			) -> $crate::endpoint::Parsed<Self> {
				let input = $crate::endpoint::Input::new(path, query, body);
				let value = Self {
					$($path_field: input.path()?,)*
					$($query_field: input.query(stringify!($query_field))?,)*
					$body_field: $crate::codec::Deserialize::from_json(
						body.ok_or_else(|| $crate::codec::DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;
				Ok(value)
			}
		}
	};
}

/// Declares a response whose whole JSON body is a single value.
#[macro_export]
macro_rules! endpoint_response_flat {
	($field:ident : $ty:ty) => {
		pub struct Response {
			pub $field: $ty,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Response")
			}
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::endpoint::enc(&self.$field)
			}

			fn from_body(body: &$crate::json::Value) -> $crate::endpoint::Parsed<Self> {
				Ok(Self {
					$field: $crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	};
}

/// Declares a response that is a JSON array `[status, value]`, as in the v1
/// `send_join` and `send_leave` endpoints.
#[macro_export]
macro_rules! endpoint_response_status_array {
	($field:ident : $ty:ty) => {
		pub struct Response {
			pub $field: $ty,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Response")
			}
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::json::Value::Array(::alloc::vec![
					$crate::codec::Serialize::to_json(&200_u64),
					$crate::endpoint::enc(&self.$field),
				])
			}

			fn from_body(body: &$crate::json::Value) -> $crate::endpoint::Parsed<Self> {
				let items =
					body.as_array().ok_or_else(|| $crate::codec::DeError::expected("array"))?;
				let status = items
					.first()
					.ok_or_else(|| $crate::codec::DeError::expected("[status, body]"))?;
				if status.as_u64() != Some(200) {
					return Err($crate::codec::DeError::expected("[200, body]"));
				}
				let value = items
					.get(1)
					.ok_or_else(|| $crate::codec::DeError::expected("[status, body]"))?;
				Ok(Self {
					$field: $crate::codec::Deserialize::from_json(value)?,
				})
			}
		}
	};
}

#[cfg(test)]
mod auth_tests {
	extern crate std;

	use std::{fs, path::Path, string::String, vec::Vec};

	use super::{AuthScheme, lookup_auth};

	/// Where the pinned ruwuma contract differs from the matrix spec: `(method, path,
	/// declared, reason)`. The declared value is the one in effect; the reason records
	/// which source won. Each was reviewed by hand (ruwuma, spec and `../synapse`).
	const SPEC_DIFFERENCES: &[(&str, &str, AuthScheme, &str)] = &[
		(
			"GET",
			"/_matrix/client/v1/room_summary/{}",
			AuthScheme::AccessTokenOptional,
			"ruwuma and synapse (`get_user_by_req(allow_guest=True)`, anonymous allowed) win; the spec generator mislabelled it",
		),
		(
			"GET",
			"/_matrix/federation/v1/query/{}",
			AuthScheme::ServerSignatures,
			"spec and synapse (`BaseFederationServerServlet`) win over ruwuma's AccessToken; not routed here",
		),
		(
			"PUT",
			"/_matrix/federation/v1/exchange_third_party_invite/{}",
			AuthScheme::ServerSignatures,
			"spec and synapse win over ruwuma's AccessToken; not routed here",
		),
		(
			"GET",
			"/_matrix/federation/v1/timestamp_to_event/{}",
			AuthScheme::ServerSignatures,
			"ruwuma and synapse (signed federation servlet) win; the spec generator picked up a client scheme",
		),
		(
			"POST",
			"/_matrix/client/v3/login",
			AuthScheme::AppserviceToken,
			"ruwuma wins: appservices may log in; `router/auth.rs` overrides every /login path to unauthenticated",
		),
		(
			"POST",
			"/_matrix/client/v1/appservice/{}/ping",
			AuthScheme::AccessToken,
			"ruwuma wins over the spec's appservice scheme: AccessToken also admits appservice tokens, so behaviour is unchanged",
		),
		(
			"PUT",
			"/_matrix/client/v3/directory/list/appservice/{}/{}",
			AuthScheme::AccessToken,
			"ruwuma wins for the same reason as appservice ping",
		),
	];

	/// Endpoints the spec gives no `security` block (`OpenAPI`: no authentication), each
	/// resolved by hand: `(method, path, declared, reason)`.
	const REVIEWED_NO_SECURITY: &[(&str, &str, AuthScheme, &str)] = &[
		("GET", "/.well-known/matrix/client", AuthScheme::None, "discovery, public"),
		("GET", "/.well-known/matrix/server", AuthScheme::None, "discovery, public"),
		("GET", "/.well-known/matrix/support", AuthScheme::None, "discovery, public"),
		("GET", "/_matrix/client/v1/auth_metadata", AuthScheme::None, "OAuth metadata, public"),
		(
			"GET",
			"/_matrix/client/v1/register/m.login.registration_token/validity",
			AuthScheme::None,
			"pre-registration, no account yet",
		),
		(
			"POST",
			"/_matrix/client/v3/account/3pid/email/requestToken",
			AuthScheme::None,
			"ruwuma None; synapse requires none for requestToken",
		),
		(
			"POST",
			"/_matrix/client/v3/account/password/email/requestToken",
			AuthScheme::None,
			"password reset, caller is logged out",
		),
		(
			"POST",
			"/_matrix/client/v3/register/email/requestToken",
			AuthScheme::None,
			"pre-registration",
		),
		("GET", "/_matrix/client/v3/register/available", AuthScheme::None, "pre-registration"),
		("GET", "/_matrix/client/v3/login", AuthScheme::None, "login flows, public"),
		("GET", "/_matrix/client/v3/login/sso/redirect", AuthScheme::None, "SSO start, public"),
		(
			"POST",
			"/_matrix/client/v3/refresh",
			AuthScheme::None,
			"authenticated by the refresh token in the body",
		),
		(
			"GET",
			"/_matrix/client/v3/profile/{}",
			AuthScheme::None,
			"ruwuma None; synapse gates it on `require_auth_for_profile_requests`, which Continuwuity handles in the handler",
		),
		("GET", "/_matrix/client/v3/profile/{}/{}", AuthScheme::None, "same as profile"),
		(
			"GET",
			"/_matrix/client/v3/publicRooms",
			AuthScheme::None,
			"ruwuma None; synapse gates on `allow_public_rooms_without_auth`, Continuwuity's handler checks its own config",
		),
		(
			"GET",
			"/_matrix/client/v3/directory/room/{}",
			AuthScheme::None,
			"alias lookup, public in ruwuma and synapse",
		),
		(
			"GET",
			"/_matrix/client/v3/directory/list/room/{}",
			AuthScheme::None,
			"room visibility, public",
		),
		(
			"GET",
			"/_matrix/media/v3/download/{}/{}",
			AuthScheme::None,
			"legacy unauthenticated media, as ruwuma",
		),
		(
			"GET",
			"/_matrix/media/v3/download/{}/{}/{}",
			AuthScheme::None,
			"legacy unauthenticated media, as ruwuma",
		),
		(
			"GET",
			"/_matrix/media/v3/thumbnail/{}/{}",
			AuthScheme::None,
			"legacy unauthenticated media, as ruwuma",
		),
		(
			"GET",
			"/_matrix/federation/v1/version",
			AuthScheme::None,
			"public in ruwuma and synapse",
		),
		(
			"GET",
			"/_matrix/federation/v1/openid/userinfo",
			AuthScheme::None,
			"authenticated by the OpenID token in the query",
		),
		(
			"PUT",
			"/_matrix/federation/v1/3pid/onbind",
			AuthScheme::None,
			"called by identity servers, not homeservers",
		),
		("GET", "/_matrix/key/v2/server", AuthScheme::None, "server keys are public"),
		("GET", "/_matrix/key/v2/query/{}", AuthScheme::None, "notary lookups are public"),
		("POST", "/_matrix/key/v2/query", AuthScheme::None, "notary lookups are public"),
	];

	fn rust_files(dir: &Path, out: &mut Vec<String>) {
		for entry in fs::read_dir(dir).unwrap() {
			let path = entry.unwrap().path();
			if path.is_dir() {
				rust_files(&path, out);
			} else if path.extension().is_some_and(|ext| ext == "rs") {
				out.push(fs::read_to_string(&path).unwrap());
			}
		}
	}

	/// Every endpoint path literal declared in `src/` has an authentication entry, so a
	/// new endpoint cannot ship without one.
	#[test]
	fn every_declared_endpoint_has_authentication() {
		let mut sources = Vec::new();
		rust_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut sources);
		let mut missing = Vec::new();
		for source in &sources {
			for line in source.lines() {
				let trimmed = line.trim_start();
				// Only endpoint declarations, not docs or prose.
				if trimmed.starts_with("//")
					|| !(line.contains("path: \"") || line.contains("Metadata::new("))
				{
					continue;
				}
				let Some(start) = line.find("\"/_matrix/") else {
					continue;
				};
				let rest = &line[start + 1..];
				let Some(end) = rest.find('"') else {
					continue;
				};
				let path = &rest[..end];
				let method = ["GET", "POST", "PUT", "DELETE"]
					.into_iter()
					.find(|method| line.contains(&std::format!("\"{method}\"")));
				if let Some(method) = method
					&& lookup_auth(method, path).is_none()
				{
					missing.push(std::format!("{method} {path}"));
				}
			}
		}
		assert!(missing.is_empty(), "endpoints with no declared authentication: {missing:#?}");
	}

	/// The declared scheme matches matrix-spec for every endpoint both describe, apart
	/// from the reviewed differences above.
	/// The recorded decisions are what is actually declared.
	#[test]
	fn recorded_decisions_match_declarations() {
		for &(method, path, expected, reason) in
			SPEC_DIFFERENCES.iter().chain(REVIEWED_NO_SECURITY)
		{
			assert!(!reason.is_empty());
			assert_eq!(lookup_auth(method, path), Some(expected), "{method} {path}");
		}
	}

	#[test]
	fn spec_cross_check() {
		let mut mismatches = Vec::new();
		for &(method, path, expected) in super::auth_spec::SPEC_AUTH {
			let Some(declared) = lookup_auth(method, path) else {
				continue;
			};
			let known = SPEC_DIFFERENCES.iter().any(|d| d.0 == method && d.1 == path);
			if declared != expected && !known {
				mismatches.push(std::format!(
					"{method} {path}: declared {declared:?}, spec {expected:?}"
				));
			}
		}
		assert!(mismatches.is_empty(), "authentication drifted from the spec: {mismatches:#?}");
	}

	/// Pins the security-relevant values reviewed by hand against ruwuma, the spec and
	/// synapse. A change here should be a deliberate decision.
	/// Anonymous access follows from the declared scheme: login, register and the
	/// optional-auth endpoints admit it; `get_token` and signed federation do not.
	#[test]
	fn anonymous_access_per_endpoint() {
		let cases = [
			("POST", "/_matrix/client/v3/login", true),
			("POST", "/_matrix/client/v3/register", true),
			("POST", "/_matrix/client/v1/login/get_token", false),
			("POST", "/_matrix/client/v3/logout", false),
			("POST", "/_matrix/client/v3/logout/all", false),
			("GET", "/_matrix/client/v1/room_summary/{room_id_or_alias}", true),
			("GET", "/_matrix/client/v3/publicRooms", true),
			("GET", "/_matrix/client/v3/profile/{user_id}", true),
			("PUT", "/_matrix/client/v3/profile/{user_id}/displayname", false),
			("PUT", "/_matrix/federation/v2/send_join/{room_id}/{event_id}", false),
		];
		for (method, path, anonymous) in cases {
			let scheme = lookup_auth(method, path).expect("declared");
			assert_eq!(scheme.allows_anonymous(), anonymous, "{method} {path}: {scheme:?}");
		}
	}

	#[test]
	fn reviewed_endpoint_authentication() {
		let cases = [
			("GET", "/_matrix/client/versions", AuthScheme::AccessTokenOptional),
			("GET", "/_matrix/client/v3/login", AuthScheme::None),
			// Appservices may log in; `router/auth.rs` still lets everyone reach /login.
			("POST", "/_matrix/client/v3/login", AuthScheme::AppserviceToken),
			("POST", "/_matrix/client/v3/register", AuthScheme::None),
			// Spec requires an access token; it must never be exempted like /login.
			("POST", "/_matrix/client/v1/login/get_token", AuthScheme::AccessToken),
			// Profile reads are unauthenticated in the spec; the router may tighten them by
			// config (`require_auth_for_profile_requests`).
			("GET", "/_matrix/client/v3/profile/{user_id}", AuthScheme::None),
			("GET", "/_matrix/client/v3/profile/{user_id}/displayname", AuthScheme::None),
			("GET", "/_matrix/client/v3/profile/{user_id}/avatar_url", AuthScheme::None),
			("PUT", "/_matrix/client/v3/profile/{user_id}/displayname", AuthScheme::AccessToken),
			("GET", "/_matrix/client/v3/publicRooms", AuthScheme::None),
			("POST", "/_matrix/client/v3/publicRooms", AuthScheme::AccessToken),
			("GET", "/_matrix/client/v3/account/whoami", AuthScheme::AccessToken),
			(
				"GET",
				"/_matrix/client/v1/room_summary/{room_id_or_alias}",
				AuthScheme::AccessTokenOptional,
			),
			(
				"PUT",
				"/_matrix/federation/v2/send_join/{room_id}/{event_id}",
				AuthScheme::ServerSignatures,
			),
		];
		for (method, path, expected) in cases {
			assert_eq!(lookup_auth(method, path), Some(expected), "{method} {path}");
		}
	}
}

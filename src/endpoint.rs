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

/// Static description of an endpoint.
#[derive(Clone, Copy, Debug)]
pub struct Metadata {
	pub method: &'static str,
	pub path: &'static str,
}

/// An error returned by the remote endpoint, parsed from an HTTP response.
pub trait EndpointError: Sized {
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
	type Response: EndpointResponse;
	const METADATA: Metadata;

	/// Values for each `{placeholder}` in the path, in order.
	fn path_args(&self) -> Vec<String>;
	fn query(&self) -> Vec<(String, String)>;
	fn body(&self) -> Option<Value>;
	/// # Errors
	///
	/// Returns an error if a required value is missing or malformed.
	fn from_parts(
		path: &[String],
		query: &[(String, String)],
		body: Option<&Value>,
	) -> Result<Self, DeError>;
}

/// Response half of an endpoint, implemented by [`endpoint!`](macro@crate::endpoint).
pub trait EndpointResponse: Sized {
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

impl<T: EndpointRequest> OutgoingRequest for T {
	const METADATA: Metadata = T::METADATA;
	type EndpointError = Error;
	type IncomingResponse = T::Response;

	fn try_into_http_request<B: Default + BufMut>(
		self,
		base_url: &str,
		access_token: SendAccessToken<'_>,
		_considering_versions: &[MatrixVersion],
	) -> Result<http::Request<B>, IntoHttpError> {
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
		builder.body(buf).map_err(|e| IntoHttpError(e.to_string()))
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
		let value = Value::parse(text).map_err(|e| DeError(e.to_string()))?;
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
		if request.method().as_str() != T::METADATA.method {
			return Err(FromHttpRequestError::MethodMismatch);
		}
		let path: Vec<String> = path_args.iter().map(|s| s.as_ref().to_string()).collect();
		let query: Vec<(String, String)> = request
			.uri()
			.query()
			.unwrap_or_default()
			.split('&')
			.filter(|pair| !pair.is_empty())
			.map(|pair| {
				let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
				(k.to_string(), v.to_string())
			})
			.collect();
		let bytes = request.body().as_ref();
		let body = if bytes.is_empty() {
			None
		} else {
			let text = core::str::from_utf8(bytes).map_err(|e| DeError(e.to_string()))?;
			Some(Value::parse(text).map_err(|e| DeError(e.to_string()))?)
		};
		Ok(T::from_parts(&path, &query, body.as_ref())?)
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
pub fn object_from(fields: Vec<(&str, Value)>) -> Object {
	fields
		.into_iter()
		.filter(|(_, value)| !value.is_null())
		.map(|(name, value)| (name.to_string(), value))
		.collect()
}

/// Flattens named values into query pairs: nulls are dropped and arrays repeat the key.
#[must_use]
pub fn query_pairs(params: Vec<(&str, Value)>) -> Vec<(String, String)> {
	let mut pairs = Vec::new();
	for (name, value) in params {
		let items = match value {
			Value::Array(items) => items,
			other => alloc::vec![other],
		};
		for item in items {
			let rendered = match item {
				Value::Null => continue,
				Value::String(text) => text,
				other => crate::json::write_string_value(&other).unwrap_or_default(),
			};
			pairs.push((name.to_string(), rendered));
		}
	}
	pairs
}

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
		let present = self
			.body
			.and_then(Value::as_object)
			.and_then(|object| object.get(name))
			.is_some_and(|value| !value.is_null());
		if present {
			self.body(name)
		} else {
			Ok(T::default())
		}
	}

	/// Checks that every path argument was consumed.
	///
	/// # Errors
	///
	/// Returns an error if the request carries unexpected path arguments.
	pub fn finish(self) -> Result<(), DeError> {
		if self.next.get() == self.path.len() {
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
#[macro_export]
macro_rules! endpoint_request {
	(
		method: $method:literal, path: $path:literal,
		request {
			path { $($path_field:ident : $pt:ty),* $(,)? }
			query { $($query_field:ident : $qt:ty),* $(,)? }
			body { $($body_field_name:ident : $bt:ty),* $(,)? }
		}
	) => {
		#[derive(Clone, Debug)]
		pub struct Request {
			$(pub $path_field: $pt,)*
			$(pub $query_field: $qt,)*
			$(pub $body_field_name: $bt,)*
		}

		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata { method: $method, path: $path };

			fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
				::alloc::vec![$($crate::endpoint::to_param(&self.$path_field).unwrap_or_default()),*]
			}

			fn query(&self) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)> {
				$crate::endpoint::query_pairs(::alloc::vec![
					$((stringify!($query_field), $crate::codec::Serialize::to_json(&self.$query_field))),*
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				let object = $crate::endpoint::object_from(::alloc::vec![
					$((stringify!($body_field_name), $crate::codec::Serialize::to_json(&self.$body_field_name))),*
				]);
				if object.is_empty() && <Self as $crate::endpoint::EndpointRequest>::METADATA.method == "GET" {
					None
				} else {
					Some($crate::json::Value::Object(object))
				}
			}

			fn from_parts(
				path: &[::alloc::string::String],
				query: &[(::alloc::string::String, ::alloc::string::String)],
				body: Option<&$crate::json::Value>,
			) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(path, query, body);
				let value = Self {
					$($path_field: input.path()?,)*
					$($query_field: input.query(stringify!($query_field))?,)*
					$($body_field_name: input.body(stringify!($body_field_name))?,)*
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
		#[derive(Clone, Debug)]
		pub struct Response {
			$(pub $resp_field: $rt,)*
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::json::Value::Object($crate::endpoint::object_from(::alloc::vec![
					$((stringify!($resp_field), $crate::codec::Serialize::to_json(&self.$resp_field))),*
				]))
			}

			fn from_body(body: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(&[], &[], Some(body));
				let value = Self {
					$($resp_field: input.body(stringify!($resp_field))?,)*
				};
				input.finish()?;
				Ok(value)
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
			body { $($body_field_name:ident : $bt:ty),* $(,)? }
		}
		response { $($resp_field:ident : $rt:ty),* $(,)? }
	) => {
		$crate::endpoint_request! {
			method: $method, path: $path,
			request {
				path { $($path_field : $pt),* }
				query { $($query_field : $qt),* }
				body { $($body_field_name : $bt),* }
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
		#[derive(Clone, Debug)]
		pub struct Request {
			$(pub $path_field: $pt,)*
			$(pub $query_field: $qt,)*
			pub $body_field: $bt,
		}

		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata { method: $method, path: $path };

			fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
				::alloc::vec![$($crate::endpoint::to_param(&self.$path_field).unwrap_or_default()),*]
			}

			fn query(&self) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)> {
				$crate::endpoint::query_pairs(::alloc::vec![
					$((stringify!($query_field), $crate::codec::Serialize::to_json(&self.$query_field))),*
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				Some($crate::codec::Serialize::to_json(&self.$body_field))
			}

			fn from_parts(
				path: &[::alloc::string::String],
				query: &[(::alloc::string::String, ::alloc::string::String)],
				body: Option<&$crate::json::Value>,
			) -> Result<Self, $crate::codec::DeError> {
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
		#[derive(Clone, Debug)]
		pub struct Response {
			pub $field: $ty,
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::codec::Serialize::to_json(&self.$field)
			}

			fn from_body(body: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
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
		#[derive(Clone, Debug)]
		pub struct Response {
			pub $field: $ty,
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::json::Value::Array(::alloc::vec![
					$crate::codec::Serialize::to_json(&200_u64),
					$crate::codec::Serialize::to_json(&self.$field),
				])
			}

			fn from_body(body: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
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
					$field: $crate::codec::Deserialize::from_json(value)?,
				})
			}
		}
	};
}

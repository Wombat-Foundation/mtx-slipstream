//! Minimal typed-endpoint machinery: requests, responses and HTTP conversion.
//!
//! An endpoint is a request type and a response type described once with
//! [`endpoint!`]. The macro implements [`EndpointRequest`] and
//! [`EndpointResponse`]; blanket impls then provide the ruma-style
//! `OutgoingRequest`, `IncomingResponse`, `IncomingRequest` and
//! `OutgoingResponse` traits in both directions.

use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use core::fmt;

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
	None,
}

impl<'a> SendAccessToken<'a> {
	/// The token to send, given whether the endpoint requires authentication.
	#[must_use]
	pub fn get_required_for_endpoint(self) -> Option<&'a str> {
		match self {
			| Self::IfRequired(token) | Self::Always(token) => Some(token),
			| Self::None => None,
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
		Self { status_code, body }
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
			| Self::Server(e) => e.fmt(f),
			| Self::Deserialization(e) => write!(f, "deserialization failed: {e}"),
		}
	}
}
impl<E: fmt::Debug + fmt::Display> core::error::Error for FromHttpResponseError<E> {}

impl<E> From<DeError> for FromHttpResponseError<E> {
	fn from(e: DeError) -> Self { Self::Deserialization(e) }
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
			| Self::Deserialization(e) => write!(f, "deserialization failed: {e}"),
			| Self::MethodMismatch => f.write_str("HTTP method mismatch"),
		}
	}
}
impl core::error::Error for FromHttpRequestError {}
impl From<DeError> for FromHttpRequestError {
	fn from(e: DeError) -> Self { Self::Deserialization(e) }
}

/// A request that can be sent to a remote server.
pub trait OutgoingRequest: Sized {
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

/// Request half of an endpoint, implemented by [`endpoint!`].
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

/// Response half of an endpoint, implemented by [`endpoint!`].
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
		| Value::Null => None,
		| Value::String(s) => Some(s),
		| other => crate::json::write_string_value(&other).ok(),
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
			out.push_str(&alloc::format!("%{byte:02X}"));
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
			builder = builder.header(http::header::AUTHORIZATION, alloc::format!("Bearer {token}"));
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
		let text = core::str::from_utf8(response.body().as_ref())
			.map_err(|e| DeError(e.to_string()))?;
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

/// Inserts a field into a JSON object under construction.
pub fn put_field<T: Serialize>(object: &mut Object, name: &str, value: &T) {
	let json = value.to_json();
	if !json.is_null() {
		object.insert(name.to_string(), json);
	}
}

/// Declares an endpoint's request and response types.
///
/// ```ignore
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
macro_rules! endpoint {
	(
		method: $method:literal, path: $path:literal,
		request {
			path { $($pn:ident : $pt:ty),* $(,)? }
			query { $($qn:ident : $qt:ty),* $(,)? }
			body { $($bn:ident : $bt:ty),* $(,)? }
		}
		response { $($rn:ident : $rt:ty),* $(,)? }
	) => {
		#[derive(Clone, Debug, Default)]
		pub struct Request {
			$(pub $pn: $pt,)*
			$(pub $qn: $qt,)*
			$(pub $bn: $bt,)*
		}

		#[derive(Clone, Debug, Default)]
		pub struct Response {
			$(pub $rn: $rt,)*
		}

		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata { method: $method, path: $path };

			fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
				#[allow(unused_mut)]
				let mut args = ::alloc::vec::Vec::new();
				$(args.push($crate::endpoint::to_param(&self.$pn).unwrap_or_default());)*
				args
			}

			fn query(&self) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)> {
				#[allow(unused_mut)]
				let mut query = ::alloc::vec::Vec::new();
				$(
					if let Some(value) = $crate::endpoint::to_param(&self.$qn) {
						query.push((::alloc::string::String::from(stringify!($qn)), value));
					}
				)*
				query
			}

			fn body(&self) -> Option<$crate::json::Value> {
				#[allow(unused_mut)]
				let mut object = $crate::json::Object::new();
				$($crate::endpoint::put_field(&mut object, stringify!($bn), &self.$bn);)*
				if object.is_empty() && $method == "GET" {
					None
				} else {
					Some($crate::json::Value::Object(object))
				}
			}

			#[allow(unused_variables, unused_mut, unused_assignments)]
			fn from_parts(
				path: &[::alloc::string::String],
				query: &[(::alloc::string::String, ::alloc::string::String)],
				body: Option<&$crate::json::Value>,
			) -> Result<Self, $crate::codec::DeError> {
				let mut index = 0_usize;
				$(
					let $pn: $pt = $crate::endpoint::from_param(
						path.get(index).map(::alloc::string::String::as_str),
					)?;
					index += 1;
				)*
				Ok(Self {
					$($pn,)*
					$($qn: $crate::endpoint::from_param(
						query
							.iter()
							.find(|(k, _)| k == stringify!($qn))
							.map(|(_, v)| v.as_str()),
					)?,)*
					$($bn: $crate::endpoint::body_field(body, stringify!($bn))?,)*
				})
			}
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				#[allow(unused_mut)]
				let mut object = $crate::json::Object::new();
				$($crate::endpoint::put_field(&mut object, stringify!($rn), &self.$rn);)*
				$crate::json::Value::Object(object)
			}

			#[allow(unused_variables)]
			fn from_body(body: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				Ok(Self {
					$($rn: $crate::endpoint::body_field(Some(body), stringify!($rn))?,)*
				})
			}
		}
	};
}

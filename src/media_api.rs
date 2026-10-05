//! Media endpoints: client download/upload, authenticated media, and the
//! federation download endpoints with their `multipart/mixed` responses.

use alloc::{string::String, vec::Vec};
use core::time::Duration;

use bytes::BufMut;

use crate::{
	api::{client::error::Error, error::IntoHttpError},
	endpoint::{EndpointError, FromHttpResponseError},
	http_headers::ContentDisposition,
	media::Method,
};

/// How long a download request waits for media that is not yet uploaded.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(20);

crate::impl_codec_enum!(Method { Crop => "crop", Scale => "scale" });

/// Declares a download-style request: path arguments, defaulted query
/// parameters and required query parameters, with no body.
macro_rules! media_request {
	(
		method: $method:literal, path: $path:literal, response: $resp:ty,
		request {
			path { $($pf:ident : $pt:ty),* $(,)? }
			query { $($qf:ident : $qt:ty = $qd:expr),* $(,)? }
			required { $($rf:ident : $rt:ty),* $(,)? }
		}
	) => {
		#[derive(Debug)]
		pub struct Request {
			$(pub $pf: $pt,)*
			$(pub $qf: $qt,)*
			$(pub $rf: $rt,)*
		}

		const _: $crate::endpoint::Metadata = $crate::endpoint::Metadata::new($method, $path);
		impl $crate::endpoint::EndpointRequest for Request {
			type Response = $resp;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata::new($method, $path);

			fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
				::alloc::vec![$($crate::endpoint::to_param(&self.$pf).unwrap_or_default()),*]
			}

			fn query(&self) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)> {
				$crate::endpoint::query_pairs(::alloc::vec![
					$((
						stringify!($qf),
						if self.$qf == $qd {
							$crate::json::Value::Null
						} else {
							$crate::codec::Serialize::to_json(&self.$qf)
						}
					),)*
					$((stringify!($rf), $crate::codec::Serialize::to_json(&self.$rf)),)*
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				None
			}

			fn from_parts(
				path: &[::alloc::string::String],
				query: &[(::alloc::string::String, ::alloc::string::String)],
				body: Option<&$crate::json::Value>,
			) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(path, query, body);
				let value = Self {
					$($pf: input.path()?,)*
					$($qf: input.query::<Option<$qt>>(stringify!($qf))?.unwrap_or($qd),)*
					$($rf: input.query(stringify!($rf))?,)*
				};
				input.finish()?;
				Ok(value)
			}
		}
	};
}

/// Declares a response whose body is the file and whose metadata travels in
/// headers.
macro_rules! file_response {
	() => {
		#[derive(Debug, Default)]
		pub struct Response {
			pub file: ::alloc::vec::Vec<u8>,
			pub content_type: Option<::alloc::string::String>,
			pub content_disposition: Option<$crate::http_headers::ContentDisposition>,
			pub cross_origin_resource_policy: Option<::alloc::string::String>,
			pub cache_control: Option<::alloc::string::String>,
		}

		impl Response {
			#[must_use]
			pub fn new(
				file: ::alloc::vec::Vec<u8>,
				content_type: Option<::alloc::string::String>,
			) -> Self {
				Self {
					file,
					content_type,
					..Self::default()
				}
			}
		}

		impl $crate::endpoint::IncomingResponse for Response {
			type EndpointError = $crate::api::client::error::Error;

			fn try_from_http_response<T: AsRef<[u8]>>(
				response: ::http::Response<T>,
			) -> Result<
				Self,
				$crate::endpoint::FromHttpResponseError<$crate::api::client::error::Error>,
			> {
				$crate::media_api::file_from_response(response).map(
					|(file, content_type, content_disposition, corp, cache)| Self {
						file,
						content_type,
						content_disposition,
						cross_origin_resource_policy: corp,
						cache_control: cache,
					},
				)
			}
		}

		impl $crate::endpoint::OutgoingResponse for Response {
			fn try_into_http_response<B: Default + ::bytes::BufMut>(
				self,
			) -> Result<::http::Response<B>, $crate::api::error::IntoHttpError> {
				$crate::media_api::file_to_response(
					&self.file,
					self.content_type.as_deref(),
					self.content_disposition.as_ref(),
					self.cross_origin_resource_policy.as_deref(),
					self.cache_control.as_deref(),
				)
			}
		}
	};
}

type FileParts =
	(Vec<u8>, Option<String>, Option<ContentDisposition>, Option<String>, Option<String>);

fn header_text(headers: &http::HeaderMap, name: http::header::HeaderName) -> Option<String> {
	headers.get(name).and_then(|value| value.to_str().ok()).map(String::from)
}

/// Splits a successful file response into body and metadata headers, or turns
/// an error response into an [`Error`].
///
/// # Errors
///
/// Returns the server's error for a non-success status.
#[doc(hidden)]
pub fn file_from_response<T: AsRef<[u8]>>(
	response: http::Response<T>,
) -> Result<FileParts, FromHttpResponseError<Error>> {
	if !response.status().is_success() {
		return Err(FromHttpResponseError::Server(Error::from_http_response(response)));
	}
	let headers = response.headers();
	let disposition = header_text(headers, http::header::CONTENT_DISPOSITION)
		.and_then(|text| text.parse::<ContentDisposition>().ok());
	Ok((
		response.body().as_ref().to_vec(),
		header_text(headers, http::header::CONTENT_TYPE),
		disposition,
		header_text(
			headers,
			http::header::HeaderName::from_static("cross-origin-resource-policy"),
		),
		header_text(headers, http::header::CACHE_CONTROL),
	))
}

/// Builds a `200` file response with the metadata headers set.
///
/// # Errors
///
/// Returns an error if a header value is not valid.
#[doc(hidden)]
pub fn file_to_response<B: Default + BufMut>(
	file: &[u8],
	content_type: Option<&str>,
	content_disposition: Option<&ContentDisposition>,
	cross_origin_resource_policy: Option<&str>,
	cache_control: Option<&str>,
) -> Result<http::Response<B>, IntoHttpError> {
	let mut builder = http::Response::builder().status(http::StatusCode::OK);
	if let Some(value) = content_type {
		builder = builder.header(http::header::CONTENT_TYPE, value);
	}
	if let Some(value) = content_disposition {
		builder = builder.header(http::header::CONTENT_DISPOSITION, value.to_string());
	}
	if let Some(value) = cross_origin_resource_policy {
		builder = builder.header("cross-origin-resource-policy", value);
	}
	if let Some(value) = cache_control {
		builder = builder.header(http::header::CACHE_CONTROL, value);
	}
	let mut body = B::default();
	body.put_slice(file);
	builder.body(body).map_err(|e| IntoHttpError(alloc::string::ToString::to_string(&e)))
}

/// Settings and limits of the media repository.
pub mod config {
	macro_rules! media_config {
		($path:literal) => {
			crate::endpoint! {
				method: "GET", path: $path,
				request { path {} query {} body {} }
				response { upload_size: crate::UInt }
			}
		};
	}
	pub(crate) use media_config;
}

macro_rules! preview_endpoint {
	($path:literal) => {
		media_request! {
			method: "GET", path: $path, response: Response,
			request {
				path {}
				query { ts: Option<$crate::MilliSecondsSinceUnixEpoch> = None }
				required { url: ::alloc::string::String }
			}
		}

		/// The preview is arbitrary JSON (Open Graph data), passed through.
		#[derive(Debug, Default)]
		pub struct Response {
			pub data: $crate::serde::Raw<$crate::json::Value>,
		}

		impl Response {
			/// Wraps preview JSON text.
			///
			/// # Errors
			///
			/// Returns an error if `json` is not valid JSON.
			pub fn from_json_text(json: &str) -> Result<Self, $crate::codec::DeError> {
				Ok(Self {
					data: $crate::serde::Raw::from_json_text(json)?,
				})
			}
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				$crate::codec::Serialize::to_json(&self.data)
			}

			fn from_body(body: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				Ok(Self {
					data: $crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	};
}

/// Authenticated client media (`/_matrix/client/v1/media/*`).
pub mod authenticated_client {
	pub mod get_media_config {
		pub mod v1 {
			crate::media_api::config::media_config!("/_matrix/client/v1/media/config");
		}
	}

	pub mod get_media_preview {
		pub mod v1 {
			preview_endpoint!("/_matrix/client/v1/media/preview_url");
		}
	}

	pub mod get_content {
		pub mod v1 {
			use core::time::Duration;

			use crate::{OwnedServerName, media_api::DEFAULT_TIMEOUT};

			file_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/client/v1/media/download/{server_name}/{media_id}",
				response: Response,
				request {
					path { server_name: OwnedServerName, media_id: alloc::string::String }
					query { timeout_ms: Duration = DEFAULT_TIMEOUT }
					required {}
				}
			}

			impl Request {
				#[must_use]
				pub fn new(
					media_id: alloc::string::String,
					server_name: OwnedServerName,
				) -> Self {
					Self {
						server_name,
						media_id,
						timeout_ms: DEFAULT_TIMEOUT,
					}
				}
			}
		}
	}

	pub mod get_content_as_filename {
		pub mod v1 {
			use core::time::Duration;

			use crate::{OwnedServerName, media_api::DEFAULT_TIMEOUT};

			file_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/client/v1/media/download/{server_name}/{media_id}/{filename}",
				response: Response,
				request {
					path {
						server_name: OwnedServerName,
						media_id: alloc::string::String,
						filename: alloc::string::String
					}
					query { timeout_ms: Duration = DEFAULT_TIMEOUT }
					required {}
				}
			}
		}
	}

	pub mod get_content_thumbnail {
		pub mod v1 {
			use core::time::Duration;

			use crate::{OwnedServerName, UInt, media::Method, media_api::DEFAULT_TIMEOUT};

			file_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/client/v1/media/thumbnail/{server_name}/{media_id}",
				response: Response,
				request {
					path { server_name: OwnedServerName, media_id: alloc::string::String }
					query {
						method: Option<Method> = None,
						timeout_ms: Duration = DEFAULT_TIMEOUT,
						animated: Option<bool> = None
					}
					required { width: UInt, height: UInt }
				}
			}
		}
	}
}

/// The legacy unauthenticated media API (`/_matrix/media/*`).
pub mod legacy {
	use alloc::string::String;

	use crate::endpoint::{Metadata, SendAccessToken};

	pub mod get_media_config {
		pub mod v3 {
			crate::media_api::config::media_config!("/_matrix/media/v3/config");
		}
	}

	pub mod get_media_preview {
		pub mod v3 {
			preview_endpoint!("/_matrix/media/v3/preview_url");
		}
	}

	pub mod get_content {
		pub mod v3 {
			use core::time::Duration;

			use crate::{OwnedServerName, media_api::DEFAULT_TIMEOUT};

			file_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/media/v3/download/{server_name}/{media_id}",
				response: Response,
				request {
					path { server_name: OwnedServerName, media_id: alloc::string::String }
					query {
						allow_remote: bool = true,
						timeout_ms: Duration = DEFAULT_TIMEOUT,
						allow_redirect: bool = false
					}
					required {}
				}
			}

			impl Request {
				#[must_use]
				pub fn new(
					media_id: alloc::string::String,
					server_name: OwnedServerName,
				) -> Self {
					Self {
						server_name,
						media_id,
						allow_remote: true,
						timeout_ms: DEFAULT_TIMEOUT,
						allow_redirect: false,
					}
				}
			}
		}
	}

	pub mod get_content_as_filename {
		pub mod v3 {
			use core::time::Duration;

			use crate::{OwnedServerName, media_api::DEFAULT_TIMEOUT};

			file_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/media/v3/download/{server_name}/{media_id}/{filename}",
				response: Response,
				request {
					path {
						server_name: OwnedServerName,
						media_id: alloc::string::String,
						filename: alloc::string::String
					}
					query {
						allow_remote: bool = true,
						timeout_ms: Duration = DEFAULT_TIMEOUT,
						allow_redirect: bool = false
					}
					required {}
				}
			}
		}
	}

	pub mod get_content_thumbnail {
		pub mod v3 {
			use core::time::Duration;

			use crate::{OwnedServerName, UInt, media::Method, media_api::DEFAULT_TIMEOUT};

			file_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/media/v3/thumbnail/{server_name}/{media_id}",
				response: Response,
				request {
					path { server_name: OwnedServerName, media_id: alloc::string::String }
					query {
						method: Option<Method> = None,
						allow_remote: bool = true,
						timeout_ms: Duration = DEFAULT_TIMEOUT,
						allow_redirect: bool = false,
						animated: Option<bool> = None
					}
					required { width: UInt, height: UInt }
				}
			}
		}
	}

	pub mod create_mxc_uri {
		pub mod v1 {
			use crate::{MilliSecondsSinceUnixEpoch, OwnedMxcUri};

			crate::endpoint! {
				method: "POST", path: "/_matrix/media/v1/create",
				request { path {} query {} body {} }
				response {
					content_uri: OwnedMxcUri,
					unused_expires_at: Option<MilliSecondsSinceUnixEpoch>
				}
			}

			impl Response {
				#[must_use]
				pub fn new(content_uri: OwnedMxcUri) -> Self {
					Self {
						content_uri,
						unused_expires_at: None,
					}
				}
			}
		}
	}

	/// Uploads a file: the request body is the raw bytes, with its type in the
	/// `Content-Type` header.
	pub mod create_content {
		pub mod v3 {
			use alloc::{string::String, vec::Vec};

			use super::super::{RawUpload, upload_request, upload_url_query};
			use crate::{
				OwnedMxcUri,
				api::error::IntoHttpError,
				codec::{DeError, Deserialize, Serialize},
				endpoint::{EndpointRequest, FromHttpRequestError, Metadata, SendAccessToken},
				json::Value,
			};

			#[derive(Debug, Default)]
			pub struct Request {
				pub filename: Option<String>,
				pub content_type: Option<String>,
				/// Ask the server for a blurhash (`xyz.amorgan.generate_blurhash`).
				pub generate_blurhash: bool,
				pub file: Vec<u8>,
			}

			impl Request {
				#[must_use]
				pub fn new(file: Vec<u8>) -> Self {
					Self {
						file,
						..Self::default()
					}
				}
			}

			#[derive(Debug)]
			pub struct Response {
				pub content_uri: OwnedMxcUri,
				pub blurhash: Option<String>,
			}

			impl Response {
				#[must_use]
				pub fn new(content_uri: OwnedMxcUri) -> Self {
					Self {
						content_uri,
						blurhash: None,
					}
				}
			}

			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> Value {
					Value::Object(crate::endpoint::object_from(alloc::vec![
						("content_uri", self.content_uri.to_json()),
						("xyz.amorgan.blurhash", self.blurhash.to_json()),
					]))
				}

				fn from_body(body: &Value) -> Result<Self, DeError> {
					let object = body.as_object().ok_or_else(|| DeError::expected("object"))?;
					Ok(Self {
						content_uri: object
							.get("content_uri")
							.ok_or_else(|| DeError::expected("content_uri"))
							.and_then(OwnedMxcUri::from_json)?,
						blurhash: object
							.get("xyz.amorgan.blurhash")
							.filter(|v| !v.is_null())
							.map(String::from_json)
							.transpose()?,
					})
				}
			}

			const _: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/media/v3/upload");
			impl EndpointRequest for Request {
				type Response = Response;
				const METADATA: Metadata = Metadata::new("POST", "/_matrix/media/v3/upload");

				fn path_args(&self) -> Vec<String> {
					Vec::new()
				}
				fn query(&self) -> Vec<(String, String)> {
					upload_url_query(self.filename.as_deref(), self.generate_blurhash)
				}
				fn body(&self) -> Option<Value> {
					None
				}
				fn try_into_http_request_raw<B: Default + bytes::BufMut>(
					self,
					base_url: &str,
					access_token: SendAccessToken<'_>,
					_versions: &[crate::endpoint::MatrixVersion],
				) -> Result<http::Request<B>, IntoHttpError> {
					let query =
						upload_url_query(self.filename.as_deref(), self.generate_blurhash);
					upload_request(
						<Request as EndpointRequest>::METADATA,
						base_url,
						&[],
						&query,
						access_token,
						RawUpload {
							content_type: self.content_type.as_deref(),
							file: &self.file,
						},
					)
				}
				fn from_parts(
					_path: &[String],
					_query: &[(String, String)],
					_body: Option<&Value>,
				) -> Result<Self, DeError> {
					Err(DeError::expected("raw upload body"))
				}

				fn from_http_parts<B: AsRef<[u8]>, S: AsRef<str>>(
					request: &http::Request<B>,
					_path_args: &[S],
				) -> Result<Self, FromHttpRequestError> {
					if request.method() != http::Method::POST {
						return Err(FromHttpRequestError::MethodMismatch);
					}
					let query = crate::endpoint::parse_query(request.uri());
					let get = |name: &str| {
						query.iter().find(|(key, _)| key == name).map(|(_, value)| value.clone())
					};
					Ok(Self {
						filename: get("filename"),
						content_type: request
							.headers()
							.get(http::header::CONTENT_TYPE)
							.and_then(|value| value.to_str().ok())
							.map(String::from),
						generate_blurhash: get("xyz.amorgan.generate_blurhash")
							.is_some_and(|value| value == "true"),
						file: request.body().as_ref().to_vec(),
					})
				}
			}
		}
	}

	/// Uploads into a previously reserved MXC URI.
	pub mod create_content_async {
		pub mod v3 {
			use alloc::{string::String, vec::Vec};

			use super::super::{RawUpload, upload_request, upload_url_query};
			use crate::{
				OwnedServerName,
				api::error::IntoHttpError,
				codec::DeError,
				endpoint::{
					EndpointRequest, EndpointResponse, FromHttpRequestError, Metadata,
					SendAccessToken,
				},
				json::{Object, Value},
			};

			#[derive(Debug)]
			pub struct Request {
				pub server_name: OwnedServerName,
				pub media_id: String,
				pub filename: Option<String>,
				pub content_type: Option<String>,
				pub file: Vec<u8>,
			}

			#[derive(Debug, Default)]
			pub struct Response {}

			impl EndpointResponse for Response {
				fn to_body(&self) -> Value {
					Value::Object(Object::new())
				}

				fn from_body(_body: &Value) -> Result<Self, DeError> {
					Ok(Self {})
				}
			}

			const _: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/media/v3/upload/{server_name}/{media_id}",
			);
			impl EndpointRequest for Request {
				type Response = Response;
				const METADATA: Metadata =
					Metadata::new("PUT", "/_matrix/media/v3/upload/{server_name}/{media_id}");

				fn path_args(&self) -> Vec<String> {
					vec![self.server_name.as_str().into(), self.media_id.clone()]
				}
				fn query(&self) -> Vec<(String, String)> {
					upload_url_query(self.filename.as_deref(), false)
				}
				fn body(&self) -> Option<Value> {
					None
				}
				fn try_into_http_request_raw<B: Default + bytes::BufMut>(
					self,
					base_url: &str,
					access_token: SendAccessToken<'_>,
					_versions: &[crate::endpoint::MatrixVersion],
				) -> Result<http::Request<B>, IntoHttpError> {
					let query = upload_url_query(self.filename.as_deref(), false);
					upload_request(
						<Request as EndpointRequest>::METADATA,
						base_url,
						&[self.server_name.as_str().into(), self.media_id.clone()],
						&query,
						access_token,
						RawUpload {
							content_type: self.content_type.as_deref(),
							file: &self.file,
						},
					)
				}
				fn from_parts(
					_path: &[String],
					_query: &[(String, String)],
					_body: Option<&Value>,
				) -> Result<Self, DeError> {
					Err(DeError::expected("raw upload body"))
				}

				fn from_http_parts<B: AsRef<[u8]>, S: AsRef<str>>(
					request: &http::Request<B>,
					path_args: &[S],
				) -> Result<Self, FromHttpRequestError> {
					if request.method() != http::Method::PUT {
						return Err(FromHttpRequestError::MethodMismatch);
					}
					let [server_name, media_id] = path_args else {
						return Err(FromHttpRequestError::Deserialization(DeError::expected(
							"server name and media ID",
						)));
					};
					let query = crate::endpoint::parse_query(request.uri());
					Ok(Self {
						server_name: OwnedServerName::from(server_name.as_ref()),
						media_id: media_id.as_ref().into(),
						filename: query
							.iter()
							.find(|(key, _)| key == "filename")
							.map(|(_, value)| value.clone()),
						content_type: request
							.headers()
							.get(http::header::CONTENT_TYPE)
							.and_then(|value| value.to_str().ok())
							.map(String::from),
						file: request.body().as_ref().to_vec(),
					})
				}
			}
		}
	}

	#[derive(Clone, Copy)]
	pub(crate) struct RawUpload<'a> {
		pub content_type: Option<&'a str>,
		pub file: &'a [u8],
	}

	pub(crate) fn upload_url_query(
		filename: Option<&str>,
		generate_blurhash: bool,
	) -> alloc::vec::Vec<(String, String)> {
		let mut query = alloc::vec::Vec::new();
		if let Some(filename) = filename {
			query.push((String::from("filename"), String::from(filename)));
		}
		if generate_blurhash {
			query.push((String::from("xyz.amorgan.generate_blurhash"), String::from("true")));
		}
		query
	}

	pub(crate) fn upload_request<B: Default + bytes::BufMut>(
		metadata: Metadata,
		base_url: &str,
		args: &[String],
		query: &[(String, String)],
		access_token: SendAccessToken<'_>,
		upload: RawUpload<'_>,
	) -> Result<http::Request<B>, crate::api::error::IntoHttpError> {
		let url = crate::endpoint::request_url(base_url, metadata.path, args, query);
		let mut builder = http::Request::builder().method(metadata.method).uri(url);
		if let Some(token) = access_token.get_required_for_endpoint() {
			builder =
				builder.header(http::header::AUTHORIZATION, alloc::format!("Bearer {token}"));
		}
		if let Some(content_type) = upload.content_type {
			builder = builder.header(http::header::CONTENT_TYPE, content_type);
		}
		let mut body = B::default();
		body.put_slice(upload.file);
		builder
			.body(body)
			.map_err(|e| crate::api::error::IntoHttpError(alloc::string::ToString::to_string(&e)))
	}
}

/// Authenticated federation media (`/_matrix/federation/v1/media/*`).
///
/// Responses are `multipart/mixed`: a JSON metadata part, then either the
/// file or a redirect to where it can be fetched.
pub mod federation {
	use alloc::{string::String, vec::Vec};

	use bytes::BufMut;

	use crate::{
		api::{client::error::Error, error::IntoHttpError},
		endpoint::{EndpointError, FromHttpResponseError},
		http_headers::ContentDisposition,
	};

	/// A file with the headers that describe it.
	#[derive(Debug, Default, Eq, PartialEq)]
	pub struct Content {
		pub file: Vec<u8>,
		pub content_type: Option<String>,
		pub content_disposition: Option<ContentDisposition>,
	}

	/// The second part of a federation media response.
	#[derive(Debug, Eq, PartialEq)]
	pub enum FileOrLocation {
		File(Content),
		/// A URL the file can be downloaded from instead.
		Location(String),
	}

	/// The first part of a federation media response; currently empty.
	#[derive(Debug, Default, Eq, PartialEq)]
	pub struct ContentMetadata {}

	impl ContentMetadata {
		#[must_use]
		pub fn new() -> Self {
			Self {}
		}
	}

	struct Part<'a> {
		headers: Vec<(String, String)>,
		body: &'a [u8],
	}

	fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
		haystack
			.get(from..)?
			.windows(needle.len())
			.position(|window| window == needle)
			.map(|offset| offset.saturating_add(from))
	}

	fn parse_part(raw: &[u8]) -> Option<Part<'_>> {
		let end = find(raw, b"\r\n\r\n", 0);
		let (head, body) = match end {
			Some(end) => (raw.get(..end)?, raw.get(end.saturating_add(4)..)?),
			None => (raw.strip_suffix(b"\r\n").unwrap_or(raw), &[][..]),
		};
		let headers = core::str::from_utf8(head)
			.ok()?
			.split("\r\n")
			.filter_map(|line| line.split_once(':'))
			.map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().into()))
			.collect();
		Some(Part {
			headers,
			body,
		})
	}

	fn parse_multipart<'a>(body: &'a [u8], boundary: &str) -> Option<Vec<Part<'a>>> {
		let delimiter = alloc::format!("--{boundary}");
		let separator = alloc::format!("\r\n--{boundary}");
		let mut parts = Vec::new();
		let mut at = find(body, delimiter.as_bytes(), 0)?;
		loop {
			let after = at.saturating_add(delimiter.len());
			if body.get(after..after.saturating_add(2)) == Some(b"--") {
				return Some(parts);
			}
			let start = body
				.get(after..)?
				.strip_prefix(b"\r\n")
				.map(|rest| body.len().saturating_sub(rest.len()))?;
			let next = find(body, separator.as_bytes(), start)?;
			parts.push(parse_part(body.get(start..next)?)?);
			at = next.saturating_add(2);
		}
	}

	fn boundary_of(content_type: &str) -> Option<String> {
		let (_, rest) = content_type.split_once("boundary=")?;
		Some(rest.split(';').next()?.trim().trim_matches('"').into())
	}

	fn header<'a>(part: &'a Part<'_>, name: &str) -> Option<&'a str> {
		part.headers.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
	}

	fn decode(
		response: http::Response<impl AsRef<[u8]>>,
	) -> Result<(FileOrLocation, ContentMetadata), FromHttpResponseError<Error>> {
		if !response.status().is_success() {
			return Err(FromHttpResponseError::Server(Error::from_http_response(response)));
		}
		let malformed = |what: &str| {
			FromHttpResponseError::Deserialization(crate::codec::DeError(alloc::format!(
				"malformed multipart media response: {what}"
			)))
		};
		let boundary = response
			.headers()
			.get(http::header::CONTENT_TYPE)
			.and_then(|value| value.to_str().ok())
			.and_then(boundary_of)
			.ok_or_else(|| malformed("missing boundary"))?;
		let parts = parse_multipart(response.body().as_ref(), &boundary)
			.ok_or_else(|| malformed("bad framing"))?;
		let [_metadata, media] = parts.as_slice() else {
			return Err(malformed("expected two parts"));
		};
		let content = if let Some(location) = header(media, "location") {
			FileOrLocation::Location(location.into())
		} else {
			FileOrLocation::File(Content {
				file: media.body.to_vec(),
				content_type: header(media, "content-type").map(String::from),
				content_disposition: header(media, "content-disposition")
					.and_then(|text| text.parse().ok()),
			})
		};
		Ok((content, ContentMetadata::new()))
	}

	fn encode<B: Default + BufMut>(
		content: &FileOrLocation,
	) -> Result<http::Response<B>, IntoHttpError> {
		let file = match content {
			FileOrLocation::File(content) => content.file.as_slice(),
			FileOrLocation::Location(_) => &[],
		};
		let mut counter = 0_u64;
		let boundary = loop {
			let candidate = alloc::format!("slipstream-boundary-{counter}");
			if find(file, candidate.as_bytes(), 0).is_none() {
				break candidate;
			}
			counter = counter.saturating_add(1);
		};
		let mut out = Vec::new();
		out.extend_from_slice(
			alloc::format!(
				"--{boundary}\r\nContent-Type: application/json\r\n\r\n{{}}\r\n--{boundary}\r\n"
			)
			.as_bytes(),
		);
		match content {
			FileOrLocation::File(content) => {
				let content_type =
					content.content_type.as_deref().unwrap_or("application/octet-stream");
				out.extend_from_slice(
					alloc::format!("Content-Type: {content_type}\r\n").as_bytes(),
				);
				if let Some(disposition) = &content.content_disposition {
					out.extend_from_slice(
						alloc::format!("Content-Disposition: {disposition}\r\n").as_bytes(),
					);
				}
				out.extend_from_slice(b"\r\n");
				out.extend_from_slice(&content.file);
			}
			FileOrLocation::Location(url) => {
				out.extend_from_slice(alloc::format!("Location: {url}\r\n\r\n").as_bytes());
			}
		}
		out.extend_from_slice(alloc::format!("\r\n--{boundary}--\r\n").as_bytes());
		let mut body = B::default();
		body.put_slice(&out);
		http::Response::builder()
			.status(http::StatusCode::OK)
			.header(
				http::header::CONTENT_TYPE,
				alloc::format!("multipart/mixed; boundary={boundary}"),
			)
			.body(body)
			.map_err(|e| IntoHttpError(alloc::string::ToString::to_string(&e)))
	}

	macro_rules! multipart_response {
		() => {
			#[derive(Debug)]
			pub struct Response {
				pub content: $crate::media_api::federation::FileOrLocation,
				pub metadata: $crate::media_api::federation::ContentMetadata,
			}

			impl $crate::endpoint::IncomingResponse for Response {
				type EndpointError = $crate::api::client::error::Error;

				fn try_from_http_response<T: AsRef<[u8]>>(
					response: ::http::Response<T>,
				) -> Result<
					Self,
					$crate::endpoint::FromHttpResponseError<$crate::api::client::error::Error>,
				> {
					$crate::media_api::federation::decode_response(response).map(
						|(content, metadata)| Self {
							content,
							metadata,
						},
					)
				}
			}

			impl $crate::endpoint::OutgoingResponse for Response {
				fn try_into_http_response<B: Default + ::bytes::BufMut>(
					self,
				) -> Result<::http::Response<B>, $crate::api::error::IntoHttpError> {
					$crate::media_api::federation::encode_response(&self.content)
				}
			}
		};
	}

	#[doc(hidden)]
	pub fn decode_response<T: AsRef<[u8]>>(
		response: http::Response<T>,
	) -> Result<(FileOrLocation, ContentMetadata), FromHttpResponseError<Error>> {
		decode(response)
	}

	#[doc(hidden)]
	pub fn encode_response<B: Default + BufMut>(
		content: &FileOrLocation,
	) -> Result<http::Response<B>, IntoHttpError> {
		encode(content)
	}

	pub mod get_content {
		pub mod v1 {
			use core::time::Duration;

			use crate::media_api::DEFAULT_TIMEOUT;

			multipart_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/federation/v1/media/download/{media_id}",
				response: Response,
				request {
					path { media_id: alloc::string::String }
					query { timeout_ms: Duration = DEFAULT_TIMEOUT }
					required {}
				}
			}
		}
	}

	pub mod get_content_thumbnail {
		pub mod v1 {
			use core::time::Duration;

			use crate::{UInt, media::Method, media_api::DEFAULT_TIMEOUT};

			multipart_response!();

			media_request! {
				method: "GET",
				path: "/_matrix/federation/v1/media/thumbnail/{media_id}",
				response: Response,
				request {
					path { media_id: alloc::string::String }
					query {
						method: Option<Method> = None,
						timeout_ms: Duration = DEFAULT_TIMEOUT,
						animated: Option<bool> = None
					}
					required { width: UInt, height: UInt }
				}
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn multipart_round_trips_file_and_location() {
			let file = FileOrLocation::File(Content {
				file: b"bin\r\n--slipstream-boundary-0\r\nary".to_vec(),
				content_type: Some("image/png".into()),
				content_disposition: None,
			});
			let response: http::Response<Vec<u8>> = encode(&file).unwrap();
			let (decoded, _) = decode(response).unwrap();
			assert_eq!(decoded, file);

			let location = FileOrLocation::Location("https://cdn.example/x".into());
			let response: http::Response<Vec<u8>> = encode(&location).unwrap();
			assert_eq!(decode(response).unwrap().0, location);
		}

		#[test]
		fn errors_surface_as_server_errors() {
			let response = http::Response::builder()
				.status(404)
				.body(br#"{"errcode":"M_NOT_FOUND","error":"x"}"#.to_vec())
				.unwrap();
			assert!(matches!(decode(response), Err(FromHttpResponseError::Server(_))));
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::vec::Vec;

	use super::*;
	use crate::{
		OwnedServerName,
		api::{
			IncomingRequest, IncomingResponse, OutgoingRequest, OutgoingResponse, SendAccessToken,
		},
		http_headers::ContentDispositionType,
	};

	#[test]
	fn download_request_round_trips_with_defaults() {
		let request = legacy::get_content::v3::Request::new(
			"abc".into(),
			OwnedServerName::from("example.org"),
		);
		let http: http::Request<Vec<u8>> = request
			.try_into_http_request(
				"https://hs.example",
				SendAccessToken::None,
				&[crate::endpoint::MatrixVersion::V1_11],
			)
			.unwrap();
		assert_eq!(
			http.uri().to_string(),
			"https://hs.example/_matrix/media/v3/download/example.org/abc"
		);
		let parsed = legacy::get_content::v3::Request::try_from_http_request(
			http,
			&["example.org", "abc"],
		)
		.unwrap();
		assert!(parsed.allow_remote && !parsed.allow_redirect);
		assert_eq!(parsed.timeout_ms, DEFAULT_TIMEOUT);

		let mut custom = parsed;
		custom.allow_remote = false;
		custom.timeout_ms = Duration::from_secs(5);
		let http: http::Request<Vec<u8>> = custom
			.try_into_http_request(
				"https://hs.example",
				SendAccessToken::None,
				&[crate::endpoint::MatrixVersion::V1_11],
			)
			.unwrap();
		assert!(http.uri().to_string().ends_with("?allow_remote=false&timeout_ms=5000"));
	}

	#[test]
	fn file_response_carries_headers_and_bytes() {
		let mut response =
			legacy::get_content::v3::Response::new(b"data".to_vec(), Some("text/plain".into()));
		response.content_disposition = Some(
			ContentDisposition::new(ContentDispositionType::Attachment)
				.with_filename(Some("a.txt".into())),
		);
		response.cache_control = Some("public, immutable".into());
		let http: http::Response<Vec<u8>> = response.try_into_http_response().unwrap();
		assert_eq!(http.headers()["content-type"], "text/plain");
		let parsed = legacy::get_content::v3::Response::try_from_http_response(http).unwrap();
		assert_eq!(parsed.file, b"data");
		assert_eq!(parsed.content_disposition.unwrap().filename.as_deref(), Some("a.txt"));
		assert_eq!(parsed.cache_control.as_deref(), Some("public, immutable"));
	}

	#[test]
	fn upload_request_keeps_raw_bytes_and_headers() {
		let mut request = legacy::create_content::v3::Request::new(alloc::vec![0, 255, 7]);
		request.filename = Some("my file.bin".into());
		request.content_type = Some("application/octet-stream".into());
		request.generate_blurhash = true;
		let http: http::Request<Vec<u8>> = request
			.try_into_http_request(
				"https://hs.example",
				SendAccessToken::Always("tok"),
				&[crate::endpoint::MatrixVersion::V1_11],
			)
			.unwrap();
		let parsed =
			legacy::create_content::v3::Request::try_from_http_request(http, &[] as &[&str])
				.unwrap();
		assert_eq!(parsed.file, [0, 255, 7]);
		assert_eq!(parsed.filename.as_deref(), Some("my file.bin"));
		assert_eq!(parsed.content_type.as_deref(), Some("application/octet-stream"));
		assert!(parsed.generate_blurhash);
	}

	#[test]
	fn thumbnail_requires_dimensions() {
		let query = [(alloc::string::String::from("width"), alloc::string::String::from("32"))];
		assert!(
			crate::endpoint::EndpointRequest::from_parts(
				&["example.org".into(), "abc".into()],
				&query,
				None,
			)
			.map(|r: legacy::get_content_thumbnail::v3::Request| r)
			.is_err()
		);
	}
}

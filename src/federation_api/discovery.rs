//! Server discovery and signing-key endpoints.

use alloc::collections::BTreeMap;

use crate::{
	MilliSecondsSinceUnixEpoch, OwnedServerName, OwnedServerSigningKeyId, Signatures,
	sswire::Base64,
};

pub mod discover_homeserver {
	use crate::OwnedServerName;
	pub struct Request {}
	impl ::core::fmt::Debug for Request {
		fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
			crate::endpoint::opaque_debug(f, "Request")
		}
	}
	const _: crate::endpoint::Metadata = <Request as crate::endpoint::EndpointRequest>::METADATA;
	impl crate::endpoint::EndpointRequest for Request {
		type Response = Response;
		const METADATA: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("GET", "/.well-known/matrix/server");
		fn path_args(&self) -> crate::endpoint::Strs {
			crate::endpoint::path_args_from(&mut [])
		}
		fn query(&self) -> crate::endpoint::Pairs {
			crate::endpoint::query_pairs_mut(&mut [])
		}
		fn body(&self) -> Option<crate::json::Value> {
			crate::endpoint::body_value(
				<Self as crate::endpoint::EndpointRequest>::METADATA.method,
				&mut [],
			)
		}
		fn from_parts(
			path: &[crate::endpoint::Str],
			query: &[crate::endpoint::Pair],
			body: Option<&crate::json::Value>,
		) -> crate::endpoint::Parsed<Self> {
			let input = crate::endpoint::Input::new(path, query, body);
			let value = Self {};
			input.finish()?;
			Ok(value)
		}
	}
	pub struct Response {
		pub server: OwnedServerName,
	}
	impl ::core::fmt::Debug for Response {
		fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
			crate::endpoint::opaque_debug(f, "Response")
		}
	}
	impl crate::endpoint::EndpointResponse for Response {
		fn to_body(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [("server", crate::endpoint::enc(&self.server))])
		}
		fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			let input = crate::endpoint::Input::body_only(body);
			Ok(Self {
				server: input.body("server")?,
			})
		}
	}
}

/// A server's current public signing key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyKey {
	pub key: Base64,
}

impl VerifyKey {
	#[must_use]
	pub fn new(key: Base64) -> Self {
		Self {
			key,
		}
	}
}

impl crate::codec::Serialize for VerifyKey {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(stringify!(key), crate::endpoint::enc(&self.key))])
	}
}
impl crate::codec::Deserialize for VerifyKey {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(VerifyKey)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			key: input.body(stringify!(key))?,
		})
	}
}

/// A server's expired public signing key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OldVerifyKey {
	pub expired_ts: MilliSecondsSinceUnixEpoch,
	pub key: Base64,
}

impl OldVerifyKey {
	#[must_use]
	pub fn new(expired_ts: MilliSecondsSinceUnixEpoch, key: Base64) -> Self {
		Self {
			expired_ts,
			key,
		}
	}
}

impl crate::codec::Serialize for OldVerifyKey {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(expired_ts), crate::endpoint::enc(&self.expired_ts)),
			(stringify!(key), crate::endpoint::enc(&self.key)),
		])
	}
}
impl crate::codec::Deserialize for OldVerifyKey {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(OldVerifyKey)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			expired_ts: input.body(stringify!(expired_ts))?,
			key: input.body(stringify!(key))?,
		})
	}
}

/// A server's published signing keys.
#[derive(Debug, Eq, PartialEq)]
pub struct ServerSigningKeys {
	pub server_name: OwnedServerName,
	pub verify_keys: BTreeMap<OwnedServerSigningKeyId, VerifyKey>,
	pub old_verify_keys: BTreeMap<OwnedServerSigningKeyId, OldVerifyKey>,
	pub signatures: Signatures,
	pub valid_until_ts: MilliSecondsSinceUnixEpoch,
}

impl ServerSigningKeys {
	#[must_use]
	pub fn new(server_name: OwnedServerName, valid_until_ts: MilliSecondsSinceUnixEpoch) -> Self {
		Self {
			server_name,
			verify_keys: BTreeMap::new(),
			old_verify_keys: BTreeMap::new(),
			signatures: Signatures::new(),
			valid_until_ts,
		}
	}
}

impl crate::codec::Serialize for ServerSigningKeys {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(server_name), crate::endpoint::enc(&self.server_name)),
			(stringify!(verify_keys), crate::endpoint::enc(&self.verify_keys)),
			(stringify!(signatures), crate::endpoint::enc(&self.signatures)),
			(stringify!(valid_until_ts), crate::endpoint::enc(&self.valid_until_ts)),
			(stringify!(old_verify_keys), crate::endpoint::enc(&self.old_verify_keys)),
		])
	}
}
impl crate::codec::Deserialize for ServerSigningKeys {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(ServerSigningKeys)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			server_name: input.body(stringify!(server_name))?,
			verify_keys: input.body(stringify!(verify_keys))?,
			signatures: input.body(stringify!(signatures))?,
			valid_until_ts: input.body(stringify!(valid_until_ts))?,
			old_verify_keys: input.body_or_default(stringify!(old_verify_keys))?,
		})
	}
}

pub mod get_server_version {
	pub mod v1 {
		use alloc::string::String;

		/// Server software name and version.
		#[derive(Debug, Default)]
		pub struct Server {
			pub name: Option<String>,
			pub version: Option<String>,
		}

		impl crate::codec::Serialize for Server {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(name), crate::endpoint::enc(&self.name)),
					(stringify!(version), crate::endpoint::enc(&self.version)),
				])
			}
		}
		impl crate::codec::Deserialize for Server {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(Server)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					name: input.body(stringify!(name))?,
					version: input.body(stringify!(version))?,
				})
			}
		}

		pub struct Request {}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/federation/v1/version");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub server: Option<Server>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"server",
					crate::endpoint::enc(&self.server),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					server: input.body("server")?,
				})
			}
		}
	}
}

pub mod get_server_keys {
	pub mod v2 {
		use super::super::ServerSigningKeys;
		use crate::{
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointRequest, EndpointResponse, Metadata},
			json::Value,
			sswire::Raw,
		};

		/// Request for this server's own signing keys.
		#[derive(Debug, Default)]
		pub struct Request;

		/// The keys document is the whole response body.
		#[derive(Debug)]
		pub struct Response {
			pub server_key: Raw<ServerSigningKeys>,
		}

		impl Response {
			#[must_use]
			pub fn new(server_key: Raw<ServerSigningKeys>) -> Self {
				Self {
					server_key,
				}
			}
		}

		const _: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("GET", "/_matrix/key/v2/server");
		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata::new("GET", "/_matrix/key/v2/server");

			fn path_args(&self) -> alloc::vec::Vec<String> {
				alloc::vec::Vec::new()
			}

			fn query(&self) -> alloc::vec::Vec<(String, String)> {
				alloc::vec::Vec::new()
			}

			fn body(&self) -> Option<Value> {
				None
			}

			fn from_parts(
				_path: &[String],
				_query: &[(String, String)],
				_body: Option<&Value>,
			) -> Result<Self, DeError> {
				Ok(Self)
			}
		}

		impl EndpointResponse for Response {
			// MSC4499: a key response with a repeated key must be rejected, and
			// the body is parsed (and deduplicated) before `from_body` runs.
			const REJECT_DUPLICATE_KEYS: bool = true;

			fn to_body(&self) -> Value {
				self.server_key.to_json()
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				Ok(Self {
					server_key: Raw::from_json(body)?,
				})
			}
		}
	}
}

pub mod get_remote_server_keys {
	pub mod v2 {
		use alloc::vec::Vec;

		use super::super::ServerSigningKeys;
		use crate::{MilliSecondsSinceUnixEpoch, OwnedServerName, sswire::Raw};

		pub struct Request {
			pub server_name: OwnedServerName,
			pub minimum_valid_until_ts: Option<MilliSecondsSinceUnixEpoch>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/key/v2/query/{server_name}");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.server_name,
				)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"minimum_valid_until_ts",
					crate::endpoint::enc(&self.minimum_valid_until_ts),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					server_name: input.path()?,
					minimum_valid_until_ts: input.query("minimum_valid_until_ts")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub server_keys: Vec<Raw<ServerSigningKeys>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"server_keys",
					crate::endpoint::enc(&self.server_keys),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					server_keys: input.body("server_keys")?,
				})
			}
		}
	}
}

pub mod get_remote_server_keys_batch {
	pub mod v2 {
		use alloc::{collections::BTreeMap, vec::Vec};

		use super::super::ServerSigningKeys;
		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedServerName, OwnedServerSigningKeyId, sswire::Raw,
		};

		/// Constraints on the keys a notary returns for one server.
		#[derive(Clone, Debug, Default)]
		pub struct QueryCriteria {
			pub minimum_valid_until_ts: Option<MilliSecondsSinceUnixEpoch>,
		}

		impl crate::codec::Serialize for QueryCriteria {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					stringify!(minimum_valid_until_ts),
					crate::endpoint::enc(&self.minimum_valid_until_ts),
				)])
			}
		}
		impl crate::codec::Deserialize for QueryCriteria {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(QueryCriteria)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					minimum_valid_until_ts: input.body(stringify!(minimum_valid_until_ts))?,
				})
			}
		}

		pub struct Request {
			pub server_keys:
				BTreeMap<OwnedServerName, BTreeMap<OwnedServerSigningKeyId, QueryCriteria>>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/key/v2/query");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("server_keys", crate::endpoint::enc(&self.server_keys))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					server_keys: input.body("server_keys")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub server_keys: Vec<Raw<ServerSigningKeys>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"server_keys",
					crate::endpoint::enc(&self.server_keys),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					server_keys: input.body("server_keys")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::collections::BTreeMap;

	use super::*;
	use crate::{
		codec::{from_str, to_string},
		sswire::Raw,
	};

	#[test]
	fn key_response_with_a_deep_duplicate_key_is_rejected_at_parse_time() {
		use crate::endpoint::IncomingResponse;

		let decode = |body: &str| {
			get_server_keys::v2::Response::try_from_http_response(
				http::Response::builder()
					.status(http::StatusCode::OK)
					.body(body.as_bytes().to_vec())
					.unwrap(),
			)
		};
		let clean = r#"{"server_name":"example.org","valid_until_ts":1,"verify_keys":{"ed25519:a":{"key":"AAA"}}}"#;
		let deep_duplicate = r#"{"server_name":"example.org","valid_until_ts":1,"verify_keys":{"ed25519:a":{"key":"AAA","key":"AAA"}}}"#;
		assert!(decode(clean).is_ok());
		// Identical values: deduplicating would lose nothing and a signature would
		// still verify, so only the parse can reject it.
		assert!(decode(deep_duplicate).is_err());
	}

	#[test]
	fn server_signing_keys_round_trip() {
		let mut keys = ServerSigningKeys::new(
			OwnedServerName::parse("example.org").unwrap(),
			MilliSecondsSinceUnixEpoch(99),
		);
		keys.verify_keys.insert(
			OwnedServerSigningKeyId::parse("ed25519:a").unwrap(),
			VerifyKey::new(Base64::new(alloc::vec![1, 2, 3])),
		);
		keys.old_verify_keys.insert(
			OwnedServerSigningKeyId::parse("ed25519:b").unwrap(),
			OldVerifyKey::new(MilliSecondsSinceUnixEpoch(5), Base64::new(alloc::vec![4])),
		);
		let mut inner = BTreeMap::new();
		inner.insert(crate::OwnedKeyId::parse("ed25519:a").unwrap(), String::from("sig"));
		keys.signatures.insert(OwnedServerName::parse("example.org").unwrap(), inner);

		let back: ServerSigningKeys = from_str(&to_string(&keys)).unwrap();
		assert_eq!(back, keys);

		let raw = Raw::<ServerSigningKeys>::new(&keys).unwrap();
		assert_eq!(raw.deserialize().unwrap(), keys);
	}
}

//! Server discovery and signing-key endpoints.

use alloc::collections::BTreeMap;

use crate::{
	MilliSecondsSinceUnixEpoch, OwnedServerName, OwnedServerSigningKeyId, Signatures,
	serde::Base64,
};

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

crate::impl_codec_struct!(VerifyKey {
	key: Base64
});

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

crate::impl_codec_struct!(OldVerifyKey {
	expired_ts: MilliSecondsSinceUnixEpoch,
	key: Base64
});

/// A server's published signing keys.
#[derive(Clone, Debug, Eq, PartialEq)]
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

crate::impl_codec_struct!(ServerSigningKeys {
	server_name: OwnedServerName,
	verify_keys: BTreeMap<OwnedServerSigningKeyId, VerifyKey>,
	old_verify_keys: BTreeMap<OwnedServerSigningKeyId, OldVerifyKey>,
	signatures: Signatures,
	valid_until_ts: MilliSecondsSinceUnixEpoch,
});

pub mod get_server_version {
	pub mod v1 {
		use alloc::string::String;

		use crate::endpoint;

		/// Server software name and version.
		#[derive(Clone, Debug, Default)]
		pub struct Server {
			pub name: Option<String>,
			pub version: Option<String>,
		}

		crate::impl_codec_struct!(Server { name: Option<String>, version: Option<String> });

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/version",
			request { path {} query {} body {} }
			response { server: Option<Server> }
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
			serde::Raw,
		};

		/// Request for this server's own signing keys.
		#[derive(Clone, Debug, Default)]
		pub struct Request;

		/// The keys document is the whole response body.
		#[derive(Clone, Debug)]
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

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata {
				method: "GET",
				path: "/_matrix/key/v2/server",
			};

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
		use crate::{MilliSecondsSinceUnixEpoch, OwnedServerName, endpoint, serde::Raw};

		endpoint! {
			method: "GET", path: "/_matrix/key/v2/query/{server_name}",
			request {
				path { server_name: OwnedServerName }
				query { minimum_valid_until_ts: MilliSecondsSinceUnixEpoch }
				body {}
			}
			response { server_keys: Vec<Raw<ServerSigningKeys>> }
		}
	}
}

pub mod get_remote_server_keys_batch {
	pub mod v2 {
		use alloc::{collections::BTreeMap, vec::Vec};

		use super::super::ServerSigningKeys;
		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedServerName, OwnedServerSigningKeyId, endpoint,
			serde::Raw,
		};

		/// Constraints on the keys a notary returns for one server.
		#[derive(Clone, Debug, Default)]
		pub struct QueryCriteria {
			pub minimum_valid_until_ts: Option<MilliSecondsSinceUnixEpoch>,
		}

		crate::impl_codec_struct!(QueryCriteria {
			minimum_valid_until_ts: Option<MilliSecondsSinceUnixEpoch>,
		});

		endpoint! {
			method: "POST", path: "/_matrix/key/v2/query",
			request {
				path {}
				query {}
				body {
					server_keys: BTreeMap<OwnedServerName, BTreeMap<OwnedServerSigningKeyId, QueryCriteria>>
				}
			}
			response { server_keys: Vec<Raw<ServerSigningKeys>> }
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::collections::BTreeMap;

	use super::*;
	use crate::{
		codec::{from_str, to_string},
		serde::Raw,
	};

	#[test]
	fn server_signing_keys_round_trip() {
		let mut keys = ServerSigningKeys::new(
			OwnedServerName::from("example.org"),
			MilliSecondsSinceUnixEpoch(99),
		);
		keys.verify_keys.insert(
			OwnedServerSigningKeyId::from("ed25519:a"),
			VerifyKey::new(Base64::new(alloc::vec![1, 2, 3])),
		);
		keys.old_verify_keys.insert(
			OwnedServerSigningKeyId::from("ed25519:b"),
			OldVerifyKey::new(MilliSecondsSinceUnixEpoch(5), Base64::new(alloc::vec![4])),
		);
		let mut inner = BTreeMap::new();
		inner.insert(crate::OwnedKeyId::from("ed25519:a"), String::from("sig"));
		keys.signatures.insert(OwnedServerName::from("example.org"), inner);

		let back: ServerSigningKeys = from_str(&to_string(&keys)).unwrap();
		assert_eq!(back, keys);

		let raw = Raw::<ServerSigningKeys>::new(&keys).unwrap();
		assert_eq!(raw.deserialize().unwrap(), keys);
	}
}

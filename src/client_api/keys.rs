pub mod upload_keys {
	pub mod v3 {
		use crate::{OwnedOneTimeKeyId, encryption::DeviceKeys, sswire::Raw};
		use std::collections::BTreeMap;

		pub struct Request {
			pub device_keys: Option<Raw<DeviceKeys>>,
			pub one_time_keys: BTreeMap<OwnedOneTimeKeyId, Raw<crate::encryption::OneTimeKey>>,
			pub fallback_keys: BTreeMap<OwnedOneTimeKeyId, Raw<crate::encryption::OneTimeKey>>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/keys/upload",
			path {}
			query {}
			body { device_keys => "device_keys", one_time_keys => "one_time_keys" = BTreeMap::new(), fallback_keys => "fallback_keys" = BTreeMap::new() }
		}
		pub struct Response {
			pub one_time_key_counts: BTreeMap<crate::OneTimeKeyAlgorithm, crate::UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { one_time_key_counts => "one_time_key_counts" }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::upload_keys::v3::Request;
	use crate::{
		endpoint::EndpointRequest,
		json::{Object, Value},
	};
	use std::collections::BTreeMap;

	fn parse(body: &Value) -> Request {
		Request::from_parts(&[], &[], Some(body)).unwrap()
	}

	#[test]
	fn upload_keys_defaults_optional_maps() {
		let request = parse(&Value::Object(Object::new()));
		assert!(request.device_keys.is_none());
		assert!(request.one_time_keys.is_empty());
		assert!(request.fallback_keys.is_empty());

		let request = parse(&Value::Object(Object::from([("device_keys".into(), Value::Null)])));
		assert!(request.device_keys.is_none());
	}

	#[test]
	fn upload_keys_serializes_all_wire_fields() {
		let request = Request {
			device_keys: None,
			one_time_keys: BTreeMap::default(),
			fallback_keys: BTreeMap::default(),
		};
		assert!(request.body().unwrap().as_object().unwrap().contains_key("one_time_keys"));
	}
}

pub mod get_keys {
	pub mod v3 {
		use crate::{
			OwnedDeviceId, OwnedUserId,
			encryption::{CrossSigningKey, DeviceKeys},
			sswire::Raw,
		};
		use std::collections::BTreeMap;

		pub struct Request {
			pub device_keys: BTreeMap<OwnedUserId, Vec<OwnedDeviceId>>,
			pub timeout: Option<std::time::Duration>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/keys/query",
			path {}
			query {}
			body { device_keys => "device_keys", timeout => "timeout" }
		}
		pub struct Response {
			pub device_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<DeviceKeys>>>,
			pub master_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
			pub self_signing_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
			pub user_signing_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
			pub failures: BTreeMap<String, crate::json::Value>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { device_keys => "device_keys", master_keys => "master_keys", self_signing_keys => "self_signing_keys", user_signing_keys => "user_signing_keys", failures => "failures" }
		}
	}
}

pub mod claim_keys {
	pub mod v3 {
		use crate::{
			OneTimeKeyAlgorithm, OwnedDeviceId, OwnedOneTimeKeyId, OwnedUserId, sswire::Raw,
		};
		use std::collections::BTreeMap;

		/// The one-time keys for a single device.
		pub type OneTimeKeys = BTreeMap<
			OwnedDeviceId,
			BTreeMap<OwnedOneTimeKeyId, Raw<crate::encryption::OneTimeKey>>,
		>;

		pub struct Request {
			pub one_time_keys:
				BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, OneTimeKeyAlgorithm>>,
			pub timeout: Option<std::time::Duration>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/keys/claim",
			path {}
			query {}
			body { one_time_keys => "one_time_keys", timeout => "timeout" }
		}
		pub struct Response {
			pub one_time_keys: BTreeMap<OwnedUserId, OneTimeKeys>,
			pub failures: BTreeMap<String, crate::json::Value>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { one_time_keys => "one_time_keys", failures => "failures" }
		}

		impl Response {
			/// Creates a response with the given keys and no failures.
			#[must_use]
			pub fn new(one_time_keys: BTreeMap<OwnedUserId, OneTimeKeys>) -> Self {
				Self {
					failures: BTreeMap::new(),
					one_time_keys,
				}
			}
		}
	}
}

pub mod upload_signing_keys {
	pub mod v3 {
		use crate::{encryption::CrossSigningKey, sswire::Raw};

		pub struct Request {
			pub auth: Option<crate::uiaa::AuthData>,
			pub master_key: Option<Raw<CrossSigningKey>>,
			pub self_signing_key: Option<Raw<CrossSigningKey>>,
			pub user_signing_key: Option<Raw<CrossSigningKey>>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/keys/device_signing/upload",
			path {}
			query {}
			body { auth => "auth", master_key => "master_key", self_signing_key => "self_signing_key", user_signing_key => "user_signing_key" }
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body {}
		}
	}
}

pub mod upload_signatures {
	pub mod v3 {
		use crate::{OwnedDeviceId, OwnedUserId, encryption::CrossSigningKey, sswire::Raw};
		use std::collections::BTreeMap;

		pub struct Request {
			pub signed_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<CrossSigningKey>>>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_raw_impl! {
			method: "POST",
			path: "/_matrix/client/v3/keys/signatures/upload",
			path {}
			query {}
			raw { signed_keys }
		}
		pub struct Response {
			pub failures: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<crate::json::Value>>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { failures => "failures" }
		}
	}
}

pub mod get_key_changes {
	pub mod v3 {
		use crate::OwnedUserId;

		pub struct Request {
			pub from: String,
			pub to: String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/keys/changes",
			path {}
			query { from => "from", to => "to" }
			body {}
		}
		pub struct Response {
			pub changed: Vec<OwnedUserId>,
			pub left: Vec<OwnedUserId>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { changed => "changed", left => "left" }
		}
	}
}

#[cfg(test)]
mod upload_signatures_tests {
	use super::upload_signatures::v3::Request;
	use crate::{codec::from_str, endpoint::EndpointRequest, json::Value};

	#[test]
	fn body_is_the_user_key_signature_map() {
		let body = from_str::<Value>(
			r#"{"@a:hs1":{"DEVICE":{"user_id":"@a:hs1","signatures":{"@a:hs1":{"ed25519:X":"sig"}}}}}"#,
		)
		.unwrap();
		let request = Request::from_parts(&[], &[], Some(&body)).unwrap();
		assert_eq!(request.signed_keys.len(), 1);
		assert_eq!(request.body().unwrap(), body);
	}
}

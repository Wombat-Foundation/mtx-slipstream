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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/keys/upload");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("device_keys", crate::endpoint::enc(&self.device_keys)),
						("one_time_keys", crate::endpoint::enc(&self.one_time_keys)),
						("fallback_keys", crate::endpoint::enc(&self.fallback_keys)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					device_keys: input.body("device_keys")?,
					one_time_keys: input.body_or("one_time_keys", BTreeMap::new())?,
					fallback_keys: input.body_or("fallback_keys", BTreeMap::new())?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub one_time_key_counts: BTreeMap<crate::OneTimeKeyAlgorithm, crate::UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"one_time_key_counts",
					crate::endpoint::enc(&self.one_time_key_counts),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					one_time_key_counts: input.body("one_time_key_counts")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/keys/query");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("device_keys", crate::endpoint::enc(&self.device_keys)),
						("timeout", crate::endpoint::enc(&self.timeout)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					device_keys: input.body("device_keys")?,
					timeout: input.body("timeout")?,
				};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("device_keys", crate::endpoint::enc(&self.device_keys)),
					("master_keys", crate::endpoint::enc(&self.master_keys)),
					("self_signing_keys", crate::endpoint::enc(&self.self_signing_keys)),
					("user_signing_keys", crate::endpoint::enc(&self.user_signing_keys)),
					("failures", crate::endpoint::enc(&self.failures)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					device_keys: input.body("device_keys")?,
					master_keys: input.body("master_keys")?,
					self_signing_keys: input.body("self_signing_keys")?,
					user_signing_keys: input.body("user_signing_keys")?,
					failures: input.body("failures")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/keys/claim");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("one_time_keys", crate::endpoint::enc(&self.one_time_keys)),
						("timeout", crate::endpoint::enc(&self.timeout)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					one_time_keys: input.body("one_time_keys")?,
					timeout: input.body("timeout")?,
				};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("one_time_keys", crate::endpoint::enc(&self.one_time_keys)),
					("failures", crate::endpoint::enc(&self.failures)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					one_time_keys: input.body("one_time_keys")?,
					failures: input.body("failures")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/keys/device_signing/upload",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("auth", crate::endpoint::enc(&self.auth)),
						("master_key", crate::endpoint::enc(&self.master_key)),
						("self_signing_key", crate::endpoint::enc(&self.self_signing_key)),
						("user_signing_key", crate::endpoint::enc(&self.user_signing_key)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					auth: input.body("auth")?,
					master_key: input.body("master_key")?,
					self_signing_key: input.body("self_signing_key")?,
					user_signing_key: input.body("user_signing_key")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [])
			}
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/keys/signatures/upload",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("signed_keys", crate::endpoint::enc(&self.signed_keys))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					signed_keys: input.body("signed_keys")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub failures: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<crate::json::Value>>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"failures",
					crate::endpoint::enc(&self.failures),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					failures: input.body("failures")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/keys/changes");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("from", crate::endpoint::enc(&self.from)),
					("to", crate::endpoint::enc(&self.to)),
				])
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
					from: input.query("from")?,
					to: input.query("to")?,
				};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("changed", crate::endpoint::enc(&self.changed)),
					("left", crate::endpoint::enc(&self.left)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					changed: input.body("changed")?,
					left: input.body("left")?,
				})
			}
		}
	}
}

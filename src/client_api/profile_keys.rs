pub mod get_profile_key {
	pub mod unstable {
		use crate::{
			OwnedUserId,
			codec::DeError,
			endpoint::{self, EndpointRequest},
			json::Value,
		};
		use std::collections::BTreeMap;

		/// The canonical path is the MSC4133 one; the stable profile path is an alias.
		pub struct Request {
			pub user_id: OwnedUserId,
			pub key_name: String,
		}
		impl core::fmt::Debug for Request {
			fn fmt(&self, f: &mut endpoint::Fmt<'_>) -> endpoint::FmtResult {
				endpoint::opaque_debug(f, "Request")
			}
		}

		impl Request {
			#[must_use]
			pub fn new(user_id: OwnedUserId, key_name: String) -> Self {
				Self {
					user_id,
					key_name,
				}
			}
		}

		impl EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata = endpoint::Metadata::new(
				"GET",
				"/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			)
			.with_aliases(&["/_matrix/client/v3/profile/{userId}/{keyName}"]);

			fn path_args(&self) -> Vec<String> {
				vec![self.user_id.to_string(), self.key_name.clone()]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				None
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = endpoint::Input::new(path, query, body);
				let request = Self {
					user_id: input.path()?,
					key_name: input.path()?,
				};
				input.finish()?;
				Ok(request)
			}
		}

		// The wire body is the bare `{"<keyName>": <value>}` map, not `{"value": ..}`.
		pub struct Response {
			pub value: BTreeMap<String, Value>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.value)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					value: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	}
}

pub mod set_profile_key {
	pub mod unstable {
		use crate::{
			OwnedUserId,
			codec::{DeError, Deserialize},
			endpoint::{self, EndpointRequest},
			json::Value,
		};
		use std::collections::BTreeMap;

		// Hand-written rather than `endpoint_request_raw!`: the body is the bare
		// map, but ruma's `#[ruma_api(body)]` decodes a completely empty body as
		// `{}` instead of rejecting it, so a missing body has to become an empty
		// map here too. `endpoint_request_raw!` is the passthrough form and stays
		// strict, which is what the `Raw` bodies want.
		#[derive(Debug)]
		pub struct Request {
			pub user_id: OwnedUserId,
			pub key_name: String,
			pub kv_pair: BTreeMap<String, Value>,
		}

		impl Request {
			#[must_use]
			pub fn new(
				user_id: OwnedUserId,
				key_name: String,
				kv_pair: BTreeMap<String, Value>,
			) -> Self {
				Self {
					user_id,
					key_name,
					kv_pair,
				}
			}
		}

		impl EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata = endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			)
			.with_aliases(&["/_matrix/client/v3/profile/{userId}/{keyName}"]);

			fn path_args(&self) -> Vec<String> {
				vec![self.user_id.to_string(), self.key_name.clone()]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				Some(Value::Object(self.kv_pair.clone()))
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = endpoint::Input::new(path, query, body);
				let kv_pair = body.map(BTreeMap::from_json).transpose()?.unwrap_or_default();
				let request = Self {
					user_id: input.path()?,
					key_name: input.path()?,
					kv_pair,
				};
				input.finish()?;
				Ok(request)
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
pub mod delete_profile_key {
	pub mod unstable {
		use crate::{
			OwnedUserId,
			codec::{DeError, Deserialize},
			endpoint::{self, EndpointRequest},
			json::Value,
		};
		use std::collections::BTreeMap;

		#[derive(Debug)]
		pub struct Request {
			pub user_id: OwnedUserId,
			pub key_name: String,
			pub kv_pair: BTreeMap<String, Value>,
		}

		impl Request {
			#[must_use]
			pub fn new(
				user_id: OwnedUserId,
				key_name: String,
				kv_pair: BTreeMap<String, Value>,
			) -> Self {
				Self {
					user_id,
					key_name,
					kv_pair,
				}
			}
		}

		impl EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata = endpoint::Metadata::new(
				"DELETE",
				"/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			)
			.with_aliases(&["/_matrix/client/v3/profile/{userId}/{keyName}"]);

			fn path_args(&self) -> Vec<String> {
				vec![self.user_id.to_string(), self.key_name.clone()]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				Some(Value::Object(self.kv_pair.clone()))
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = endpoint::Input::new(path, query, body);
				let kv_pair = body.map(BTreeMap::from_json).transpose()?.unwrap_or_default();
				let request = Self {
					user_id: input.path()?,
					key_name: input.path()?,
					kv_pair,
				};
				input.finish()?;
				Ok(request)
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

#[cfg(test)]
mod alias_tests {
	use crate::endpoint::{EndpointRequest, Metadata};

	const STABLE: &str = "/_matrix/client/v3/profile/{userId}/{keyName}";

	#[test]
	fn profile_field_endpoints_are_also_served_on_the_stable_path() {
		for metadata in [
			super::get_profile_key::unstable::Request::METADATA,
			super::set_profile_key::unstable::Request::METADATA,
			super::delete_profile_key::unstable::Request::METADATA,
		] {
			assert!(metadata.path.contains("uk.tcpip.msc4133"));
			assert_eq!(metadata.aliases, &[STABLE]);
		}
	}

	#[test]
	fn endpoints_without_aliases_have_none() {
		assert!(Metadata::new("GET", "/_matrix/client/versions").aliases.is_empty());
	}
}

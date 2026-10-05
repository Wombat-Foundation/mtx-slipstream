pub mod get_profile_key {
	pub mod unstable {
		use crate::{OwnedUserId, json::Value};
		use std::collections::BTreeMap;

		crate::endpoint_request! {
			method: "GET",
			path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			request {
				path { user_id: OwnedUserId, key_name: String }
				query {}
				body {}
			}
		}
		// The wire body is the bare `{"<keyName>": <value>}` map, not `{"value": ..}`.
		crate::endpoint_response_flat!(value: BTreeMap<String, Value>);
	}
}
pub mod set_profile_key {
	pub mod unstable {
		use crate::{OwnedUserId, json::Value};
		use std::collections::BTreeMap;

		crate::endpoint_request_raw! {
			method: "PUT",
			path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			request {
				path { user_id: OwnedUserId, key_name: String }
				query {}
				raw_body { kv_pair: BTreeMap<String, Value> }
			}
		}
		crate::endpoint_response! { response {} }
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
			);

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

		crate::endpoint_response! { response {} }
	}
}

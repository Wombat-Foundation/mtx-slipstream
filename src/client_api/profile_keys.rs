pub mod get_profile_key {
	pub mod unstable {
		use crate::{OwnedUserId, json::Value};
		use std::collections::BTreeMap;

		crate::endpoint! {
			method: "GET",
			path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			request {
				path { user_id: OwnedUserId, key_name: String }
				query {}
				body {}
			}
			response { value: BTreeMap<String, Value> }
		}
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
		use crate::OwnedUserId;
		use crate::json::Value;
		use std::collections::BTreeMap;

		crate::endpoint! {
			method: "DELETE",
			path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}",
			request {
				path { user_id: OwnedUserId, key_name: String }
				query {}
				body { kv_pair: BTreeMap<String, Value> }
			}
			response {}
		}
	}
}

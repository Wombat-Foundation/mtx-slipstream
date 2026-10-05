pub mod get_profile_key {
	pub mod unstable {
		use crate::{OwnedUserId, endpoint, json::Value};
		endpoint! { method: "GET", path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}", request { path { user_id: OwnedUserId, key_name: String } query {} body {} } response { value: Option<Value> } }
	}
}
pub mod set_profile_key {
	pub mod unstable {
		use crate::{OwnedUserId, endpoint, json::Value};
		use std::collections::BTreeMap;
		endpoint! { method: "PUT", path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}", request { path { user_id: OwnedUserId, key_name: String } query {} body { kv_pair: BTreeMap<String, Value> } } response {} }
	}
}
pub mod delete_profile_key {
	pub mod unstable {
		use crate::{OwnedUserId, endpoint, json::Value};
		use std::collections::BTreeMap;
		endpoint! { method: "DELETE", path: "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{userId}/{keyName}", request { path { user_id: OwnedUserId, key_name: String } query {} body { kv_pair: BTreeMap<String, Value> } } response {} }
	}
}

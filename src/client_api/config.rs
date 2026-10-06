pub mod get_global_account_data {
	pub mod v3 {
		use crate::{OwnedUserId, events::GlobalAccountDataEventType, sswire::Raw};
		crate::endpoint_request! { method: "GET", path: "/_matrix/client/v3/user/{userId}/account_data/{type}", request { path { user_id: OwnedUserId, event_type: GlobalAccountDataEventType } query {} body {} } }
		crate::endpoint_response_flat!(account_data: Raw<crate::events::AnyGlobalAccountDataEvent>);
	}
}
pub mod get_room_account_data {
	pub mod v3 {
		use crate::{OwnedRoomId, OwnedUserId, events::RoomAccountDataEventType, sswire::Raw};
		crate::endpoint_request! { method: "GET", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/account_data/{type}", request { path { user_id: OwnedUserId, room_id: OwnedRoomId, event_type: RoomAccountDataEventType } query {} body {} } }
		crate::endpoint_response_flat!(account_data: Raw<crate::events::AnyRoomAccountDataEvent>);
	}
}
pub mod set_global_account_data {
	pub mod v3 {
		use crate::{OwnedUserId, events::GlobalAccountDataEventType, json::Value};
		crate::endpoint_request_raw! { method: "PUT", path: "/_matrix/client/v3/user/{userId}/account_data/{type}", request { path { user_id: OwnedUserId, event_type: GlobalAccountDataEventType } query {} raw_body { data: Value } } }
		crate::endpoint_response! { response {} }
	}
}
pub mod set_room_account_data {
	pub mod v3 {
		use crate::{OwnedRoomId, OwnedUserId, events::RoomAccountDataEventType, json::Value};
		crate::endpoint_request_raw! { method: "PUT", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/account_data/{type}", request { path { user_id: OwnedUserId, room_id: OwnedRoomId, event_type: RoomAccountDataEventType } query {} raw_body { data: Value } } }
		crate::endpoint_response! { response {} }
	}
}

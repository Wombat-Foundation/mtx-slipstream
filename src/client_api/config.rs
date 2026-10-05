pub mod get_global_account_data {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint, events::GlobalAccountDataEventType, serde::Raw};
		endpoint! { method: "GET", path: "/_matrix/client/v3/user/{userId}/account_data/{type}", request { path { user_id: OwnedUserId, event_type: GlobalAccountDataEventType } query {} body {} } response { account_data: Raw<crate::events::AnyGlobalAccountDataEvent> } }
	}
}
pub mod get_room_account_data {
	pub mod v3 {
		use crate::{
			OwnedRoomId, OwnedUserId, endpoint, events::RoomAccountDataEventType, serde::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/account_data/{type}", request { path { user_id: OwnedUserId, room_id: OwnedRoomId, event_type: RoomAccountDataEventType } query {} body {} } response { account_data: Raw<crate::events::AnyRoomAccountDataEvent> } }
	}
}
pub mod set_global_account_data {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint, events::GlobalAccountDataEventType, json::Value};
		endpoint! { method: "PUT", path: "/_matrix/client/v3/user/{userId}/account_data/{type}", request { path { user_id: OwnedUserId, event_type: GlobalAccountDataEventType } query {} body { data: Value } } response {} }
	}
}
pub mod set_room_account_data {
	pub mod v3 {
		use crate::{
			OwnedRoomId, OwnedUserId, endpoint, events::RoomAccountDataEventType, json::Value,
		};
		endpoint! { method: "PUT", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/account_data/{type}", request { path { user_id: OwnedUserId, room_id: OwnedRoomId, event_type: RoomAccountDataEventType } query {} body { data: Value } } response {} }
	}
}

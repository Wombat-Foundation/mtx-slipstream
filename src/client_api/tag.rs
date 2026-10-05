pub mod create_tag {
	pub mod v3 {
		use crate::{OwnedRoomId, endpoint, events::tag::TagInfo};
		endpoint! { method: "PUT", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}", request { path { user_id: crate::OwnedUserId, room_id: OwnedRoomId, tag: String } query {} body { tag_info: TagInfo } } response {} }
	}
}
pub mod delete_tag {
	pub mod v3 {
		use crate::{OwnedRoomId, endpoint};
		endpoint! { method: "DELETE", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}", request { path { user_id: crate::OwnedUserId, room_id: OwnedRoomId, tag: String } query {} body {} } response {} }
	}
}
pub mod get_tags {
	pub mod v3 {
		use crate::{OwnedRoomId, endpoint, events::tag::TagInfo};
		use std::collections::BTreeMap;
		endpoint! { method: "GET", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags", request { path { user_id: crate::OwnedUserId, room_id: OwnedRoomId } query {} body {} } response { tags: BTreeMap<String, TagInfo> } }
	}
}

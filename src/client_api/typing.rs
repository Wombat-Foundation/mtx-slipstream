pub mod create_typing_event {
	pub mod v3 {
		use crate::{OwnedRoomId, OwnedUserId, endpoint};
		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/rooms/{roomId}/typing/{userId}",
			request { path { room_id: OwnedRoomId, user_id: OwnedUserId } query {} body { typing: bool, timeout: Option<std::time::Duration> } }
			response {}
		}
	}
}

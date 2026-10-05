pub mod report_user {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};
		endpoint! { method: "POST", path: "/_matrix/client/v3/users/{userId}/report", request { path { user_id: OwnedUserId } query {} body { reason: Option<String> } } response {} }
	}
}
pub mod room {
	pub mod report_room {
		pub mod v3 {
			use crate::{OwnedRoomId, endpoint};
			endpoint! { method: "POST", path: "/_matrix/client/v3/rooms/{roomId}/report", request { path { room_id: OwnedRoomId } query {} body { reason: Option<String>, score: Option<crate::Int> } } response {} }
		}
	}
	pub mod report_content {
		pub mod v3 {
			use crate::{OwnedEventId, OwnedRoomId, endpoint};
			endpoint! { method: "POST", path: "/_matrix/client/v3/rooms/{roomId}/report/{eventId}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId } query {} body { reason: Option<String>, score: Option<crate::Int> } } response {} }
		}
	}
}

pub mod create_typing_event {
	pub mod v3 {
		use crate::{OwnedRoomId, OwnedUserId, endpoint};
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum Typing {
			Yes,
			No,
		}
		crate::impl_codec_enum!(Typing { Yes => "typing", No => "stopped" });
		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/rooms/{roomId}/typing/{userId}",
			request { path { room_id: OwnedRoomId, user_id: OwnedUserId } query {} body { state: Typing, timeout: Option<crate::UInt> } }
			response {}
		}
	}
}

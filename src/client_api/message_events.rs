pub mod get_message_events {
	pub mod v3 {
		use crate::{
			OwnedRoomId, UInt, api::Direction, endpoint, filter::RoomEventFilter, serde::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v3/rooms/{roomId}/messages", request { path { room_id: OwnedRoomId } query { from: Option<String>, to: Option<String>, dir: Direction, limit: UInt, filter: RoomEventFilter } body {} } response { start: String, end: Option<String>, chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, state: Vec<Raw<crate::events::AnyStateEvent>> } }
	}
}

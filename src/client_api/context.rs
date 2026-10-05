pub mod get_context {
	pub mod v3 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt, endpoint, filter::RoomEventFilter, serde::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v3/rooms/{roomId}/context/{eventId}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId } query { limit: UInt, filter: Option<RoomEventFilter> } body {} } response { start: Option<OwnedEventId>, end: Option<OwnedEventId>, events_before: Vec<Raw<crate::events::AnyTimelineEvent>>, event: Option<Raw<crate::events::AnyTimelineEvent>>, events_after: Vec<Raw<crate::events::AnyTimelineEvent>>, state: Vec<Raw<crate::events::AnyStateEvent>> } }
	}
}

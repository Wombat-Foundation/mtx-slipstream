pub mod get_relating_events {
	pub mod v1 {
		use crate::{OwnedEventId, OwnedRoomId, UInt, api::Direction, endpoint, serde::Raw};
		endpoint! { method: "GET", path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId } query { from: Option<String>, to: Option<String>, limit: UInt, dir: Direction } body {} } response { chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, next_batch: Option<String>, prev_batch: Option<String>, recursion_depth: Option<UInt> } }
	}
}
pub mod get_relating_events_with_rel_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt, api::Direction, endpoint,
			events::relation::RelationType, serde::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId, rel_type: RelationType } query { from: Option<String>, to: Option<String>, limit: UInt, dir: Direction, recurse: Option<bool> } body {} } response { chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, next_batch: Option<String>, prev_batch: Option<String>, recursion_depth: Option<UInt> } }
	}
}
pub mod get_relating_events_with_rel_type_and_event_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt,
			api::Direction,
			endpoint,
			events::{TimelineEventType, relation::RelationType},
			serde::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}/{eventType}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId, rel_type: RelationType, event_type: TimelineEventType } query { from: Option<String>, to: Option<String>, limit: UInt, dir: Direction, recurse: Option<bool> } body {} } response { chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, next_batch: Option<String>, prev_batch: Option<String>, recursion_depth: Option<UInt> } }
	}
}

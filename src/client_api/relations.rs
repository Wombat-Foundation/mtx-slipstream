pub mod get_relating_events {
	pub mod v1 {
		use crate::{OwnedEventId, OwnedRoomId, UInt, api::Direction, endpoint, sswire::Raw};
		endpoint! { method: "GET", path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId } query { from: Option<String>, to: Option<String>, limit: Option<UInt>, dir: Direction, recurse: bool } body {} } response { chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, next_batch: Option<String>, prev_batch: Option<String>, recursion_depth: Option<UInt> } }
	}
}
pub mod get_relating_events_with_rel_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt, api::Direction, endpoint,
			events::relation::RelationType, sswire::Raw,
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
			sswire::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}/{eventType}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId, rel_type: RelationType, event_type: TimelineEventType } query { from: Option<String>, to: Option<String>, limit: UInt, dir: Direction, recurse: Option<bool> } body {} } response { chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, next_batch: Option<String>, prev_batch: Option<String>, recursion_depth: Option<UInt> } }
	}
}

pub mod event_relationships {
	pub mod unstable {
		use crate::{OwnedEventId, OwnedRoomId, endpoint, sswire::Raw};

		endpoint! {
			method: "POST", path: "/_matrix/client/unstable/event_relationships",
			request {
				path {}
				query {}
				body {
					event_id: OwnedEventId,
					room_id: Option<OwnedRoomId>,
					max_depth: Option<i64>,
					max_breadth: Option<i64>,
					limit: Option<i64>,
					depth_first: Option<bool>,
					recent_first: Option<bool>,
					include_parent: Option<bool>,
					include_children: Option<bool>,
					direction: Option<String>,
					batch: Option<String>
				}
			}
			response {
				events: Vec<Raw<crate::json::Value>>,
				next_batch: Option<String>,
				limited: bool
			}
		}
	}
}

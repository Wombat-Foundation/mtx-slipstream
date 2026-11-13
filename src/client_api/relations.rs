use alloc::{string::String, vec::Vec};

use crate::{UInt, sswire::Raw};

/// The shared response of the room-relations pagination endpoints.
#[derive(Debug)]
pub struct RelatingEventsResponse {
	pub chunk: Vec<Raw<crate::events::AnyTimelineEvent>>,
	pub next_batch: Option<String>,
	pub prev_batch: Option<String>,
	pub recursion_depth: Option<UInt>,
}

impl crate::endpoint::EndpointResponse for RelatingEventsResponse {
	fn to_body(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			("chunk", crate::endpoint::enc(&self.chunk)),
			("next_batch", crate::endpoint::enc(&self.next_batch)),
			("prev_batch", crate::endpoint::enc(&self.prev_batch)),
			("recursion_depth", crate::endpoint::enc(&self.recursion_depth)),
		])
	}
	fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		let input = crate::endpoint::Input::body_only(body);
		Ok(Self {
			chunk: input.body("chunk")?,
			next_batch: input.body("next_batch")?,
			prev_batch: input.body("prev_batch")?,
			recursion_depth: input.body("recursion_depth")?,
		})
	}
}

pub mod get_relating_events {
	pub mod v1 {
		use crate::{OwnedEventId, OwnedRoomId, UInt, api::Direction};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub from: Option<String>,
			pub to: Option<String>,
			pub limit: Option<UInt>,
			pub dir: Direction,
			pub recurse: bool,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}",
			path { room_id, event_id }
			query { from => "from", to => "to", limit => "limit", dir => "dir" = Direction::Backward, recurse => "recurse" = false }
			body {}
		}
		pub type Response = crate::client_api::relations::RelatingEventsResponse;
	}
}
pub mod get_relating_events_with_rel_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt, api::Direction, events::relation::RelationType,
		};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub rel_type: RelationType,
			pub from: Option<String>,
			pub to: Option<String>,
			pub limit: UInt,
			pub dir: Direction,
			pub recurse: Option<bool>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}",
			path { room_id, event_id, rel_type }
			query { from => "from", to => "to", limit => "limit" = 10, dir => "dir" = Direction::Backward, recurse => "recurse" }
			body {}
		}
		pub type Response = crate::client_api::relations::RelatingEventsResponse;
	}
}
pub mod get_relating_events_with_rel_type_and_event_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt,
			api::Direction,
			events::{TimelineEventType, relation::RelationType},
		};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub rel_type: RelationType,
			pub event_type: TimelineEventType,
			pub from: Option<String>,
			pub to: Option<String>,
			pub limit: UInt,
			pub dir: Direction,
			pub recurse: Option<bool>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}/{eventType}",
			path { room_id, event_id, rel_type, event_type }
			query { from => "from", to => "to", limit => "limit" = 10, dir => "dir" = Direction::Backward, recurse => "recurse" }
			body {}
		}
		pub type Response = crate::client_api::relations::RelatingEventsResponse;
	}
}

/// Declares the shared MSC2836 event-relationships request.
///
/// Client and federation variants differ only in their method, path and response.
#[doc(hidden)]
#[macro_export]
macro_rules! event_relationships_request {
	($method:literal, $path:literal) => {
		pub struct Request {
			pub event_id: $crate::OwnedEventId,
			pub room_id: Option<$crate::OwnedRoomId>,
			pub max_depth: Option<i64>,
			pub max_breadth: Option<i64>,
			pub limit: Option<i64>,
			pub depth_first: Option<bool>,
			pub recent_first: Option<bool>,
			pub include_parent: Option<bool>,
			pub include_children: Option<bool>,
			pub direction: Option<::alloc::string::String>,
			pub batch: Option<::alloc::string::String>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut $crate::endpoint::Fmt<'_>) -> $crate::endpoint::FmtResult {
				$crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: $crate::endpoint::Metadata =
			<Request as $crate::endpoint::EndpointRequest>::METADATA;
		$crate::endpoint_request_impl! {
			method: $method,
			path: $path,
			path {}
			query {}
			body {
				event_id => "event_id", room_id => "room_id", max_depth => "max_depth",
				max_breadth => "max_breadth", limit => "limit", depth_first => "depth_first",
				recent_first => "recent_first", include_parent => "include_parent",
				include_children => "include_children", direction => "direction", batch => "batch"
			}
		}
	};
}

pub mod event_relationships {
	pub mod unstable {
		use crate::sswire::Raw;

		crate::event_relationships_request!(
			"POST",
			"/_matrix/client/unstable/event_relationships"
		);
		pub struct Response {
			pub events: Vec<Raw<crate::json::Value>>,
			pub next_batch: Option<String>,
			pub limited: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { events => "events", next_batch => "next_batch", limited => "limited" }
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{api::Direction, endpoint::EndpointRequest};

	fn path(extra: &[&str]) -> Vec<String> {
		["!room:example.org", "$event"].iter().chain(extra).map(|s| (*s).to_owned()).collect()
	}

	#[test]
	fn relations_query_defaults() {
		let request =
			super::get_relating_events::v1::Request::from_parts(&path(&[]), &[], None).unwrap();
		assert_eq!(request.dir, Direction::Backward);
		assert!(!request.recurse);
		assert!(request.limit.is_none());

		let request = super::get_relating_events_with_rel_type::v1::Request::from_parts(
			&path(&["m.annotation"]),
			&[],
			None,
		)
		.unwrap();
		assert_eq!(request.dir, Direction::Backward);
		assert_eq!(request.limit, 10);
	}
}

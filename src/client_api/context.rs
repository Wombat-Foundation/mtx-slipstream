pub mod get_context {
	pub mod v3 {
		use crate::{OwnedEventId, OwnedRoomId, UInt, filter::RoomEventFilter, sswire::Raw};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub limit: UInt,
			pub filter: Option<RoomEventFilter>,
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
			path: "/_matrix/client/v3/rooms/{roomId}/context/{eventId}",
			path { room_id, event_id }
			query { limit => "limit" = 10, filter => "filter" }
			body {}
		}
		pub struct Response {
			pub start: Option<String>,
			pub end: Option<String>,
			pub events_before: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub event: Option<Raw<crate::events::AnyTimelineEvent>>,
			pub events_after: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub state: Vec<Raw<crate::events::AnyStateEvent>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { start => "start", end => "end", events_before => "events_before", event => "event", events_after => "events_after", state => "state" }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::get_context::v3::Request;
	use crate::endpoint::EndpointRequest;

	#[test]
	fn context_limit_defaults_to_ten() {
		let path = ["!room:example.org".to_owned(), "$event".to_owned()];
		let request = Request::from_parts(&path, &[], None).unwrap();
		assert_eq!(request.limit, 10);
		assert!(request.filter.is_none());
	}
}

pub mod get_context {
	pub mod v3 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt, endpoint, filter::RoomEventFilter, sswire::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v3/rooms/{roomId}/context/{eventId}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId } query { limit: UInt = 10, filter: Option<RoomEventFilter> } body {} } response { start: Option<String>, end: Option<String>, events_before: Vec<Raw<crate::events::AnyTimelineEvent>>, event: Option<Raw<crate::events::AnyTimelineEvent>>, events_after: Vec<Raw<crate::events::AnyTimelineEvent>>, state: Vec<Raw<crate::events::AnyStateEvent>> } }
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

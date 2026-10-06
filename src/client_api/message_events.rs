pub mod get_message_events {
	pub mod v3 {
		use crate::{
			OwnedRoomId, UInt, api::Direction, endpoint, filter::RoomEventFilter, sswire::Raw,
		};
		endpoint! { method: "GET", path: "/_matrix/client/v3/rooms/{roomId}/messages", request { path { room_id: OwnedRoomId } query { from: Option<String>, to: Option<String>, dir: Direction = Direction::Backward, limit: UInt = 10, filter: RoomEventFilter = RoomEventFilter::default() } body {} } response { start: String, end: Option<String>, chunk: Vec<Raw<crate::events::AnyTimelineEvent>>, state: Vec<Raw<crate::events::AnyStateEvent>> } }
	}
}

#[cfg(test)]
mod tests {
	use super::get_message_events::v3::Request;
	use crate::{api::Direction, endpoint::EndpointRequest};

	fn parse(query: &[(&str, &str)]) -> Request {
		let path = ["!room:example.org".to_owned()];
		let query: Vec<_> =
			query.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect();
		Request::from_parts(&path, &query, None).unwrap()
	}

	#[test]
	fn messages_query_defaults_match_the_spec() {
		let request = parse(&[]);
		assert_eq!(request.limit, 10);
		assert_eq!(request.dir, Direction::Backward);
		assert!(request.from.is_none() && request.to.is_none());
	}

	#[test]
	fn messages_query_explicit_values_win() {
		let request = parse(&[("limit", "3"), ("dir", "f"), ("from", "t1")]);
		assert_eq!(request.limit, 3);
		assert_eq!(request.dir, Direction::Forward);
		assert_eq!(request.from.as_deref(), Some("t1"));
	}
}

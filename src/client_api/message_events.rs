pub mod get_message_events {
	pub mod v3 {
		use crate::{OwnedRoomId, UInt, api::Direction, filter::RoomEventFilter, sswire::Raw};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub from: Option<String>,
			pub to: Option<String>,
			pub dir: Direction,
			pub limit: UInt,
			pub filter: RoomEventFilter,
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
			path: "/_matrix/client/v3/rooms/{roomId}/messages",
			path { room_id }
			query { from => "from", to => "to", dir => "dir", limit => "limit" = 10, filter => "filter" = RoomEventFilter::default() }
			body {}
		}
		pub struct Response {
			pub start: String,
			pub end: Option<String>,
			pub chunk: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub state: Vec<Raw<crate::events::AnyStateEvent>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { start => "start", end => "end", chunk => "chunk", state => "state" }
		}
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
	fn messages_query_requires_direction() {
		assert!(Request::from_parts(&["!room:example.org".to_owned()], &[], None).is_err());
	}

	#[test]
	fn messages_query_explicit_values_win() {
		let request = parse(&[("limit", "3"), ("dir", "f"), ("from", "t1")]);
		assert_eq!(request.limit, 3);
		assert_eq!(request.dir, Direction::Forward);
		assert_eq!(request.from.as_deref(), Some("t1"));
	}
}

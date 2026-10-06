pub use crate::client_api::message_events::get_message_events;

pub mod send_message_event {
	pub mod v3 {
		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedTransactionId,
			events::MessageLikeEventType, sswire::Raw,
		};

		// The request body is the event content as given, not an object with a
		// `body` key. (`delay` is a query parameter handled by the router.)
		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/client/v3/rooms/{roomId}/send/{eventType}/{txnId}",
			request {
				path { room_id: OwnedRoomId, event_type: MessageLikeEventType, txn_id: OwnedTransactionId }
				query { timestamp: Option<MilliSecondsSinceUnixEpoch> }
				raw_body { body: Raw<crate::events::AnyMessageLikeEventContent> }
			}
		}
		crate::endpoint_response! { response { event_id: OwnedEventId } }
	}
}

#[cfg(test)]
mod tests {
	use super::send_message_event::v3::Request;
	use crate::{endpoint::EndpointRequest, json::Value};

	fn path() -> [String; 3] {
		["!room:example.org".into(), "m.reaction".into(), "1".into()]
	}

	#[test]
	fn send_message_event_takes_the_whole_body_as_content() {
		// A reaction has no `body` key; the content must still come through.
		let content = Value::parse(
			r#"{"m.relates_to":{"rel_type":"m.annotation","event_id":"$e","key":"x"}}"#,
		)
		.unwrap();
		let request = Request::from_parts(&path(), &[], Some(&content)).unwrap();
		assert!(request.body.0.contains("m.relates_to"));
		assert!(request.timestamp.is_none());
	}

	#[test]
	fn send_message_event_round_trips_content_with_a_body_key() {
		let content = Value::parse(r#"{"msgtype":"m.text","body":"hello"}"#).unwrap();
		let request = Request::from_parts(&path(), &[], Some(&content)).unwrap();
		assert!(request.body.0.contains("msgtype") && request.body.0.contains("hello"));
		assert_eq!(request.body(), Some(content));
	}
}

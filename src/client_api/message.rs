pub mod send_message_event {
	pub mod v3 {
		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedTransactionId, endpoint,
			events::MessageLikeEventType, serde::Raw,
		};

		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/rooms/{roomId}/send/{eventType}/{txnId}",
			request {
				path { room_id: OwnedRoomId, event_type: MessageLikeEventType, txn_id: OwnedTransactionId }
				query { timestamp: Option<MilliSecondsSinceUnixEpoch> }
				body { body: Raw<crate::events::AnyMessageLikeEventContent>, delay: Option<crate::UInt> }
			}
			response { event_id: OwnedEventId }
		}
	}
}

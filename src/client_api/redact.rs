pub mod redact_event {
	pub mod v3 {
		use crate::{OwnedEventId, OwnedRoomId, OwnedTransactionId, endpoint};
		endpoint! { method: "PUT", path: "/_matrix/client/v3/rooms/{roomId}/redact/{eventId}/{txnId}", request { path { room_id: OwnedRoomId, event_id: OwnedEventId, txn_id: OwnedTransactionId } query {} body { reason: Option<String> } } response { event_id: OwnedEventId } }
	}
}

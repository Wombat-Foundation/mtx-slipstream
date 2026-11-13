pub mod redact_event {
	pub mod v3 {
		use crate::{OwnedEventId, OwnedRoomId, OwnedTransactionId};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub txn_id: OwnedTransactionId,
			pub reason: Option<String>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "PUT",
			path: "/_matrix/client/v3/rooms/{roomId}/redact/{eventId}/{txnId}",
			path { room_id, event_id, txn_id }
			query {}
			body { reason => "reason" }
		}
		pub struct Response {
			pub event_id: OwnedEventId,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { event_id => "event_id" }
		}
	}
}

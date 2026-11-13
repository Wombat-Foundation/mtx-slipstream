//! Event authorization chain.

pub mod get_event_authorization {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
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
			path: "/_matrix/federation/v1/event_auth/{room_id}/{event_id}",
			path { room_id, event_id }
			query {}
			body {}
		}
		pub struct Response {
			pub auth_chain: Vec<RawPdu>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			strict;
			body { auth_chain => "auth_chain" }
		}
	}
}

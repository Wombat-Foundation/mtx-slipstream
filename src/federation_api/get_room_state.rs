//! Fetch the state of a room from a remote homeserver.

use alloc::vec::Vec;

use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

pub mod v1 {
	use super::{OwnedEventId, OwnedRoomId, RawPdu, Vec};

	pub struct Request {
		pub room_id: OwnedRoomId,
		pub event_id: OwnedEventId,
	}
	impl ::core::fmt::Debug for Request {
		fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
			crate::endpoint::opaque_debug(f, "Request")
		}
	}
	const _: crate::endpoint::Metadata = <Request as crate::endpoint::EndpointRequest>::METADATA;
	crate::endpoint_request_impl! {
		method: "GET",
		path: "/_matrix/federation/v1/state/{room_id}",
		path { room_id }
		query { event_id => "event_id" }
		body {}
	}
	pub struct Response {
		pub pdus: Vec<RawPdu>,
		pub auth_chain: Vec<RawPdu>,
	}
	impl ::core::fmt::Debug for Response {
		fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
			crate::endpoint::opaque_debug(f, "Response")
		}
	}
	crate::endpoint_response_impl! {
		strict;
		body { pdus => "pdus", auth_chain => "auth_chain" }
	}

	impl Request {
		#[must_use]
		pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId) -> Self {
			Self {
				room_id,
				event_id,
			}
		}
	}
}

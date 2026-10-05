//! Fetch the state of a room from a remote homeserver.

use alloc::vec::Vec;

use crate::{OwnedEventId, OwnedRoomId, endpoint, federation_api::RawPdu};

pub mod v1 {
	use super::{endpoint, OwnedRoomId, OwnedEventId, Vec, RawPdu};

	endpoint! {
		method: "GET", path: "/_matrix/federation/v1/state/{room_id}",
		request {
			path { room_id: OwnedRoomId }
			query { event_id: OwnedEventId }
			body {}
		}
		response {
			pdus: Vec<RawPdu>,
			auth_chain: Vec<RawPdu>
		}
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

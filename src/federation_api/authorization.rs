//! Event authorization chain.

pub mod get_event_authorization {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{OwnedEventId, OwnedRoomId, endpoint, federation_api::RawPdu};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/event_auth/{room_id}/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query {}
				body {}
			}
			response { auth_chain: Vec<RawPdu> }
		}
	}
}

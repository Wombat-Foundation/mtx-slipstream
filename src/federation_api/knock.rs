//! Knocking on rooms over federation.

pub mod send_knock {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{
			OwnedEventId, OwnedRoomId, endpoint_request_raw, federation_api::RawPdu, json::Value,
			serde::Raw,
		};

		endpoint_request_raw! {
			method: "PUT", path: "/_matrix/federation/v1/send_knock/{room_id}/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query {}
				raw_body { pdu: RawPdu }
			}
		}
		crate::endpoint_response! { response { knock_room_state: Vec<Raw<Value>> } }
	}
}

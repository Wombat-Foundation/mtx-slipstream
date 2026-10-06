//! Knocking on rooms over federation.

pub mod send_knock {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{
			OwnedEventId, OwnedRoomId, endpoint_request_raw, federation_api::RawPdu, json::Value,
			sswire::Raw,
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

pub mod create_knock_event_template {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{OwnedRoomId, OwnedUserId, RoomVersionId, endpoint, federation_api::RawPdu};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/make_knock/{room_id}/{user_id}",
			request {
				path { room_id: OwnedRoomId, user_id: OwnedUserId }
				query { ver: Vec<RoomVersionId> }
				body {}
			}
			response { room_version: RoomVersionId, event: RawPdu }
		}
	}
}

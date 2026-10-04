//! Joining and leaving rooms over federation.

use alloc::{string::String, vec::Vec};

use crate::{federation_api::RawPdu, impl_codec_struct};

/// Room state returned from a join, in the v1 layout.
#[derive(Clone, Debug)]
pub struct RoomStateV1 {
	pub origin: String,
	pub auth_chain: Vec<RawPdu>,
	pub state: Vec<RawPdu>,
	pub event: Option<RawPdu>,
}

impl_codec_struct!(RoomStateV1 {
	origin: String,
	auth_chain: Vec<RawPdu>,
	state: Vec<RawPdu>,
	event: Option<RawPdu>,
});

/// Room state returned from a join, in the v2 layout.
#[derive(Clone, Debug)]
pub struct RoomStateV2 {
	pub auth_chain: Vec<RawPdu>,
	pub state: Vec<RawPdu>,
	pub event: Option<RawPdu>,
	pub members_omitted: bool,
	pub servers_in_room: Option<Vec<String>>,
}

impl_codec_struct!(RoomStateV2 {
	auth_chain: Vec<RawPdu>,
	state: Vec<RawPdu>,
	event: Option<RawPdu>,
	members_omitted: bool,
	servers_in_room: Option<Vec<String>>,
});

pub mod prepare_join_event {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{OwnedRoomId, OwnedUserId, RoomVersionId, endpoint, federation_api::RawPdu};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/make_join/{room_id}/{user_id}",
			request {
				path { room_id: OwnedRoomId, user_id: OwnedUserId }
				query { ver: Vec<RoomVersionId> }
				body {}
			}
			response { room_version: Option<RoomVersionId>, event: RawPdu }
		}
	}
}

pub mod prepare_leave_event {
	pub mod v1 {
		use crate::{OwnedRoomId, OwnedUserId, RoomVersionId, endpoint, federation_api::RawPdu};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/make_leave/{room_id}/{user_id}",
			request {
				path { room_id: OwnedRoomId, user_id: OwnedUserId }
				query {}
				body {}
			}
			response { room_version: Option<RoomVersionId>, event: RawPdu }
		}
	}
}

pub mod create_join_event {
	pub use super::{RoomStateV1, RoomStateV2};

	pub mod v1 {
		use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

		/// The v1 room state type.
		pub type RoomState = super::RoomStateV1;

		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/federation/v1/send_join/{room_id}/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query {}
				raw_body { pdu: RawPdu }
			}
		}
		crate::endpoint_response_status_array!(room_state: RoomState);
	}

	pub mod v2 {
		use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

		/// The v2 room state type.
		pub type RoomState = super::RoomStateV2;

		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/federation/v2/send_join/{room_id}/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query { omit_members: bool }
				raw_body { pdu: RawPdu }
			}
		}
		crate::endpoint_response_flat!(room_state: RoomState);
	}
}

pub mod create_leave_event {
	pub mod v1 {
		use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/federation/v1/send_leave/{room_id}/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query {}
				raw_body { pdu: RawPdu }
			}
		}
		crate::endpoint_response_status_array!(empty: crate::json::Value);

		impl Response {
			#[must_use]
			pub fn new() -> Self {
				Self {
					empty: crate::json::Value::Object(crate::json::Object::new()),
				}
			}
		}

		impl Default for Response {
			fn default() -> Self {
				Self::new()
			}
		}
	}

	pub mod v2 {
		use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/federation/v2/send_leave/{room_id}/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query {}
				raw_body { pdu: RawPdu }
			}
		}
		crate::endpoint_response! { response {} }

		impl Response {
			#[must_use]
			pub fn new() -> Self {
				Self {}
			}
		}

		impl Default for Response {
			fn default() -> Self {
				Self::new()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::vec::Vec;

	use crate::{
		OwnedEventId, OwnedRoomId, OwnedUserId, RoomVersionId,
		api::{
			IncomingRequest, IncomingResponse, OutgoingRequest, OutgoingResponse, SendAccessToken,
		},
		federation_api::{
			RawPdu,
			backfill::get_backfill,
			membership::{create_join_event, prepare_join_event},
		},
		serde::Raw,
	};

	#[test]
	fn repeated_query_values_round_trip() {
		let request = prepare_join_event::v1::Request {
			room_id: OwnedRoomId::from("!r:b"),
			user_id: OwnedUserId::from("@u:b"),
			ver: alloc::vec![RoomVersionId::V10, RoomVersionId::V11],
		};
		let http = request
			.try_into_http_request::<Vec<u8>>("https://b", SendAccessToken::None, &[])
			.unwrap();
		assert!(http.uri().to_string().ends_with("?ver=10&ver=11"), "{}", http.uri());
		let parsed =
			prepare_join_event::v1::Request::try_from_http_request(http, &["!r:b", "@u:b"])
				.unwrap();
		assert_eq!(parsed.ver, alloc::vec![RoomVersionId::V10, RoomVersionId::V11]);

		let backfill = get_backfill::v1::Request {
			room_id: OwnedRoomId::from("!r:b"),
			v: alloc::vec![OwnedEventId::from("$a")],
			limit: 10,
		};
		let http = backfill
			.try_into_http_request::<Vec<u8>>("https://b", SendAccessToken::None, &[])
			.unwrap();
		let parsed = get_backfill::v1::Request::try_from_http_request(http, &["!r:b"]).unwrap();
		assert_eq!(parsed.limit, 10);
	}

	#[test]
	fn whole_body_request_and_status_array_response() {
		let pdu: RawPdu =
			Raw::new(&crate::json::Value::parse(r#"{"type":"m.room.member"}"#).unwrap()).unwrap();
		let request = create_join_event::v2::Request {
			room_id: OwnedRoomId::from("!r:b"),
			event_id: OwnedEventId::from("$e"),
			omit_members: true,
			pdu,
		};
		let http = request
			.try_into_http_request::<Vec<u8>>("https://b", SendAccessToken::None, &[])
			.unwrap();
		assert!(http.uri().to_string().contains("omit_members=true"));
		assert!(core::str::from_utf8(http.body()).unwrap().contains("m.room.member"));
		let parsed =
			create_join_event::v2::Request::try_from_http_request(http, &["!r:b", "$e"]).unwrap();
		assert!(parsed.omit_members);
		assert!(parsed.pdu.get().contains("m.room.member"));

		let v1 = create_join_event::v1::Response {
			room_state: create_join_event::RoomStateV1 {
				origin: "b".into(),
				auth_chain: Vec::new(),
				state: Vec::new(),
				event: None,
			},
		};
		let http = v1.try_into_http_response::<Vec<u8>>().unwrap();
		assert!(core::str::from_utf8(http.body()).unwrap().starts_with("[200,"));
		let back = create_join_event::v1::Response::try_from_http_response(http).unwrap();
		assert_eq!(back.room_state.origin, "b");
	}
}

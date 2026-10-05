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

pub mod create_invite {
	pub mod v2 {
		use alloc::{string::String, vec::Vec};

		use crate::{
			OwnedEventId, OwnedRoomId, OwnedServerName, RoomVersionId,
			codec::{DeError, Serialize},
			endpoint::{EndpointRequest, EndpointResponse, Input, Metadata, object_from},
			events::AnyStrippedStateEvent,
			federation_api::RawPdu,
			json::Value,
			serde::Raw,
		};

		const _: Metadata =
			Metadata::new("PUT", "/_matrix/federation/v2/invite/{room_id}/{event_id}");

		/// Invites a user on the receiving server; the `via` list uses the MSC4125 key.
		#[derive(Clone, Debug)]
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub room_version: RoomVersionId,
			pub event: RawPdu,
			pub invite_room_state: Vec<Raw<AnyStrippedStateEvent>>,
			pub via: Option<Vec<OwnedServerName>>,
		}

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata =
				Metadata::new("PUT", "/_matrix/federation/v2/invite/{room_id}/{event_id}");

			fn path_args(&self) -> Vec<String> {
				alloc::vec![
					crate::endpoint::to_param(&self.room_id).unwrap_or_default(),
					crate::endpoint::to_param(&self.event_id).unwrap_or_default(),
				]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				Some(Value::Object(object_from(alloc::vec![
					("room_version", self.room_version.to_json()),
					("event", self.event.to_json()),
					("invite_room_state", self.invite_room_state.to_json()),
					("org.matrix.msc4125.via", self.via.to_json()),
				])))
			}

			fn from_parts(
				path: &[String],
				_query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = Input::new(path, &[], body);
				Ok(Self {
					room_id: input.path()?,
					event_id: input.path()?,
					room_version: input.body("room_version")?,
					event: input.body("event")?,
					invite_room_state: input.body("invite_room_state")?,
					via: input.body_or_default("org.matrix.msc4125.via")?,
				})
			}
		}

		/// The invited user's server returns the event, countersigned.
		#[derive(Clone, Debug)]
		pub struct Response {
			pub event: RawPdu,
		}

		impl Response {
			#[must_use]
			pub fn new(event: RawPdu) -> Self {
				Self {
					event,
				}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(object_from(alloc::vec![("event", self.event.to_json())]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				Ok(Self {
					event: Input::new(&[], &[], Some(body)).body("event")?,
				})
			}
		}

		#[cfg(test)]
		mod tests {
			use super::*;

			fn json(text: &str) -> crate::json::Value {
				crate::codec::from_str(text).unwrap()
			}

			#[test]
			fn via_uses_the_msc4125_key_and_round_trips() {
				let body = json(
					r#"{
					"room_version": "11",
					"event": {"type": "m.room.member"},
					"invite_room_state": [],
					"org.matrix.msc4125.via": ["a.example"]
				}"#,
				);
				let request = Request::from_parts(
					&["!r:example.org".to_owned(), "$e".to_owned()],
					&[],
					Some(&body),
				)
				.unwrap();
				assert_eq!(request.via.as_ref().map(Vec::len), Some(1));
				let again = request.body().unwrap();
				assert!(again.get("org.matrix.msc4125.via").is_some());
				assert!(again.get("via").is_none());
			}

			#[test]
			fn via_is_optional() {
				let body =
					json(r#"{"room_version": "11", "event": {}, "invite_room_state": []}"#);
				let request = Request::from_parts(
					&["!r:example.org".to_owned(), "$e".to_owned()],
					&[],
					Some(&body),
				)
				.unwrap();
				assert!(request.via.is_none());
				assert!(request.body().unwrap().get("org.matrix.msc4125.via").is_none());
			}
		}
	}
}

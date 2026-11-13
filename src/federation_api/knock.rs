//! Knocking on rooms over federation.

pub mod send_knock {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{
			OwnedEventId, OwnedRoomId, federation_api::RawPdu, json::Value, sswire::Raw,
		};

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub pdu: RawPdu,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_raw_impl! {
			method: "PUT",
			path: "/_matrix/federation/v1/send_knock/{room_id}/{event_id}",
			path { room_id, event_id }
			query {}
			raw { pdu }
		}
		pub struct Response {
			pub knock_room_state: Vec<Raw<Value>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			strict;
			body { knock_room_state => "knock_room_state" }
		}
	}
}

pub mod create_knock_event_template {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{OwnedRoomId, OwnedUserId, RoomVersionId, federation_api::RawPdu};

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub user_id: OwnedUserId,
			pub ver: Vec<RoomVersionId>,
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
			path: "/_matrix/federation/v1/make_knock/{room_id}/{user_id}",
			path { room_id, user_id }
			query { ver => "ver" }
			body {}
		}
		pub struct Response {
			pub room_version: RoomVersionId,
			pub event: RawPdu,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { room_version => "room_version", event => "event" }
		}
	}
}

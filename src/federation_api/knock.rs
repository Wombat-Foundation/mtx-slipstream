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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/federation/v1/send_knock/{room_id}/{event_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				Some(crate::endpoint::enc(&self.pdu))
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					event_id: input.path()?,
					pdu: crate::codec::Deserialize::from_json(
						body.ok_or_else(|| crate::codec::DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub knock_room_state: Vec<Raw<Value>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"knock_room_state",
					crate::endpoint::enc(&self.knock_room_state),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					knock_room_state: _input.body("knock_room_state")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/federation/v1/make_knock/{room_id}/{user_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.user_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [("ver", crate::endpoint::enc(&self.ver))])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					user_id: input.path()?,
					ver: input.query("ver")?,
				};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("room_version", crate::endpoint::enc(&self.room_version)),
					("event", crate::endpoint::enc(&self.event)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					room_version: _input.body("room_version")?,
					event: _input.body("event")?,
				})
			}
		}
	}
}

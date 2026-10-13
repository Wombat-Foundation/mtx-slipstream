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
	impl crate::endpoint::EndpointRequest for Request {
		type Response = Response;
		const METADATA: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("GET", "/_matrix/federation/v1/state/{room_id}");
		fn path_args(&self) -> crate::endpoint::Strs {
			crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
		}
		fn query(&self) -> crate::endpoint::Pairs {
			crate::endpoint::query_pairs_mut(&mut [(
				"event_id",
				crate::endpoint::enc(&self.event_id),
			)])
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
				event_id: input.query("event_id")?,
			};
			input.finish()?;
			Ok(value)
		}
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
	impl crate::endpoint::EndpointResponse for Response {
		fn to_body(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				("pdus", crate::endpoint::enc(&self.pdus)),
				("auth_chain", crate::endpoint::enc(&self.auth_chain)),
			])
		}
		fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			let input = crate::endpoint::Input::body_only(body);
			Ok(Self {
				pdus: input.body("pdus")?,
				auth_chain: input.body("auth_chain")?,
			})
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

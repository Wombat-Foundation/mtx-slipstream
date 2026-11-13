//! Backfilling history.

pub mod get_backfill {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedServerName, UInt,
			federation_api::RawPdu,
		};

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub v: Vec<OwnedEventId>,
			pub limit: UInt,
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
			path: "/_matrix/federation/v1/backfill/{room_id}",
			path { room_id }
			query { v => "v", limit => "limit" }
			body {}
		}
		pub struct Response {
			pub origin: OwnedServerName,
			pub origin_server_ts: MilliSecondsSinceUnixEpoch,
			pub pdus: Vec<RawPdu>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			strict;
			body { origin => "origin", origin_server_ts => "origin_server_ts", pdus => "pdus" }
		}
	}
}

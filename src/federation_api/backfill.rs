//! Backfilling history.

pub mod get_backfill {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedServerName, UInt,
			endpoint, federation_api::RawPdu,
		};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/backfill/{room_id}",
			request {
				path { room_id: OwnedRoomId }
				query { v: Vec<OwnedEventId>, limit: UInt }
				body {}
			}
			response {
				origin: OwnedServerName,
				origin_server_ts: MilliSecondsSinceUnixEpoch,
				pdus: Vec<RawPdu>
			}
		}
	}
}

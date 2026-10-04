//! Federation API endpoints.

use crate::{
	OwnedEventId, OwnedRoomId, OwnedServerName, UInt, endpoint, json::Value, serde::Raw,
};

/// A PDU as it appears on the wire.
pub type RawPdu = Raw<Value>;

pub mod event {
	use super::*;

	pub mod get_event {
		pub mod v1 {
			use super::super::*;

			endpoint! {
				method: "GET", path: "/_matrix/federation/v1/event/{event_id}",
				request {
					path { event_id: OwnedEventId }
					query { include_unredacted_content: Option<bool> }
					body {}
				}
				response {
					origin: OwnedServerName,
					origin_server_ts: crate::MilliSecondsSinceUnixEpoch,
					pdu: RawPdu,
				}
			}

			impl Request {
				#[must_use]
				pub fn new(
					event_id: OwnedEventId,
					include_unredacted_content: Option<bool>,
				) -> Self {
					Self {
						event_id,
						include_unredacted_content,
					}
				}
			}
		}
	}

	pub mod get_missing_events {
		pub mod v1 {
			use super::super::*;

			endpoint! {
				method: "POST", path: "/_matrix/federation/v1/get_missing_events/{room_id}",
				request {
					path { room_id: OwnedRoomId }
					query {}
					body {
						limit: UInt,
						min_depth: UInt,
						earliest_events: Vec<OwnedEventId>,
						latest_events: Vec<OwnedEventId>
					}
				}
				response { events: Vec<RawPdu> }
			}
		}
	}

	pub mod get_room_state_ids {
		pub mod v1 {
			use super::super::*;

			endpoint! {
				method: "GET", path: "/_matrix/federation/v1/state_ids/{room_id}",
				request {
					path { room_id: OwnedRoomId }
					query { event_id: OwnedEventId }
					body {}
				}
				response {
					auth_chain_ids: Vec<OwnedEventId>,
					pdu_ids: Vec<OwnedEventId>
				}
			}

			impl Request {
				#[must_use]
				pub fn new(event_id: OwnedEventId, room_id: OwnedRoomId) -> Self {
					Self {
						room_id,
						event_id,
					}
				}
			}
		}
	}
}

pub mod discovery {
	pub mod get_server_version {
		pub mod v1 {
			use crate::endpoint;

			/// Server software name and version.
			#[derive(Clone, Debug, Default)]
			pub struct Server {
				pub name: Option<String>,
				pub version: Option<String>,
			}

			crate::impl_codec_struct!(Server { name: Option<String>, version: Option<String> });

			endpoint! {
				method: "GET", path: "/_matrix/federation/v1/version",
				request { path {} query {} body {} }
				response { server: Option<Server> }
			}
		}
	}
}

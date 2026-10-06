//! Federation API endpoints.

use crate::{json::Value, sswire::Raw};

/// A PDU as it appears on the wire.
pub type RawPdu = Raw<Value>;

pub use crate::directory::federation as directory;

pub mod openid {
	pub mod get_openid_userinfo {
		pub mod v1 {
			use crate::{OwnedUserId, endpoint};
			endpoint! {
				method: "GET", path: "/_matrix/federation/v1/openid/userinfo",
				request { path {} query { access_token: String } body {} }
				response { sub: OwnedUserId }
			}
			impl Response {
				#[must_use]
				pub fn new(sub: OwnedUserId) -> Self {
					Self {
						sub,
					}
				}
			}
		}
	}
}

pub mod edutypes {
	pub mod get_edutypes {
		pub mod unstable {
			use crate::endpoint;
			endpoint! {
				method: "GET", path: "/_matrix/federation/v1/edutypes",
				request { path {} query {} body {} }
				response { typing: bool, presence: bool, receipt: bool }
			}
		}
	}
}

pub mod keys {
	use crate::{
		OwnedDeviceId, OwnedServerName, OwnedUserId,
		encryption::{CrossSigningKey, DeviceKeys, OneTimeKey},
		endpoint,
		sswire::Raw,
	};
	use alloc::collections::BTreeMap;

	pub mod get_keys {
		pub mod v1 {
			use super::super::{
				BTreeMap, CrossSigningKey, DeviceKeys, OwnedDeviceId, OwnedServerName,
				OwnedUserId, Raw, endpoint,
			};
			endpoint! {
				method: "POST", path: "/_matrix/federation/v1/user/keys/query",
				request { path {} query {} body { device_keys: BTreeMap<OwnedUserId, alloc::vec::Vec<OwnedDeviceId>> } }
				response {
					device_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<DeviceKeys>>>,
					master_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
					self_signing_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
					failures: BTreeMap<OwnedServerName, crate::json::Value>
				}
			}
		}
	}

	pub mod claim_keys {
		pub mod v1 {
			use super::super::{
				BTreeMap, OneTimeKey, OwnedDeviceId, OwnedServerName, OwnedUserId, endpoint,
			};
			use crate::{OneTimeKeyAlgorithm, OwnedOneTimeKeyId, sswire::Raw};

			pub type OneTimeKeyClaims =
				BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, OneTimeKeyAlgorithm>>;
			pub type OneTimeKeys = BTreeMap<
				OwnedUserId,
				BTreeMap<OwnedDeviceId, BTreeMap<OwnedOneTimeKeyId, Raw<OneTimeKey>>>,
			>;

			endpoint! {
				method: "POST", path: "/_matrix/federation/v1/user/keys/claim",
				request { path {} query {} body { one_time_keys: OneTimeKeyClaims } }
				response {
					one_time_keys: OneTimeKeys,
					failures: BTreeMap<OwnedServerName, crate::json::Value>
				}
			}
		}
	}
}

/// Policy server endpoints (MSC4284).
pub mod room {
	pub mod policy_check {
		pub mod unstable {
			use alloc::string::String;

			use crate::{OwnedEventId, endpoint, federation_api::RawPdu};

			endpoint! {
				method: "POST",
				path: "/_matrix/policy/unstable/org.matrix.msc4284/event/{event_id}/check",
				request {
					path { event_id: OwnedEventId }
					query {}
					body { pdu: Option<RawPdu> }
				}
				response { recommendation: String }
			}
		}
	}

	pub mod policy_sign {
		pub mod unstable {
			use crate::{Signatures, endpoint, federation_api::RawPdu};

			endpoint! {
				method: "POST", path: "/_matrix/policy/unstable/org.matrix.msc4284/sign",
				request { path {} query {} body { pdu: RawPdu } }
				response { signatures: Option<Signatures> }
			}
		}
	}
}

pub mod event {
	pub use super::get_room_state;
	pub mod get_event {
		pub mod v1 {
			use crate::{OwnedEventId, OwnedServerName, endpoint, federation_api::RawPdu};

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

	/// MSC2836 relationship walking, used as a fallback for missing events.
	pub mod event_relationships {
		pub mod unstable {
			use alloc::{string::String, vec::Vec};

			use crate::{OwnedEventId, OwnedRoomId, endpoint, federation_api::RawPdu};

			endpoint! {
				method: "POST", path: "/_matrix/federation/unstable/event_relationships",
				request {
					path {}
					query {}
					body {
						event_id: OwnedEventId,
						room_id: Option<OwnedRoomId>,
					max_depth: Option<i64>,
					max_breadth: Option<i64>,
					limit: Option<i64>,
						depth_first: Option<bool>,
						recent_first: Option<bool>,
						include_parent: Option<bool>,
						include_children: Option<bool>,
						direction: Option<String>,
						batch: Option<String>
					}
				}
				response {
					events: Vec<RawPdu>,
					next_batch: Option<String>,
					limited: Option<bool>,
					auth_chain: Vec<RawPdu>
				}
			}
		}
	}

	pub mod get_event_by_timestamp {
		pub mod v1 {
			use crate::{
				MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, api::Direction, endpoint,
			};

			endpoint! {
				method: "GET", path: "/_matrix/federation/v1/timestamp_to_event/{room_id}",
				request {
					path { room_id: OwnedRoomId }
					query { dir: Direction, ts: MilliSecondsSinceUnixEpoch }
					body {}
				}
				response { event_id: OwnedEventId, origin_server_ts: MilliSecondsSinceUnixEpoch }
			}

			impl Response {
				#[must_use]
				pub fn new(
					event_id: OwnedEventId,
					origin_server_ts: MilliSecondsSinceUnixEpoch,
				) -> Self {
					Self {
						event_id,
						origin_server_ts,
					}
				}
			}

			impl Request {
				#[must_use]
				pub fn new(
					room_id: OwnedRoomId,
					ts: MilliSecondsSinceUnixEpoch,
					dir: Direction,
				) -> Self {
					Self {
						room_id,
						dir,
						ts,
					}
				}
			}
		}
	}

	pub mod get_missing_events {
		pub mod v1 {
			use crate::{OwnedEventId, OwnedRoomId, UInt, endpoint, federation_api::RawPdu};

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
			use crate::{OwnedEventId, OwnedRoomId, endpoint};

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

/// Authenticated media downloads.
pub mod authenticated_media {
	pub use crate::media_api::federation::{
		Content, ContentMetadata, FileOrLocation, get_content, get_content_thumbnail,
	};
}

pub mod authentication;
pub mod authorization;
pub mod backfill;
pub mod device;
pub mod discovery;
pub mod get_room_state;
pub mod knock;
pub mod membership;
pub mod query;
pub mod space;
pub mod transactions;

#[cfg(test)]
mod tests {
	use alloc::vec::Vec;

	use crate::{
		MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId,
		api::{
			IncomingRequest, IncomingResponse, MatrixVersion, OutgoingRequest, OutgoingResponse,
			SendAccessToken,
		},
		federation_api::{
			discovery::get_server_version,
			event::{get_event, get_missing_events, get_room_state_ids},
		},
	};

	#[test]
	fn get_event_request_builds_path_and_query() {
		let request =
			get_event::v1::Request::new(OwnedEventId::from("$ev:example.org"), Some(false));
		let http = request
			.try_into_http_request::<Vec<u8>>(
				"https://example.org/",
				SendAccessToken::None,
				&[MatrixVersion::V1_11],
			)
			.unwrap();
		assert_eq!(http.method(), http::Method::GET);
		assert_eq!(
			http.uri().to_string(),
			"https://example.org/_matrix/federation/v1/event/%24ev%3Aexample.org\
			 ?include_unredacted_content=false"
		);
		assert!(http.body().is_empty());
	}

	#[test]
	fn get_missing_events_request_round_trips_through_http() {
		let request = get_missing_events::v1::Request {
			room_id: OwnedRoomId::from("!room:example.org"),
			limit: 50,
			min_depth: 0,
			earliest_events: alloc::vec![OwnedEventId::from("$a")],
			latest_events: alloc::vec![OwnedEventId::from("$b")],
		};
		let http = request
			.try_into_http_request::<Vec<u8>>("https://example.org", SendAccessToken::None, &[])
			.unwrap();
		assert_eq!(http.method(), http::Method::POST);
		let parsed =
			get_missing_events::v1::Request::try_from_http_request(http, &["!room:example.org"])
				.unwrap();
		assert_eq!(parsed.limit, 50);
		assert_eq!(parsed.latest_events, alloc::vec![OwnedEventId::from("$b")]);
		assert_eq!(parsed.room_id.as_str(), "!room:example.org");
	}

	#[test]
	fn get_event_response_decodes_and_encodes() {
		let body =
			r#"{"origin":"example.org","origin_server_ts":5,"pdu":{"type":"m.room.message"}}"#;
		let response =
			http::Response::builder().status(200).body(body.as_bytes().to_vec()).unwrap();
		let decoded = get_event::v1::Response::try_from_http_response(response).unwrap();
		assert_eq!(decoded.origin.as_str(), "example.org");
		assert_eq!(decoded.origin_server_ts, MilliSecondsSinceUnixEpoch(5));
		assert!(decoded.pdu.get().contains("m.room.message"));
		let again = decoded.try_into_http_response::<Vec<u8>>().unwrap();
		assert_eq!(again.status(), http::StatusCode::OK);
	}

	#[test]
	fn error_response_becomes_matrix_error() {
		let response = http::Response::builder()
			.status(404)
			.body(br#"{"errcode":"M_NOT_FOUND","error":"nope"}"#.to_vec())
			.unwrap();
		let Err(err) = get_room_state_ids::v1::Response::try_from_http_response(response) else {
			panic!("expected error response");
		};
		assert!(err.to_string().contains("nope"));
		let _ = get_server_version::v1::Request {};
	}
}

//! Federation API endpoints.

use crate::{json::Value, sswire::Raw};

/// A PDU as it appears on the wire.
pub type RawPdu = Raw<Value>;

pub use crate::directory::federation as directory;

pub mod openid {
	pub mod get_openid_userinfo {
		pub mod v1 {
			use crate::OwnedUserId;
			pub struct Request {
				pub access_token: String,
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
				path: "/_matrix/federation/v1/openid/userinfo",
				path {}
				query { access_token => "access_token" }
				body {}
			}
			pub struct Response {
				pub sub: OwnedUserId,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { sub => "sub" }
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

			pub struct Request {}
			impl ::core::fmt::Debug for Request {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Request")
				}
			}
			const _: crate::endpoint::Metadata =
				<Request as crate::endpoint::EndpointRequest>::METADATA;
			crate::endpoint_request_impl! {
				method: "GET",
				path: "/_matrix/federation/v1/edutypes",
				path {}
				query {}
				body {}
			}
			pub struct Response {
				pub typing: bool,
				pub presence: bool,
				pub receipt: bool,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { typing => "typing", presence => "presence", receipt => "receipt" }
			}
		}
	}
}

pub mod keys {
	use crate::{
		OwnedDeviceId, OwnedServerName, OwnedUserId,
		encryption::{CrossSigningKey, DeviceKeys, OneTimeKey},
		sswire::Raw,
	};
	use alloc::collections::BTreeMap;

	pub mod get_keys {
		pub mod v1 {
			use super::super::{
				BTreeMap, CrossSigningKey, DeviceKeys, OwnedDeviceId, OwnedServerName,
				OwnedUserId, Raw,
			};
			pub struct Request {
				pub device_keys: BTreeMap<OwnedUserId, alloc::vec::Vec<OwnedDeviceId>>,
			}
			impl ::core::fmt::Debug for Request {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Request")
				}
			}
			const _: crate::endpoint::Metadata =
				<Request as crate::endpoint::EndpointRequest>::METADATA;
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/federation/v1/user/keys/query",
				path {}
				query {}
				body { device_keys => "device_keys" }
			}
			pub struct Response {
				pub device_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<DeviceKeys>>>,
				pub master_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
				pub self_signing_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>,
				pub failures: BTreeMap<OwnedServerName, crate::json::Value>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { device_keys => "device_keys", master_keys => "master_keys" = crate::endpoint::default(), self_signing_keys => "self_signing_keys" = crate::endpoint::default(), failures => "failures" = crate::endpoint::default() }
			}
		}
	}

	pub mod claim_keys {
		pub mod v1 {
			use super::super::{
				BTreeMap, OneTimeKey, OwnedDeviceId, OwnedServerName, OwnedUserId,
			};
			use crate::{OneTimeKeyAlgorithm, OwnedOneTimeKeyId, sswire::Raw};

			pub type OneTimeKeyClaims =
				BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, OneTimeKeyAlgorithm>>;
			pub type OneTimeKeys = BTreeMap<
				OwnedUserId,
				BTreeMap<OwnedDeviceId, BTreeMap<OwnedOneTimeKeyId, Raw<OneTimeKey>>>,
			>;

			pub struct Request {
				pub one_time_keys: OneTimeKeyClaims,
			}
			impl ::core::fmt::Debug for Request {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Request")
				}
			}
			const _: crate::endpoint::Metadata =
				<Request as crate::endpoint::EndpointRequest>::METADATA;
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/federation/v1/user/keys/claim",
				path {}
				query {}
				body { one_time_keys => "one_time_keys" }
			}
			pub struct Response {
				pub one_time_keys: OneTimeKeys,
				pub failures: BTreeMap<OwnedServerName, crate::json::Value>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { one_time_keys => "one_time_keys", failures => "failures" }
			}
		}
	}
}

/// Policy server endpoints (MSC4284).
pub mod room {
	pub mod policy_check {
		pub mod unstable {
			use alloc::string::String;

			use crate::{OwnedEventId, federation_api::RawPdu};

			pub struct Request {
				pub event_id: OwnedEventId,
				pub pdu: Option<RawPdu>,
			}
			impl ::core::fmt::Debug for Request {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Request")
				}
			}
			const _: crate::endpoint::Metadata =
				<Request as crate::endpoint::EndpointRequest>::METADATA;
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/policy/unstable/org.matrix.msc4284/event/{event_id}/check",
				path { event_id }
				query {}
				body { pdu => "pdu" }
			}
			pub struct Response {
				pub recommendation: String,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { recommendation => "recommendation" }
			}
		}
	}

	pub mod policy_sign {
		pub mod unstable {
			use crate::{Signatures, federation_api::RawPdu};

			pub struct Request {
				pub pdu: RawPdu,
			}
			impl ::core::fmt::Debug for Request {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Request")
				}
			}
			const _: crate::endpoint::Metadata =
				<Request as crate::endpoint::EndpointRequest>::METADATA;
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/policy/unstable/org.matrix.msc4284/sign",
				path {}
				query {}
				body { pdu => "pdu" }
			}
			pub struct Response {
				pub signatures: Option<Signatures>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { signatures => "signatures" }
			}
		}
	}
}

pub mod event {
	pub use super::get_room_state;
	pub mod get_event {
		pub mod v1 {
			use crate::{OwnedEventId, OwnedServerName, federation_api::RawPdu};

			pub struct Request {
				pub event_id: OwnedEventId,
				pub include_unredacted_content: Option<bool>,
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
				path: "/_matrix/federation/v1/event/{event_id}",
				path { event_id }
				query { include_unredacted_content => "include_unredacted_content" }
				body {}
			}
			pub struct Response {
				pub origin: OwnedServerName,
				pub origin_server_ts: crate::MilliSecondsSinceUnixEpoch,
				pub pdus: Vec<RawPdu>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { origin => "origin", origin_server_ts => "origin_server_ts", pdus => "pdus" }
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

			use crate::federation_api::RawPdu;

			crate::event_relationships_request!(
				"POST",
				"/_matrix/federation/unstable/event_relationships"
			);
			pub struct Response {
				pub events: Vec<RawPdu>,
				pub next_batch: Option<String>,
				pub limited: Option<bool>,
				pub auth_chain: Vec<RawPdu>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { events => "events", next_batch => "next_batch", limited => "limited", auth_chain => "auth_chain" }
			}
		}
	}

	pub mod get_event_by_timestamp {
		pub mod v1 {
			use crate::{MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, api::Direction};

			pub struct Request {
				pub room_id: OwnedRoomId,
				pub dir: Direction,
				pub ts: MilliSecondsSinceUnixEpoch,
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
				path: "/_matrix/federation/v1/timestamp_to_event/{room_id}",
				path { room_id }
				query { dir => "dir", ts => "ts" }
				body {}
			}
			pub struct Response {
				pub event_id: OwnedEventId,
				pub origin_server_ts: MilliSecondsSinceUnixEpoch,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { event_id => "event_id", origin_server_ts => "origin_server_ts" }
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
			use crate::{OwnedEventId, OwnedRoomId, UInt, federation_api::RawPdu};

			pub struct Request {
				pub room_id: OwnedRoomId,
				pub limit: UInt,
				pub min_depth: UInt,
				pub earliest_events: Vec<OwnedEventId>,
				pub latest_events: Vec<OwnedEventId>,
			}
			impl ::core::fmt::Debug for Request {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Request")
				}
			}
			const _: crate::endpoint::Metadata =
				<Request as crate::endpoint::EndpointRequest>::METADATA;
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/federation/v1/get_missing_events/{room_id}",
				path { room_id }
				query {}
				body { limit => "limit", min_depth => "min_depth", earliest_events => "earliest_events", latest_events => "latest_events" }
			}
			pub struct Response {
				pub events: Vec<RawPdu>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { events => "events" }
			}
		}
	}

	pub mod get_room_state_ids {
		pub mod v1 {
			use crate::{OwnedEventId, OwnedRoomId};

			pub struct Request {
				pub room_id: OwnedRoomId,
				pub event_id: OwnedEventId,
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
				path: "/_matrix/federation/v1/state_ids/{room_id}",
				path { room_id }
				query { event_id => "event_id" }
				body {}
			}
			pub struct Response {
				pub auth_chain_ids: Vec<OwnedEventId>,
				pub pdu_ids: Vec<OwnedEventId>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { auth_chain_ids => "auth_chain_ids", pdu_ids => "pdu_ids" }
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
pub mod reconciliation;
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
		let request = get_event::v1::Request::new(
			OwnedEventId::parse("$ev:example.org").unwrap(),
			Some(false),
		);
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
			room_id: OwnedRoomId::parse("!room:example.org").unwrap(),
			limit: 50,
			min_depth: 0,
			earliest_events: alloc::vec![OwnedEventId::parse("$a").unwrap()],
			latest_events: alloc::vec![OwnedEventId::parse("$b").unwrap()],
		};
		let http = request
			.try_into_http_request::<Vec<u8>>("https://example.org", SendAccessToken::None, &[])
			.unwrap();
		assert_eq!(http.method(), http::Method::POST);
		let parsed =
			get_missing_events::v1::Request::try_from_http_request(http, &["!room:example.org"])
				.unwrap();
		assert_eq!(parsed.limit, 50);
		assert_eq!(parsed.latest_events, alloc::vec![OwnedEventId::parse("$b").unwrap()]);
		assert_eq!(parsed.room_id.as_str(), "!room:example.org");
	}

	#[test]
	fn federation_request_rejects_duplicate_keys() {
		use super::transactions::send_transaction_message::v1::Request;

		let body =
			r#"{"origin":"example.org","origin_server_ts":1,"pdus":[{"a":1,"a":2}],"edus":[]}"#;
		let http = http::Request::builder()
			.method("PUT")
			.uri("/_matrix/federation/v1/send/txn1")
			.body(body.as_bytes().to_vec())
			.unwrap();
		assert!(Request::try_from_http_request(http, &["txn1"]).is_err());
	}

	#[test]
	fn get_event_response_decodes_and_encodes() {
		let body =
			r#"{"origin":"example.org","origin_server_ts":5,"pdus":[{"type":"m.room.message"}]}"#;
		let response =
			http::Response::builder().status(200).body(body.as_bytes().to_vec()).unwrap();
		let decoded = get_event::v1::Response::try_from_http_response(response).unwrap();
		assert_eq!(decoded.origin.as_str(), "example.org");
		assert_eq!(decoded.origin_server_ts, MilliSecondsSinceUnixEpoch(5));
		assert!(decoded.pdus[0].get().contains("m.room.message"));
		let again = decoded.try_into_http_response::<Vec<u8>>().unwrap();
		assert_eq!(again.status(), http::StatusCode::OK);
	}

	#[test]
	fn key_query_response_requires_device_keys() {
		use super::keys::get_keys::v1::Response;
		let body = crate::json::Value::parse(
			r#"{"master_keys":{},"self_signing_keys":{},"failures":{}}"#,
		)
		.unwrap();
		assert!(<Response as crate::endpoint::EndpointResponse>::from_body(&body).is_err());
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

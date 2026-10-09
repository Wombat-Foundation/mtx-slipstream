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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"GET",
					"/_matrix/federation/v1/openid/userinfo",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [(
						"access_token",
						crate::endpoint::enc(&self.access_token),
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
						access_token: input.query("access_token")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub sub: OwnedUserId,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [("sub", crate::endpoint::enc(&self.sub))])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						sub: _input.body("sub")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata =
					crate::endpoint::Metadata::new("GET", "/_matrix/federation/v1/edutypes");
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
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
					let value = Self {};
					input.finish()?;
					Ok(value)
				}
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
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("typing", crate::endpoint::enc(&self.typing)),
						("presence", crate::endpoint::enc(&self.presence)),
						("receipt", crate::endpoint::enc(&self.receipt)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						typing: _input.body("typing")?,
						presence: _input.body("presence")?,
						receipt: _input.body("receipt")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"POST",
					"/_matrix/federation/v1/user/keys/query",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("device_keys", crate::endpoint::enc(&self.device_keys))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						device_keys: input.body("device_keys")?,
					};
					input.finish()?;
					Ok(value)
				}
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
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("device_keys", crate::endpoint::enc(&self.device_keys)),
						("master_keys", crate::endpoint::enc(&self.master_keys)),
						("self_signing_keys", crate::endpoint::enc(&self.self_signing_keys)),
						("failures", crate::endpoint::enc(&self.failures)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						device_keys: _input.body("device_keys")?,
						master_keys: _input.body("master_keys")?,
						self_signing_keys: _input.body("self_signing_keys")?,
						failures: _input.body("failures")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"POST",
					"/_matrix/federation/v1/user/keys/claim",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("one_time_keys", crate::endpoint::enc(&self.one_time_keys))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						one_time_keys: input.body("one_time_keys")?,
					};
					input.finish()?;
					Ok(value)
				}
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
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("one_time_keys", crate::endpoint::enc(&self.one_time_keys)),
						("failures", crate::endpoint::enc(&self.failures)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						one_time_keys: _input.body("one_time_keys")?,
						failures: _input.body("failures")?,
					})
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"POST",
					"/_matrix/policy/unstable/org.matrix.msc4284/event/{event_id}/check",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.event_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("pdu", crate::endpoint::enc(&self.pdu))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						event_id: input.path()?,
						pdu: input.body("pdu")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub recommendation: String,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [(
						"recommendation",
						crate::endpoint::enc(&self.recommendation),
					)])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						recommendation: _input.body("recommendation")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"POST",
					"/_matrix/policy/unstable/org.matrix.msc4284/sign",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("pdu", crate::endpoint::enc(&self.pdu))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						pdu: input.body("pdu")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub signatures: Option<Signatures>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [(
						"signatures",
						crate::endpoint::enc(&self.signatures),
					)])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						signatures: _input.body("signatures")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"GET",
					"/_matrix/federation/v1/event/{event_id}",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.event_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [(
						"include_unredacted_content",
						crate::endpoint::enc(&self.include_unredacted_content),
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
						event_id: input.path()?,
						include_unredacted_content: input.query("include_unredacted_content")?,
					};
					input.finish()?;
					Ok(value)
				}
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
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("origin", crate::endpoint::enc(&self.origin)),
						("origin_server_ts", crate::endpoint::enc(&self.origin_server_ts)),
						("pdus", crate::endpoint::enc(&self.pdus)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						origin: _input.body("origin")?,
						origin_server_ts: _input.body("origin_server_ts")?,
						pdus: _input.body("pdus")?,
					})
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

			use crate::{OwnedEventId, OwnedRoomId, federation_api::RawPdu};

			pub struct Request {
				pub event_id: OwnedEventId,
				pub room_id: Option<OwnedRoomId>,
				pub max_depth: Option<i64>,
				pub max_breadth: Option<i64>,
				pub limit: Option<i64>,
				pub depth_first: Option<bool>,
				pub recent_first: Option<bool>,
				pub include_parent: Option<bool>,
				pub include_children: Option<bool>,
				pub direction: Option<String>,
				pub batch: Option<String>,
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
					"POST",
					"/_matrix/federation/unstable/event_relationships",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("event_id", crate::endpoint::enc(&self.event_id)),
							("room_id", crate::endpoint::enc(&self.room_id)),
							("max_depth", crate::endpoint::enc(&self.max_depth)),
							("max_breadth", crate::endpoint::enc(&self.max_breadth)),
							("limit", crate::endpoint::enc(&self.limit)),
							("depth_first", crate::endpoint::enc(&self.depth_first)),
							("recent_first", crate::endpoint::enc(&self.recent_first)),
							("include_parent", crate::endpoint::enc(&self.include_parent)),
							("include_children", crate::endpoint::enc(&self.include_children)),
							("direction", crate::endpoint::enc(&self.direction)),
							("batch", crate::endpoint::enc(&self.batch)),
						],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						event_id: input.body("event_id")?,
						room_id: input.body("room_id")?,
						max_depth: input.body("max_depth")?,
						max_breadth: input.body("max_breadth")?,
						limit: input.body("limit")?,
						depth_first: input.body("depth_first")?,
						recent_first: input.body("recent_first")?,
						include_parent: input.body("include_parent")?,
						include_children: input.body("include_children")?,
						direction: input.body("direction")?,
						batch: input.body("batch")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
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
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("events", crate::endpoint::enc(&self.events)),
						("next_batch", crate::endpoint::enc(&self.next_batch)),
						("limited", crate::endpoint::enc(&self.limited)),
						("auth_chain", crate::endpoint::enc(&self.auth_chain)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						events: _input.body("events")?,
						next_batch: _input.body("next_batch")?,
						limited: _input.body("limited")?,
						auth_chain: _input.body("auth_chain")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"GET",
					"/_matrix/federation/v1/timestamp_to_event/{room_id}",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.room_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [
						("dir", crate::endpoint::enc(&self.dir)),
						("ts", crate::endpoint::enc(&self.ts)),
					])
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
						dir: input.query("dir")?,
						ts: input.query("ts")?,
					};
					input.finish()?;
					Ok(value)
				}
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
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("event_id", crate::endpoint::enc(&self.event_id)),
						("origin_server_ts", crate::endpoint::enc(&self.origin_server_ts)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						event_id: _input.body("event_id")?,
						origin_server_ts: _input.body("origin_server_ts")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"POST",
					"/_matrix/federation/v1/get_missing_events/{room_id}",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.room_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("limit", crate::endpoint::enc(&self.limit)),
							("min_depth", crate::endpoint::enc(&self.min_depth)),
							("earliest_events", crate::endpoint::enc(&self.earliest_events)),
							("latest_events", crate::endpoint::enc(&self.latest_events)),
						],
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
						limit: input.body("limit")?,
						min_depth: input.body("min_depth")?,
						earliest_events: input.body("earliest_events")?,
						latest_events: input.body("latest_events")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub events: Vec<RawPdu>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [(
						"events",
						crate::endpoint::enc(&self.events),
					)])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						events: _input.body("events")?,
					})
				}
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
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"GET",
					"/_matrix/federation/v1/state_ids/{room_id}",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.room_id,
					)])
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
				pub auth_chain_ids: Vec<OwnedEventId>,
				pub pdu_ids: Vec<OwnedEventId>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("auth_chain_ids", crate::endpoint::enc(&self.auth_chain_ids)),
						("pdu_ids", crate::endpoint::enc(&self.pdu_ids)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						auth_chain_ids: _input.body("auth_chain_ids")?,
						pdu_ids: _input.body("pdu_ids")?,
					})
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

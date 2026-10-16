//! Federation endpoints for MSC4521 algebraic set reconciliation and the
//! MSC4500 state accumulator.
//!
//! Both are still unstable, so they live under `/_matrix/federation/unstable`.
//! The reconciliation endpoints use the `tk.nutra.msc0501` prefix and the state
//! accumulator the `tk.nutra.msc4500` prefix.

use alloc::{string::String, vec::Vec};

use crate::{OwnedEventId, OwnedRoomId};

/// `GET .../room_digest/{roomId}`: the responder's level-0 digest and frame.
pub mod get_room_digest {
	pub mod unstable {
		use super::super::{OwnedEventId, OwnedRoomId, String, Vec};

		pub struct Request {
			pub room_id: OwnedRoomId,
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
				"/_matrix/federation/unstable/tk.nutra.msc0501/room_digest/{room_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
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
				let value = Self {
					room_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}

		pub struct Response {
			pub digest: String,
			pub digest_type: String,
			pub known_event_count: u64,
			pub frame_id: String,
			pub frame_event_ids: Vec<OwnedEventId>,
			pub extremity_event_ids: Vec<OwnedEventId>,
			pub depth_range: Vec<u64>,
			pub origin_server_ts_range: Vec<u64>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("digest", crate::endpoint::enc(&self.digest)),
					("digest_type", crate::endpoint::enc(&self.digest_type)),
					("known_event_count", crate::endpoint::enc(&self.known_event_count)),
					("frame_id", crate::endpoint::enc(&self.frame_id)),
					("frame_event_ids", crate::endpoint::enc(&self.frame_event_ids)),
					("extremity_event_ids", crate::endpoint::enc(&self.extremity_event_ids)),
					("depth_range", crate::endpoint::enc(&self.depth_range)),
					(
						"origin_server_ts_range",
						crate::endpoint::enc(&self.origin_server_ts_range),
					),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					digest: input.body("digest")?,
					digest_type: input.body("digest_type")?,
					known_event_count: input.body("known_event_count")?,
					frame_id: input.body("frame_id")?,
					frame_event_ids: input.body_or_default("frame_event_ids")?,
					extremity_event_ids: input.body_or_default("extremity_event_ids")?,
					depth_range: input.body_or_default("depth_range")?,
					origin_server_ts_range: input.body_or_default("origin_server_ts_range")?,
				})
			}
		}
	}
}

/// `GET .../room_digest/{roomId}/strata`: the digest plus the 32 strata.
pub mod get_room_digest_strata {
	pub mod unstable {
		use super::super::{OwnedEventId, OwnedRoomId, String, Vec};

		pub struct Request {
			pub room_id: OwnedRoomId,
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
				"/_matrix/federation/unstable/tk.nutra.msc0501/room_digest/{room_id}/strata",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
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
				let value = Self {
					room_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}

		pub struct Response {
			pub digest: String,
			pub digest_type: String,
			pub known_event_count: u64,
			pub frame_id: String,
			pub frame_event_ids: Vec<OwnedEventId>,
			pub extremity_event_ids: Vec<OwnedEventId>,
			pub depth_range: Vec<u64>,
			pub origin_server_ts_range: Vec<u64>,
			/// One base64url-encoded stratum per entry; 32 of them.
			pub strata: Vec<String>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("digest", crate::endpoint::enc(&self.digest)),
					("digest_type", crate::endpoint::enc(&self.digest_type)),
					("known_event_count", crate::endpoint::enc(&self.known_event_count)),
					("frame_id", crate::endpoint::enc(&self.frame_id)),
					("frame_event_ids", crate::endpoint::enc(&self.frame_event_ids)),
					("extremity_event_ids", crate::endpoint::enc(&self.extremity_event_ids)),
					("depth_range", crate::endpoint::enc(&self.depth_range)),
					(
						"origin_server_ts_range",
						crate::endpoint::enc(&self.origin_server_ts_range),
					),
					("strata", crate::endpoint::enc(&self.strata)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					digest: input.body("digest")?,
					digest_type: input.body("digest_type")?,
					known_event_count: input.body("known_event_count")?,
					frame_id: input.body("frame_id")?,
					frame_event_ids: input.body_or_default("frame_event_ids")?,
					extremity_event_ids: input.body_or_default("extremity_event_ids")?,
					depth_range: input.body_or_default("depth_range")?,
					origin_server_ts_range: input.body_or_default("origin_server_ts_range")?,
					strata: input.body("strata")?,
				})
			}
		}
	}
}

/// `POST .../room_diff/{roomId}`: sketch-mode reconciliation.
pub mod post_room_diff {
	pub mod unstable {
		use super::super::{OwnedEventId, OwnedRoomId, String, Vec};

		/// One localized sketch request.
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub struct SketchBucket {
			pub depth: u64,
			pub prefix: u64,
			pub capacity: u64,
		}

		impl crate::codec::Serialize for SketchBucket {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("depth", crate::endpoint::enc(&self.depth)),
					("prefix", crate::endpoint::enc(&self.prefix)),
					("capacity", crate::endpoint::enc(&self.capacity)),
				])
			}
		}
		impl crate::codec::Deserialize for SketchBucket {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected("SketchBucket"));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					depth: input.body("depth")?,
					prefix: input.body("prefix")?,
					capacity: input.body("capacity")?,
				})
			}
		}

		pub struct Request {
			pub room_id: OwnedRoomId,
			/// Must be `sketch`.
			pub mode: String,
			/// Must be `algebraic_v1`.
			pub digest_type: String,
			pub frame_id: String,
			pub frame_negotiation: bool,
			pub frame_event_ids: Vec<OwnedEventId>,
			pub local_digest: String,
			pub local_known_event_count: u64,
			pub requests: Vec<SketchBucket>,
			pub local_sketches: Vec<String>,
			/// `event_set` (the default) or `resolved_state`.
			pub scope: Option<String>,
			pub state_at: Option<OwnedEventId>,
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
				"/_matrix/federation/unstable/tk.nutra.msc0501/room_diff/{room_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("mode", crate::endpoint::enc(&self.mode)),
						("digest_type", crate::endpoint::enc(&self.digest_type)),
						("frame_id", crate::endpoint::enc(&self.frame_id)),
						("frame_negotiation", crate::endpoint::enc(&self.frame_negotiation)),
						("frame_event_ids", crate::endpoint::enc(&self.frame_event_ids)),
						("local_digest", crate::endpoint::enc(&self.local_digest)),
						(
							"local_known_event_count",
							crate::endpoint::enc(&self.local_known_event_count),
						),
						("requests", crate::endpoint::enc(&self.requests)),
						("local_sketches", crate::endpoint::enc(&self.local_sketches)),
						("scope", crate::endpoint::enc(&self.scope)),
						("state_at", crate::endpoint::enc(&self.state_at)),
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
					mode: input.body("mode")?,
					digest_type: input.body("digest_type")?,
					frame_id: input.body("frame_id")?,
					frame_negotiation: input.body_or_default("frame_negotiation")?,
					frame_event_ids: input.body_or_default("frame_event_ids")?,
					local_digest: input.body("local_digest")?,
					local_known_event_count: input.body("local_known_event_count")?,
					requests: input.body("requests")?,
					local_sketches: input.body("local_sketches")?,
					scope: input.body("scope")?,
					state_at: input.body("state_at")?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		pub struct Response {
			pub digest_type: String,
			pub missing_event_ids: Vec<OwnedEventId>,
			pub requester_only_short_ids: Option<Vec<String>>,
			pub expected_requester_side_accumulator: Option<String>,
			pub remote_known_event_count: u64,
			pub remote_extremity_event_ids: Vec<OwnedEventId>,
			pub frame_id: String,
			pub frame_status: String,
			pub negotiated_frame_event_ids: Option<Vec<OwnedEventId>>,
			pub scope: String,
			pub state_at: Option<OwnedEventId>,
			/// `decoded`, `capacity_exceeded` or `not_applicable`.
			pub sketch_status: String,
			pub truncated: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("digest_type", crate::endpoint::enc(&self.digest_type)),
					("missing_event_ids", crate::endpoint::enc(&self.missing_event_ids)),
					(
						"requester_only_short_ids",
						crate::endpoint::enc(&self.requester_only_short_ids),
					),
					(
						"expected_requester_side_accumulator",
						crate::endpoint::enc(&self.expected_requester_side_accumulator),
					),
					(
						"remote_known_event_count",
						crate::endpoint::enc(&self.remote_known_event_count),
					),
					(
						"remote_extremity_event_ids",
						crate::endpoint::enc(&self.remote_extremity_event_ids),
					),
					("frame_id", crate::endpoint::enc(&self.frame_id)),
					("frame_status", crate::endpoint::enc(&self.frame_status)),
					(
						"negotiated_frame_event_ids",
						crate::endpoint::enc(&self.negotiated_frame_event_ids),
					),
					("scope", crate::endpoint::enc(&self.scope)),
					("state_at", crate::endpoint::enc(&self.state_at)),
					("sketch_status", crate::endpoint::enc(&self.sketch_status)),
					("truncated", crate::endpoint::enc(&self.truncated)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					digest_type: input.body("digest_type")?,
					missing_event_ids: input.body_or_default("missing_event_ids")?,
					requester_only_short_ids: input.body("requester_only_short_ids")?,
					expected_requester_side_accumulator: input
						.body("expected_requester_side_accumulator")?,
					remote_known_event_count: input.body("remote_known_event_count")?,
					remote_extremity_event_ids: input
						.body_or_default("remote_extremity_event_ids")?,
					frame_id: input.body("frame_id")?,
					frame_status: input.body("frame_status")?,
					negotiated_frame_event_ids: input.body("negotiated_frame_event_ids")?,
					scope: input.body("scope")?,
					state_at: input.body("state_at")?,
					sketch_status: input.body("sketch_status")?,
					truncated: input.body("truncated")?,
				})
			}
		}
	}
}

/// `GET .../state_accumulator/{roomId}?event_id=`: MSC4500 state accumulator.
pub mod get_state_accumulator {
	pub mod unstable {
		use super::super::{OwnedEventId, OwnedRoomId, String};

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
				"/_matrix/federation/unstable/tk.nutra.msc4500/state_accumulator/{room_id}",
			);
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

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId) -> Self {
				Self {
					room_id,
					event_id,
				}
			}
		}

		pub struct Response {
			pub event_id: OwnedEventId,
			pub algorithm: String,
			/// The serialized LtHash lattice, base64url.
			pub lattice: String,
			pub n_state_events: u64,
			pub digest: String,
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
					("algorithm", crate::endpoint::enc(&self.algorithm)),
					("lattice", crate::endpoint::enc(&self.lattice)),
					("n_state_events", crate::endpoint::enc(&self.n_state_events)),
					("digest", crate::endpoint::enc(&self.digest)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					event_id: input.body("event_id")?,
					algorithm: input.body("algorithm")?,
					lattice: input.body("lattice")?,
					n_state_events: input.body("n_state_events")?,
					digest: input.body("digest")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::{string::ToString, vec};

	use super::{
		get_room_digest, get_room_digest_strata, get_state_accumulator,
		post_room_diff::unstable::{Request as DiffRequest, SketchBucket},
	};
	use crate::{OwnedEventId, OwnedRoomId, endpoint::EndpointRequest};

	fn room() -> OwnedRoomId {
		OwnedRoomId::parse("!room:example.org").unwrap()
	}

	#[test]
	fn paths_are_distinct_and_unstable() {
		let digest = get_room_digest::unstable::Request::METADATA;
		let strata = get_room_digest_strata::unstable::Request::METADATA;
		assert_ne!(digest.path, strata.path);
		assert!(strata.path.ends_with("/strata"));
		assert!(digest.path.starts_with("/_matrix/federation/unstable/"));
	}

	#[test]
	fn room_diff_request_round_trips() {
		let request = DiffRequest {
			room_id: room(),
			mode: "sketch".to_string(),
			digest_type: "algebraic_v1".to_string(),
			frame_id: "frame".to_string(),
			frame_negotiation: true,
			frame_event_ids: vec![OwnedEventId::parse("$a").unwrap()],
			local_digest: "digest".to_string(),
			local_known_event_count: 7,
			requests: vec![SketchBucket {
				depth: 0,
				prefix: 0,
				capacity: 2,
			}],
			local_sketches: vec!["sketch".to_string()],
			scope: None,
			state_at: None,
		};
		let body = request.body();
		let parsed =
			DiffRequest::from_parts(&request.path_args(), &request.query(), body.as_ref())
				.unwrap();
		assert_eq!(parsed.room_id, request.room_id);
		assert_eq!(parsed.mode, "sketch");
		assert!(parsed.frame_negotiation);
		assert_eq!(parsed.frame_event_ids, request.frame_event_ids);
		assert_eq!(parsed.local_known_event_count, 7);
		assert_eq!(parsed.requests, request.requests);
		assert_eq!(parsed.scope, None);
	}

	#[test]
	fn room_diff_request_defaults_optional_fields() {
		let body = crate::json::Value::parse(
			r#"{"mode":"sketch","digest_type":"algebraic_v1","frame_id":"f",
			"local_digest":"d","local_known_event_count":1,"requests":[],"local_sketches":[]}"#,
		)
		.unwrap();
		let path = vec![room().to_string()];
		let parsed = DiffRequest::from_parts(&path, &[], Some(&body)).unwrap();
		assert!(!parsed.frame_negotiation);
		assert!(parsed.frame_event_ids.is_empty());
		assert_eq!(parsed.scope, None);
	}

	#[test]
	fn room_diff_request_rejects_missing_required_field() {
		let body = crate::json::Value::parse(r#"{"mode":"sketch"}"#).unwrap();
		let path = vec![room().to_string()];
		assert!(DiffRequest::from_parts(&path, &[], Some(&body)).is_err());
	}

	#[test]
	fn state_accumulator_takes_event_id_from_query() {
		let request = get_state_accumulator::unstable::Request::new(
			room(),
			OwnedEventId::parse("$e").unwrap(),
		);
		let parsed = get_state_accumulator::unstable::Request::from_parts(
			&request.path_args(),
			&request.query(),
			None,
		)
		.unwrap();
		assert_eq!(parsed.event_id, request.event_id);
	}
}

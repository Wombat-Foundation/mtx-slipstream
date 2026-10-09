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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/federation/v1/backfill/{room_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("v", crate::endpoint::enc(&self.v)),
					("limit", crate::endpoint::enc(&self.limit)),
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
					v: input.query("v")?,
					limit: input.query("limit")?,
				};
				input.finish()?;
				Ok(value)
			}
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
	}
}

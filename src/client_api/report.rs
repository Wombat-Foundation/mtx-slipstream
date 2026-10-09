pub mod report_user {
	pub mod v3 {
		use crate::OwnedUserId;
		pub struct Request {
			pub user_id: OwnedUserId,
			pub reason: Option<String>,
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
				"/_matrix/client/v3/users/{userId}/report",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.user_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("reason", crate::endpoint::enc(&self.reason))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					user_id: input.path()?,
					reason: input.body("reason")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {})
			}
		}
	}
}
pub mod room {
	pub mod report_room {
		pub mod v3 {
			use crate::OwnedRoomId;
			pub struct Request {
				pub room_id: OwnedRoomId,
				pub reason: Option<String>,
				pub score: Option<crate::Int>,
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
					"/_matrix/client/v3/rooms/{roomId}/report",
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
							("reason", crate::endpoint::enc(&self.reason)),
							("score", crate::endpoint::enc(&self.score)),
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
						reason: input.body("reason")?,
						score: input.body("score")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {})
				}
			}
		}
	}
	pub mod report_content {
		pub mod v3 {
			use crate::{OwnedEventId, OwnedRoomId};
			pub struct Request {
				pub room_id: OwnedRoomId,
				pub event_id: OwnedEventId,
				pub reason: Option<String>,
				pub score: Option<crate::Int>,
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
					"/_matrix/client/v3/rooms/{roomId}/report/{eventId}",
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
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("reason", crate::endpoint::enc(&self.reason)),
							("score", crate::endpoint::enc(&self.score)),
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
						event_id: input.path()?,
						reason: input.body("reason")?,
						score: input.body("score")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {})
				}
			}
		}
	}
}

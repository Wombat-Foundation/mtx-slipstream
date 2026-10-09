//! Requests to external antispam services (Meowlnir and Draupnir).

/// Meowlnir's antispam API, scoped to a management room.
pub mod meowlnir {
	pub mod user_may_invite {
		pub mod v1 {
			use crate::{OwnedRoomId, OwnedUserId};

			pub struct Request {
				pub management_room: OwnedRoomId,
				pub inviter: OwnedUserId,
				pub invitee: OwnedUserId,
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
					"POST",
					"/_meowlnir/antispam/{management_room}/user_may_invite",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.management_room,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("inviter", crate::endpoint::enc(&self.inviter)),
							("invitee", crate::endpoint::enc(&self.invitee)),
							("room_id", crate::endpoint::enc(&self.room_id)),
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
						management_room: input.path()?,
						inviter: input.body("inviter")?,
						invitee: input.body("invitee")?,
						room_id: input.body("room_id")?,
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

			impl Request {
				#[must_use]
				pub fn new(
					management_room: OwnedRoomId,
					inviter: OwnedUserId,
					invitee: OwnedUserId,
					room_id: OwnedRoomId,
				) -> Self {
					Self {
						management_room,
						inviter,
						invitee,
						room_id,
					}
				}
			}
		}
	}

	pub mod user_may_join_room {
		pub mod v1 {
			use crate::{OwnedRoomId, OwnedUserId};

			pub struct Request {
				pub management_room: OwnedRoomId,
				pub user_id: OwnedUserId,
				pub room_id: OwnedRoomId,
				pub is_invited: bool,
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
					"/_meowlnir/antispam/{management_room}/user_may_join_room",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.management_room,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("user_id", crate::endpoint::enc(&self.user_id)),
							("room_id", crate::endpoint::enc(&self.room_id)),
							("is_invited", crate::endpoint::enc(&self.is_invited)),
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
						management_room: input.path()?,
						user_id: input.body("user_id")?,
						room_id: input.body("room_id")?,
						is_invited: input.body("is_invited")?,
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

			impl Request {
				#[must_use]
				pub fn new(
					management_room: OwnedRoomId,
					user_id: OwnedUserId,
					room_id: OwnedRoomId,
					is_invited: bool,
				) -> Self {
					Self {
						management_room,
						user_id,
						room_id,
						is_invited,
					}
				}
			}
		}
	}

	pub mod accept_make_join {
		pub mod v1 {
			use crate::{OwnedRoomId, OwnedUserId};

			pub struct Request {
				pub management_room: OwnedRoomId,
				pub user_id: OwnedUserId,
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
					"POST",
					"/_meowlnir/antispam/{management_room}/accept_make_join",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.management_room,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("user_id", crate::endpoint::enc(&self.user_id)),
							("room_id", crate::endpoint::enc(&self.room_id)),
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
						management_room: input.path()?,
						user_id: input.body("user_id")?,
						room_id: input.body("room_id")?,
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

			impl Request {
				#[must_use]
				pub fn new(
					management_room: OwnedRoomId,
					user_id: OwnedUserId,
					room_id: OwnedRoomId,
				) -> Self {
					Self {
						management_room,
						user_id,
						room_id,
					}
				}
			}
		}
	}
}

/// Draupnir's antispam API (`synapse-http-antispam`).
pub mod draupnir {
	pub mod user_may_invite {
		pub mod v1 {
			use crate::{OwnedRoomId, OwnedUserId};

			pub struct Request {
				pub room_id: OwnedRoomId,
				pub inviter: OwnedUserId,
				pub invitee: OwnedUserId,
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
				const METADATA: crate::endpoint::Metadata =
					crate::endpoint::Metadata::new("POST", "/api/1/spam_check/user_may_invite");
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
							("room_id", crate::endpoint::enc(&self.room_id)),
							("inviter", crate::endpoint::enc(&self.inviter)),
							("invitee", crate::endpoint::enc(&self.invitee)),
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
						room_id: input.body("room_id")?,
						inviter: input.body("inviter")?,
						invitee: input.body("invitee")?,
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

			impl Request {
				#[must_use]
				pub fn new(
					room_id: OwnedRoomId,
					inviter: OwnedUserId,
					invitee: OwnedUserId,
				) -> Self {
					Self {
						room_id,
						inviter,
						invitee,
					}
				}
			}
		}
	}

	pub mod user_may_join_room {
		pub mod v1 {
			use crate::{OwnedRoomId, OwnedUserId};

			pub struct Request {
				pub user_id: OwnedUserId,
				pub room_id: OwnedRoomId,
				pub is_invited: bool,
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
					"/api/1/spam_check/user_may_join_room",
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
							("user_id", crate::endpoint::enc(&self.user_id)),
							("room_id", crate::endpoint::enc(&self.room_id)),
							("is_invited", crate::endpoint::enc(&self.is_invited)),
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
						user_id: input.body("user_id")?,
						room_id: input.body("room_id")?,
						is_invited: input.body("is_invited")?,
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

			impl Request {
				#[must_use]
				pub fn new(user_id: OwnedUserId, room_id: OwnedRoomId, is_invited: bool) -> Self {
					Self {
						user_id,
						room_id,
						is_invited,
					}
				}
			}
		}
	}
}

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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_meowlnir/antispam/{management_room}/user_may_invite",
				path { management_room }
				query {}
				body { inviter => "inviter", invitee => "invitee", room_id => "room_id" }
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body {}
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_meowlnir/antispam/{management_room}/user_may_join_room",
				path { management_room }
				query {}
				body { user_id => "user_id", room_id => "room_id", is_invited => "is_invited" }
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body {}
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_meowlnir/antispam/{management_room}/accept_make_join",
				path { management_room }
				query {}
				body { user_id => "user_id", room_id => "room_id" }
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body {}
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/api/1/spam_check/user_may_invite",
				path {}
				query {}
				body { room_id => "room_id", inviter => "inviter", invitee => "invitee" }
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body {}
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/api/1/spam_check/user_may_join_room",
				path {}
				query {}
				body { user_id => "user_id", room_id => "room_id", is_invited => "is_invited" }
			}
			pub struct Response {}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body {}
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

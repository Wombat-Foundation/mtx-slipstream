//! Requests to external antispam services (Meowlnir and Draupnir).

/// Meowlnir's antispam API, scoped to a management room.
pub mod meowlnir {
	pub mod user_may_invite {
		pub mod v1 {
			use crate::{OwnedRoomId, OwnedUserId, endpoint};

			endpoint! {
				method: "POST", path: "/_meowlnir/antispam/{management_room}/user_may_invite",
				request {
					path { management_room: OwnedRoomId }
					query {}
					body { inviter: OwnedUserId, invitee: OwnedUserId, room_id: OwnedRoomId }
				}
				response {}
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
			use crate::{OwnedRoomId, OwnedUserId, endpoint};

			endpoint! {
				method: "POST", path: "/_meowlnir/antispam/{management_room}/user_may_join_room",
				request {
					path { management_room: OwnedRoomId }
					query {}
					body { user_id: OwnedUserId, room_id: OwnedRoomId, is_invited: bool }
				}
				response {}
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
			use crate::{OwnedRoomId, OwnedUserId, endpoint};

			endpoint! {
				method: "POST", path: "/_meowlnir/antispam/{management_room}/accept_make_join",
				request {
					path { management_room: OwnedRoomId }
					query {}
					body { user_id: OwnedUserId, room_id: OwnedRoomId }
				}
				response {}
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
			use crate::{OwnedRoomId, OwnedUserId, endpoint};

			endpoint! {
				method: "POST", path: "/api/1/spam_check/user_may_invite",
				request {
					path {}
					query {}
					body { room_id: OwnedRoomId, inviter: OwnedUserId, invitee: OwnedUserId }
				}
				response {}
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
			use crate::{OwnedRoomId, OwnedUserId, endpoint};

			endpoint! {
				method: "POST", path: "/api/1/spam_check/user_may_join_room",
				request {
					path {}
					query {}
					body { user_id: OwnedUserId, room_id: OwnedRoomId, is_invited: bool }
				}
				response {}
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

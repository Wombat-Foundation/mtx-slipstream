pub mod get_presence {
	pub mod v3 {
		use crate::{OwnedUserId, events::presence::PresenceState};
		pub struct Request {
			pub user_id: OwnedUserId,
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
			path: "/_matrix/client/v3/presence/{userId}/status",
			path { user_id }
			query {}
			body {}
		}
		pub struct Response {
			pub presence: PresenceState,
			pub last_active_ago: Option<std::time::Duration>,
			pub status_msg: Option<String>,
			pub currently_active: Option<bool>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { presence => "presence", last_active_ago => "last_active_ago", status_msg => "status_msg", currently_active => "currently_active" }
		}
	}
}

pub mod set_presence {
	pub mod v3 {
		use crate::{OwnedUserId, events::presence::PresenceState};
		pub struct Request {
			pub user_id: OwnedUserId,
			pub presence: PresenceState,
			pub status_msg: Option<String>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "PUT",
			path: "/_matrix/client/v3/presence/{userId}/status",
			path { user_id }
			query {}
			body { presence => "presence", status_msg => "status_msg" }
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
	}
}

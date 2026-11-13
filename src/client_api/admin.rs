pub mod get_suspended {
	pub mod v1 {
		use crate::OwnedUserId;
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
			path: "/_matrix/client/unstable/uk.timedout.msc4323/admin/suspend/{userId}",
			path { user_id }
			query {}
			body {}
		}
		pub struct Response {
			pub suspended: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { suspended => "suspended" }
		}
		impl Response {
			#[must_use]
			pub fn new(suspended: bool) -> Self {
				Self {
					suspended,
				}
			}
		}
	}
}
pub mod set_suspended {
	pub mod v1 {
		use crate::OwnedUserId;
		pub struct Request {
			pub user_id: OwnedUserId,
			pub suspended: bool,
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
			path: "/_matrix/client/unstable/uk.timedout.msc4323/admin/suspend/{userId}",
			path { user_id }
			query {}
			body { suspended => "suspended" }
		}
		pub struct Response {
			pub suspended: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { suspended => "suspended" }
		}
		impl Response {
			#[must_use]
			pub fn new(suspended: bool) -> Self {
				Self {
					suspended,
				}
			}
		}
	}
}

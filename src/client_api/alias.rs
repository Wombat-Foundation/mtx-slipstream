pub mod create_alias {
	pub mod v3 {
		use crate::{OwnedRoomAliasId, OwnedRoomId};
		pub struct Request {
			pub room_alias: OwnedRoomAliasId,
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
			method: "PUT",
			path: "/_matrix/client/v3/directory/room/{roomAlias}",
			path { room_alias }
			query {}
			body { room_id => "room_id" }
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
pub mod delete_alias {
	pub mod v3 {
		use crate::OwnedRoomAliasId;
		pub struct Request {
			pub room_alias: OwnedRoomAliasId,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "DELETE",
			path: "/_matrix/client/v3/directory/room/{roomAlias}",
			path { room_alias }
			query {}
			body {}
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
pub mod get_alias {
	pub mod v3 {
		use crate::{OwnedRoomAliasId, OwnedRoomId};
		pub struct Request {
			pub room_alias: OwnedRoomAliasId,
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
			path: "/_matrix/client/v3/directory/room/{roomAlias}",
			path { room_alias }
			query {}
			body {}
		}
		pub struct Response {
			pub room_id: OwnedRoomId,
			pub servers: Vec<String>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { room_id => "room_id", servers => "servers" }
		}
	}
}

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
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/users/{userId}/report",
			path { user_id }
			query {}
			body { reason => "reason" }
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/client/v3/rooms/{roomId}/report",
				path { room_id }
				query {}
				body { reason => "reason", score => "score" }
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/client/v3/rooms/{roomId}/report/{eventId}",
				path { room_id, event_id }
				query {}
				body { reason => "reason", score => "score" }
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
}

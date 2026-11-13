pub mod get_global_account_data {
	pub mod v3 {
		use crate::{OwnedUserId, events::GlobalAccountDataEventType, sswire::Raw};
		pub struct Request {
			pub user_id: OwnedUserId,
			pub event_type: GlobalAccountDataEventType,
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
			path: "/_matrix/client/v3/user/{userId}/account_data/{type}",
			path { user_id, event_type }
			query {}
			body {}
		}
		pub struct Response {
			pub account_data: Raw<crate::events::AnyGlobalAccountDataEvent>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.account_data)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					account_data: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	}
}
pub mod get_room_account_data {
	pub mod v3 {
		use crate::{OwnedRoomId, OwnedUserId, events::RoomAccountDataEventType, sswire::Raw};
		pub struct Request {
			pub user_id: OwnedUserId,
			pub room_id: OwnedRoomId,
			pub event_type: RoomAccountDataEventType,
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
			path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/account_data/{type}",
			path { user_id, room_id, event_type }
			query {}
			body {}
		}
		pub struct Response {
			pub account_data: Raw<crate::events::AnyRoomAccountDataEvent>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.account_data)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					account_data: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	}
}
pub mod set_global_account_data {
	pub mod v3 {
		use crate::{OwnedUserId, events::GlobalAccountDataEventType, json::Value};
		pub struct Request {
			pub user_id: OwnedUserId,
			pub event_type: GlobalAccountDataEventType,
			pub data: Value,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_raw_impl! {
			method: "PUT",
			path: "/_matrix/client/v3/user/{userId}/account_data/{type}",
			path { user_id, event_type }
			query {}
			raw { data }
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
pub mod set_room_account_data {
	pub mod v3 {
		use crate::{OwnedRoomId, OwnedUserId, events::RoomAccountDataEventType, json::Value};
		pub struct Request {
			pub user_id: OwnedUserId,
			pub room_id: OwnedRoomId,
			pub event_type: RoomAccountDataEventType,
			pub data: Value,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_raw_impl! {
			method: "PUT",
			path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/account_data/{type}",
			path { user_id, room_id, event_type }
			query {}
			raw { data }
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

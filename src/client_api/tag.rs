pub mod create_tag {
	pub mod v3 {
		use crate::{OwnedRoomId, events::tag::TagInfo};
		pub struct Request {
			pub user_id: crate::OwnedUserId,
			pub room_id: OwnedRoomId,
			pub tag: String,
			pub tag_info: TagInfo,
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
			path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}",
			path { user_id, room_id, tag }
			query {}
			raw { tag_info }
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
pub mod delete_tag {
	pub mod v3 {
		use crate::OwnedRoomId;
		pub struct Request {
			pub user_id: crate::OwnedUserId,
			pub room_id: OwnedRoomId,
			pub tag: String,
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
			path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}",
			path { user_id, room_id, tag }
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
pub mod get_tags {
	pub mod v3 {
		use crate::{OwnedRoomId, events::tag::TagInfo};
		use std::collections::BTreeMap;
		pub struct Request {
			pub user_id: crate::OwnedUserId,
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
			method: "GET",
			path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags",
			path { user_id, room_id }
			query {}
			body {}
		}
		pub struct Response {
			pub tags: BTreeMap<String, TagInfo>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { tags => "tags" }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::create_tag::v3::Request;
	use crate::{endpoint::EndpointRequest, json::Value};

	#[test]
	fn create_tag_takes_the_whole_body_as_tag_info() {
		let path = [
			"@alice:example.org".to_owned(),
			"!room:example.org".to_owned(),
			"m.favourite".to_owned(),
		];
		let body = Value::parse(r#"{"order":0.5}"#).unwrap();
		let request = Request::from_parts(&path, &[], Some(&body)).unwrap();
		assert_eq!(request.tag, "m.favourite");

		let empty = Value::parse("{}").unwrap();
		assert!(Request::from_parts(&path, &[], Some(&empty)).is_ok());
	}
}

pub mod create_tag {
	pub mod v3 {
		use crate::{OwnedRoomId, events::tag::TagInfo};
		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}",
			request {
				path { user_id: crate::OwnedUserId, room_id: OwnedRoomId, tag: String }
				query {}
				raw_body { tag_info: TagInfo }
			}
		}
		crate::endpoint_response! { response {} }
	}
}
pub mod delete_tag {
	pub mod v3 {
		use crate::{OwnedRoomId, endpoint};
		endpoint! { method: "DELETE", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}", request { path { user_id: crate::OwnedUserId, room_id: OwnedRoomId, tag: String } query {} body {} } response {} }
	}
}
pub mod get_tags {
	pub mod v3 {
		use crate::{OwnedRoomId, endpoint, events::tag::TagInfo};
		use std::collections::BTreeMap;
		endpoint! { method: "GET", path: "/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags", request { path { user_id: crate::OwnedUserId, room_id: OwnedRoomId } query {} body {} } response { tags: BTreeMap<String, TagInfo> } }
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

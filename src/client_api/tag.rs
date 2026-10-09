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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.user_id),
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.tag),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				Some(crate::endpoint::enc(&self.tag_info))
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					user_id: input.path()?,
					room_id: input.path()?,
					tag: input.path()?,
					tag_info: crate::codec::Deserialize::from_json(
						body.ok_or_else(|| crate::codec::DeError::expected("request body"))?,
					)?,
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"DELETE",
				"/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags/{tag}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.user_id),
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.tag),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					user_id: input.path()?,
					room_id: input.path()?,
					tag: input.path()?,
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/user/{userId}/rooms/{roomId}/tags",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.user_id),
					crate::endpoint::path_param(&self.room_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					user_id: input.path()?,
					room_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub tags: BTreeMap<String, TagInfo>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("tags", crate::endpoint::enc(&self.tags))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					tags: _input.body("tags")?,
				})
			}
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

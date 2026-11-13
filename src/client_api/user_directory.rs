pub mod search_users {
	pub mod v3 {
		use crate::{OwnedMxcUri, UInt};
		#[derive(Debug)]
		pub struct User {
			pub user_id: crate::OwnedUserId,
			pub display_name: Option<String>,
			pub avatar_url: Option<OwnedMxcUri>,
		}
		impl crate::codec::Serialize for User {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
					(stringify!(display_name), crate::endpoint::enc(&self.display_name)),
					(stringify!(avatar_url), crate::endpoint::enc(&self.avatar_url)),
				])
			}
		}
		impl crate::codec::Deserialize for User {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(User)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					user_id: input.body(stringify!(user_id))?,
					display_name: input.body(stringify!(display_name))?,
					avatar_url: input.body(stringify!(avatar_url))?,
				})
			}
		}
		pub struct Request {
			pub search_term: String,
			pub limit: UInt,
			pub language: Option<String>,
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
			path: "/_matrix/client/v3/user_directory/search",
			path {}
			query {}
			body { search_term => "search_term", limit => "limit" = 10, language => "language" }
		}
		pub struct Response {
			pub results: Vec<User>,
			pub limited: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { results => "results", limited => "limited" }
		}
	}
}

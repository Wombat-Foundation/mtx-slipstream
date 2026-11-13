pub mod get_turn_server_info {
	pub mod v3 {
		use crate::UInt;
		pub struct Request {}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/voip/turnServer",
			path {}
			query {}
			body {}
		}
		pub struct Response {
			pub username: String,
			pub password: String,
			pub uris: Vec<String>,
			pub ttl: UInt,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { username => "username", password => "password", uris => "uris", ttl => "ttl" }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::get_turn_server_info::v3::Response;
	use crate::{UInt, endpoint::EndpointResponse};

	#[test]
	fn turn_ttl_is_encoded_as_seconds() {
		let response = Response {
			username: "user".into(),
			password: "password".into(),
			uris: vec!["turn:example.org".into()],
			ttl: UInt::from(86_400_u64),
		};

		assert_eq!(
			response.to_body().get("ttl").and_then(crate::json::Value::as_u64),
			Some(86_400)
		);
	}
}

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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/voip/turnServer");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
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
				let value = Self {};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("username", crate::endpoint::enc(&self.username)),
					("password", crate::endpoint::enc(&self.password)),
					("uris", crate::endpoint::enc(&self.uris)),
					("ttl", crate::endpoint::enc(&self.ttl)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					username: _input.body("username")?,
					password: _input.body("password")?,
					uris: _input.body("uris")?,
					ttl: _input.body("ttl")?,
				})
			}
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

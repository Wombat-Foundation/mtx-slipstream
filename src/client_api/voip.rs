pub mod get_turn_server_info {
	pub mod v3 {
		use crate::{UInt, endpoint};
		endpoint! { method: "GET", path: "/_matrix/client/v3/voip/turnServer", request { path {} query {} body {} } response { username: String, password: String, uris: Vec<String>, ttl: UInt } }
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

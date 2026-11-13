// @generated from matrix-org/matrix-spec `data/api` (client-server and server-server).
// Used only by the cross-check test; placeholders are `{}`.
//
// The entries live in `endpoint_auth_spec.json` so the spec oracle is data, not a
// second Rust table that jscpd sees as duplicated from `endpoint_auth.rs`.
use super::AuthScheme;

const SPEC_AUTH_JSON: &str = include_str!("endpoint_auth_spec.json");

pub(super) fn spec_auth() -> Vec<(String, String, AuthScheme)> {
	let value = crate::json::Value::parse(SPEC_AUTH_JSON).expect("valid spec auth JSON");
	value
		.as_array()
		.expect("spec auth array")
		.iter()
		.map(|entry| {
			let items = entry.as_array().expect("spec auth tuple");
			let method = items[0].as_str().expect("method").to_owned();
			let path = items[1].as_str().expect("path").to_owned();
			let scheme = match items[2].as_str().expect("scheme") {
				"None" => AuthScheme::None,
				"AccessToken" => AuthScheme::AccessToken,
				"AccessTokenOptional" => AuthScheme::AccessTokenOptional,
				"AppserviceToken" => AuthScheme::AppserviceToken,
				"ServerSignatures" => AuthScheme::ServerSignatures,
				other => panic!("unknown auth scheme {other}"),
			};
			(method, path, scheme)
		})
		.collect()
}

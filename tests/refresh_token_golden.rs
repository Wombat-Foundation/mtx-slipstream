//! Golden fixtures for `POST /_matrix/client/v3/refresh` (MSC2918).

use core::time::Duration;

use mtx_slipstream::{
	codec::{from_str, to_string},
	endpoint::{EndpointRequest, EndpointResponse},
	json::Value,
	session::refresh_token,
};

#[test]
fn request_round_trips_the_refresh_token() {
	let request = refresh_token::v3::Request::new("rt_abc".to_owned());
	let body = request.body().expect("refresh has a body");
	assert_eq!(to_string(&body), r#"{"refresh_token":"rt_abc"}"#);

	let parsed =
		refresh_token::v3::Request::from_parts(&[], &[], Some(&body)).expect("request decodes");
	assert_eq!(parsed.refresh_token, "rt_abc");
}

#[test]
fn response_uses_expires_in_ms() {
	let mut response = refresh_token::v3::Response::new("at_new".to_owned());
	response.refresh_token = Some("rt_new".to_owned());
	response.expires_in_ms = Some(Duration::from_millis(300_000));

	let fixture = to_string(&response.to_body());
	assert!(fixture.contains(r#""expires_in_ms":300000"#), "{fixture}");

	let parsed: Value = from_str(&fixture).expect("fixture is JSON");
	let decoded = refresh_token::v3::Response::from_body(&parsed).expect("response decodes");
	assert_eq!(decoded.access_token, "at_new");
	assert_eq!(decoded.refresh_token.as_deref(), Some("rt_new"));
	assert_eq!(decoded.expires_in_ms, Some(Duration::from_millis(300_000)));
}

#[test]
fn response_omits_absent_optionals() {
	let response = refresh_token::v3::Response::new("at_new".to_owned());
	let fixture = to_string(&response.to_body());
	assert!(!fixture.contains("refresh_token"), "{fixture}");
	assert!(!fixture.contains("expires_in_ms"), "{fixture}");
}

//! Golden fixtures for the profile endpoints.
//!
//! The important wire details here are unstable-prefixed or flattened, so a
//! naive endpoint definition would silently emit the wrong JSON:
//!   - `blurhash` is sent as `xyz.amorgan.blurhash` (MSC2448), never `blurhash`.
//!   - `custom_profile_fields` is flattened into the response object rather than
//!     nested under its own key (MSC4133).

use mtx_slipstream::{
	client_api::profile::{get_avatar_url, get_profile},
	codec::{from_str, to_string},
	endpoint::EndpointResponse,
	json::Value,
};

/// Decodes a response body through the endpoint codec.
fn decode<T: EndpointResponse>(fixture: &str) -> T {
	let parsed: Value = from_str(fixture).unwrap_or_else(|e| panic!("{fixture}: {e:?}"));
	T::from_body(&parsed).unwrap_or_else(|e| panic!("{fixture}: {e:?}"))
}

#[test]
fn get_profile_omits_absent_optional_fields() {
	let fixture = r#"{"avatar_url":"mxc://example.org/abc","displayname":"Alice"}"#;

	let response = decode::<get_profile::v3::Response>(fixture);

	assert_eq!(response.avatar_url.as_deref(), Some("mxc://example.org/abc"));
	assert_eq!(response.displayname.as_deref(), Some("Alice"));
	assert!(response.blurhash.is_none());
	assert!(response.custom_profile_fields.is_empty());
	assert_eq!(to_string(&response.to_body()), fixture, "absent fields are omitted");
}

#[test]
fn get_profile_uses_the_unstable_blurhash_prefix() {
	let fixture = r#"{"avatar_url":"mxc://example.org/abc","displayname":"Alice","xyz.amorgan.blurhash":"LEHV6nWB"}"#;

	let response = decode::<get_profile::v3::Response>(fixture);

	assert_eq!(response.blurhash.as_deref(), Some("LEHV6nWB"), "the prefixed key is decoded");
	assert_eq!(
		to_string(&response.to_body()),
		fixture,
		"it must be re-encoded under the prefix, not as `blurhash`"
	);
	assert!(
		!to_string(&response.to_body()).contains("\"blurhash\""),
		"the unprefixed name must not appear"
	);
}

#[test]
fn get_profile_flattens_custom_fields() {
	// MSC4133 fields sit alongside avatar_url/displayname, not nested.
	let fixture = concat!(
		r#"{"avatar_url":"mxc://example.org/abc","favourite_food":"cheese","#,
		r#""m.favourite":true}"#,
	);

	let response = decode::<get_profile::v3::Response>(fixture);

	assert_eq!(
		response.custom_profile_fields.get("favourite_food").and_then(Value::as_str),
		Some("cheese")
	);
	assert_eq!(
		response.custom_profile_fields.get("m.favourite").and_then(Value::as_bool),
		Some(true),
		"boolean custom fields survive"
	);
	assert_eq!(response.custom_profile_fields.len(), 2, "no reserved key leaked in");
	assert_eq!(to_string(&response.to_body()), fixture);
}

#[test]
fn get_profile_custom_field_cannot_shadow_a_reserved_key() {
	// A hostile or buggy client sending `avatar_url` inside the flattened map
	// must not overwrite the real avatar.
	let response = decode::<get_profile::v3::Response>(concat!(
		r#"{"avatar_url":"mxc://example.org/real","displayname":"Alice","#,
		r#""extra":"kept"}"#,
	));

	assert_eq!(response.avatar_url.as_deref(), Some("mxc://example.org/real"));
	assert_eq!(response.custom_profile_fields.get("extra").and_then(Value::as_str), Some("kept"));
	assert!(!response.custom_profile_fields.contains_key("avatar_url"));
}

#[test]
fn get_profile_empty_response_is_an_empty_object() {
	let response = get_profile::v3::Response::new(None, None);

	assert_eq!(to_string(&response.to_body()), "{}");
}

#[test]
fn get_avatar_url_omits_absent_blurhash() {
	let fixture = r#"{"avatar_url":"mxc://example.org/abc"}"#;

	let response = decode::<get_avatar_url::v3::Response>(fixture);

	assert_eq!(response.avatar_url.as_deref(), Some("mxc://example.org/abc"));
	assert_eq!(to_string(&response.to_body()), fixture);
}

#[test]
fn get_avatar_url_uses_the_unstable_blurhash_prefix() {
	let fixture = r#"{"avatar_url":"mxc://example.org/abc","xyz.amorgan.blurhash":"LEHV6nWB"}"#;

	let response = decode::<get_avatar_url::v3::Response>(fixture);

	assert_eq!(response.blurhash.as_deref(), Some("LEHV6nWB"));
	assert_eq!(to_string(&response.to_body()), fixture);
}

#[test]
fn get_avatar_url_new_matches_ruwuma_defaults() {
	let response = get_avatar_url::v3::Response::new(None);

	assert!(response.blurhash.is_none(), "ruwuma's new() leaves blurhash unset");
	assert_eq!(to_string(&response.to_body()), "{}");
}

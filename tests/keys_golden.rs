//! Golden fixtures for client key endpoints.
//!
//! These pin the *wire* shape against what ruma/serde produced, so the codec
//! cannot silently change nesting. The `claim_keys` fixture is the important
//! one: its response is keyed `user -> device -> one-time-key-id`, and dropping
//! the device level would merge keys from different devices of the same user.

use std::collections::BTreeMap;

use mtx_slipstream::{
	client_api::{
		keys::claim_keys,
		profile_keys::{get_profile_key, set_profile_key},
	},
	codec::{from_str, to_string},
	endpoint::{EndpointRequest, EndpointResponse},
	json::Value,
};

/// Decodes a response body then re-encodes it, requiring an exact round trip.
///
/// Responses go through [`EndpointResponse`] in both directions, since the
/// macro generates `from_body`/`to_body` rather than `Deserialize`/`Serialize`.
fn golden<T: EndpointResponse>(fixture: &str) -> T {
	let parsed: Value = from_str(fixture).unwrap_or_else(|e| panic!("{fixture}: {e:?}"));
	let value = T::from_body(&parsed).unwrap_or_else(|e| panic!("{fixture}: {e:?}"));
	assert_eq!(
		to_string(&value.to_body()),
		fixture,
		"re-encoding must reproduce the fixture bytes"
	);
	value
}

#[test]
fn claim_keys_response_keeps_the_device_level() {
	// Device IDs are opaque strings without colons; the algorithm belongs in the
	// innermost key id, not the device level.
	let fixture = r#"{"failures":{},"one_time_keys":{"@alice:example.org":{"DEVICE1":{"curve25519:2":{"key":"second"},"ed25519:1":{"key":"first"}}}}}"#;

	let response: claim_keys::v3::Response = golden(fixture);

	// user -> device -> key id. Three levels, not two.
	let devices = &response.one_time_keys["@alice:example.org"];
	assert_eq!(devices.len(), 1, "one device is present");
	assert_eq!(devices["DEVICE1"].len(), 2, "both keys belong to that device");
}

#[test]
fn claim_keys_response_separates_two_devices_of_one_user() {
	// The regression this file exists for: with only two nesting levels the
	// `ed25519:1` key of both devices would collide.
	let fixture = r#"{"failures":{},"one_time_keys":{"@alice:example.org":{"DEVICE1":{"ed25519:1":{"key":"first"}},"DEVICE2":{"ed25519:1":{"key":"second"}}}}}"#;

	let response: claim_keys::v3::Response = golden(fixture);

	let devices = &response.one_time_keys["@alice:example.org"];
	assert_eq!(devices.len(), 2, "both devices are preserved");
	assert!(devices.contains_key("DEVICE1"));
	assert!(devices.contains_key("DEVICE2"));
}

#[test]
fn claim_keys_response_is_empty_without_failures() {
	let response: claim_keys::v3::Response = golden(r#"{"failures":{},"one_time_keys":{}}"#);

	assert!(response.one_time_keys.is_empty());
	assert!(response.failures.is_empty());
}

#[test]
fn claim_keys_response_keeps_failures() {
	let fixture = r#"{"failures":{"bad:example.org":{"errcode":"M_NOT_FOUND","error":"nope"}},"one_time_keys":{}}"#;

	let response: claim_keys::v3::Response = golden(fixture);

	assert!(response.one_time_keys.is_empty());
	assert_eq!(
		response.failures["bad:example.org"].get("errcode").and_then(Value::as_str),
		Some("M_NOT_FOUND")
	);
}

#[test]
fn claim_keys_new_matches_ruwuma_defaults() {
	let response = claim_keys::v3::Response::new(BTreeMap::new());

	assert!(response.failures.is_empty(), "ruwuma's new() starts with no failures");
	assert_eq!(to_string(&response.to_body()), r#"{"failures":{},"one_time_keys":{}}"#);
}

#[test]
fn get_profile_key_response_is_the_bare_map() {
	let fixture = r#"{"m.example":{"a":1}}"#;
	let response: get_profile_key::unstable::Response = golden(fixture);
	assert_eq!(response.value.len(), 1);
	assert!(response.value.contains_key("m.example"));
}

#[test]
fn set_profile_key_body_is_the_bare_object() {
	let kv_pair = BTreeMap::from([("m.example".to_owned(), from_str::<Value>("true").unwrap())]);
	let request = set_profile_key::unstable::Request {
		user_id: mtx_slipstream::user_id!("@a:example.org"),
		key_name: "m.example".to_owned(),
		kv_pair,
	};
	assert_eq!(to_string(&request.body().unwrap()), r#"{"m.example":true}"#);
}

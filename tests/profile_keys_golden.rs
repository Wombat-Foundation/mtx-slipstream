use std::collections::BTreeMap;

use mtx_slipstream::{
	OwnedUserId,
	client_api::profile_keys::{delete_profile_key, get_profile_key, set_profile_key},
	codec::{from_str, to_string},
	endpoint::{EndpointRequest, EndpointResponse},
	json::Value,
};

#[test]
fn get_profile_key_is_a_bare_map() {
	let body: Value = from_str(r#"{"color":"blue","count":3}"#).unwrap();
	let response = get_profile_key::unstable::Response::from_body(&body).unwrap();
	assert_eq!(to_string(&response.to_body()), r#"{"color":"blue","count":3}"#);
}

#[test]
fn set_profile_key_uses_a_bare_object_body() {
	let request = set_profile_key::unstable::Request {
		user_id: OwnedUserId::parse("@alice:example.org").unwrap(),
		key_name: "profile".into(),
		kv_pair: BTreeMap::from([
			("color".into(), Value::String("blue".into())),
			("count".into(), Value::from(3_u64)),
		]),
	};
	assert_eq!(to_string(&request.body().unwrap()), r#"{"color":"blue","count":3}"#);
}

#[test]
fn set_profile_key_accepts_a_missing_body_as_empty_map() {
	// ruma substitutes `{}` for a completely empty body, so the same has to hold
	// here or a bodyless PUT would 400 where ruma decodes it.
	let path = vec!["@alice:example.org".into(), "profile".into()];
	let request = set_profile_key::unstable::Request::from_parts(&path, &[], None).unwrap();
	assert!(request.kv_pair.is_empty());
}

#[test]
fn delete_profile_key_accepts_a_body() {
	let path = vec!["@alice:example.org".into(), "profile".into()];
	let body: Value = from_str(r#"{"profile":null}"#).unwrap();
	let request =
		delete_profile_key::unstable::Request::from_parts(&path, &[], Some(&body)).unwrap();
	assert_eq!(request.kv_pair.len(), 1);
}

#[test]
fn delete_profile_key_accepts_no_body() {
	let path = vec!["@alice:example.org".into(), "profile".into()];
	let request = delete_profile_key::unstable::Request::from_parts(&path, &[], None).unwrap();
	assert!(request.kv_pair.is_empty());
}

use std::collections::BTreeMap;

use mtx_slipstream::{
	OwnedUserId,
	client_api::profile_keys::{get_profile_key, set_profile_key},
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
fn set_profile_key_rejects_a_missing_body() {
	let path = vec!["@alice:example.org".into(), "profile".into()];
	assert!(set_profile_key::unstable::Request::from_parts(&path, &[], None).is_err());
}

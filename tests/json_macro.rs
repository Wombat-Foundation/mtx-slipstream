use mtx_slipstream::json::json;

#[test]
fn json_macro_handles_common_shapes() {
	let name = "bob";
	let n = 3_u64;
	let v = json!({
		"a": null,
		"name": name,
		"n": n + 1,
		"list": [1, "two", null, { "k": true }],
		"nested": { "x": [] },
	});
	assert_eq!(
		mtx_slipstream::codec::to_string(&v),
		r#"{"a":null,"list":[1,"two",null,{"k":true}],"n":4,"name":"bob","nested":{"x":[]}}"#
	);
}

#[test]
fn matrix_ids_parse_via_from_str() {
	let id: mtx_slipstream::OwnedUserId = "@a:example.org".parse().unwrap();
	assert_eq!(id.as_str(), "@a:example.org");
}

#[test]
fn ids_convert_into_string_and_triples_round_trip() {
	use mtx_slipstream::codec::{from_str, to_string};
	let id: mtx_slipstream::OwnedEventId = "$e".into();
	assert_eq!(String::from(&id), "$e");
	assert_eq!(String::from(id), "$e");
	let triple = (1_u64, "x".to_owned(), 3_u64);
	let back: (u64, String, u64) = from_str(&to_string(&triple)).unwrap();
	assert_eq!(back, triple);
}

#[test]
fn slipstream_json_interpolates_codec_types() {
	use mtx_slipstream::{OwnedEventId, codec::to_string, json};
	let id: OwnedEventId = "$e".into();
	let ids = vec![id.clone()];
	let v =
		json!({ "id": id, "ids": ids, "n": 1_u64, "s": "x", "nested": [null, { "ok": true }] });
	assert_eq!(
		to_string(&v),
		r#"{"id":"$e","ids":["$e"],"n":1,"nested":[null,{"ok":true}],"s":"x"}"#
	);
}

#[test]
fn slipstream_json_accepts_models_values_and_primitives() {
	use mtx_slipstream::{
		OwnedRoomId, OwnedUserId, codec::to_string, events::receipt::ReceiptEventContent, json,
	};

	let content = ReceiptEventContent::default();
	let user: OwnedUserId = "@a:example.org".into();
	let rooms: Vec<OwnedRoomId> = vec!["!r:example.org".into(), "!s:example.org".into()];
	let raw = json::json!({ "inner": [1, 2] });

	let v = json!({
		"type": "m.receipt",
		"content": content,
		"user": user,
		"rooms": rooms,
		"nested": { "deep": [ { "user": user, }, null, ], },
		"flag": false,
		"raw": raw,
		"missing": null,
		"count": 7_u64,
	});

	assert_eq!(
		to_string(&v),
		concat!(
			r#"{"content":{},"count":7,"flag":false,"missing":null,"#,
			r#""nested":{"deep":[{"user":"@a:example.org"},null]},"#,
			r#""raw":{"inner":[1,2]},"rooms":["!r:example.org","!s:example.org"],"#,
			r#""type":"m.receipt","user":"@a:example.org"}"#
		)
	);
}

#[test]
fn slipstream_json_accepts_computed_keys() {
	use mtx_slipstream::{OwnedUserId, codec::to_string, json};
	let alice: OwnedUserId = "@alice:foo".into();
	let v = json!({ "users": { (alice): 100, ("lit"): 1 } });
	assert_eq!(to_string(&v), r#"{"users":{"@alice:foo":100,"lit":1}}"#);
}

#[test]
fn room_version_parse() {
	use mtx_slipstream::RoomVersionId;
	assert_eq!(RoomVersionId::parse("10").unwrap(), RoomVersionId::V10);
	assert_eq!(RoomVersionId::parse("custom-1").unwrap().as_str(), "custom-1");
}

#[test]
fn id_accessors() {
	use mtx_slipstream::{OwnedRoomAliasId, OwnedRoomId, OwnedServerName};
	let alias: OwnedRoomAliasId = "#room:example.org".into();
	assert_eq!(alias.alias(), "room");
	let room: OwnedRoomId = "!abc:example.org".into();
	assert_eq!(room.localpart(), "abc");
	let plain: OwnedServerName = "example.org".into();
	assert_eq!((plain.host(), plain.port(), plain.is_ip_literal()), ("example.org", None, false));
	let with_port: OwnedServerName = "example.org:8448".into();
	assert_eq!((with_port.host(), with_port.port()), ("example.org", Some(8448)));
	let v6: OwnedServerName = "[::1]:8448".into();
	assert_eq!((v6.host(), v6.port(), v6.is_ip_literal()), ("[::1]", Some(8448), true));
	let v4: OwnedServerName = "10.0.0.1:1".into();
	assert!(v4.is_ip_literal());
}

#[derive(Debug, PartialEq)]
struct Sample {
	kind: String,
	sender: mtx_slipstream::OwnedUserId,
	redacts: Option<mtx_slipstream::OwnedEventId>,
	tags: Vec<String>,
}

mtx_slipstream::codec_struct!(Sample {
	kind: String = ("type"),
	sender: mtx_slipstream::OwnedUserId = ("sender"),
	redacts: Option<mtx_slipstream::OwnedEventId> = ("redacts", omit),
	tags: Vec<String> = ("tags", omit),
});

#[test]
fn codec_struct_renames_omits_and_ignores_unknown_keys() {
	use mtx_slipstream::codec::{from_str, to_string};
	let sample = Sample {
		kind: "m.room.message".into(),
		sender: "@a:example.org".into(),
		redacts: None,
		tags: vec![],
	};
	assert_eq!(
		to_string(&sample),
		r#"{"sender":"@a:example.org","tags":[],"type":"m.room.message"}"#
	);
	let parsed: Sample =
		from_str(r#"{"type":"m.room.message","sender":"@a:example.org","extra":1}"#).unwrap();
	assert_eq!(parsed, sample);
	assert!(from_str::<Sample>(r#"{"sender":"@a:example.org"}"#).is_err());
}

#[test]
fn error_codes_round_trip_for_every_named_kind() {
	use mtx_slipstream::api::client::error::ErrorKind;
	for code in [
		"M_THREEPID_IN_USE",
		"M_THREEPID_DENIED",
		"M_THREEPID_AUTH_FAILED",
		"M_INVITE_BLOCKED",
		"M_NOT_YET_UPLOADED",
		"M_CANNOT_OVERWRITE_MEDIA",
		"M_FEATURE_DISABLED",
		"UK.TIMEDOUT.MSC4406.SENDER_IGNORED",
		"M_WRONG_ROOM_KEYS_VERSION",
		"M_EXCLUSIVE",
	] {
		assert_eq!(ErrorKind::from_errcode(code).errcode(), code);
	}
	assert_eq!(ErrorKind::from_errcode("M_SOMETHING_ELSE").errcode(), "M_UNKNOWN");
	assert_eq!(
		ErrorKind::from_errcode("M_SENDER_IGNORED").errcode(),
		"UK.TIMEDOUT.MSC4406.SENDER_IGNORED"
	);
}

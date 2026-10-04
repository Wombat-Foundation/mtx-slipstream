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

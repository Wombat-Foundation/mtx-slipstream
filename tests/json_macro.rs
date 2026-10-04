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
	assert_eq!(String::from(id), "$e");
	let triple = (1_u64, "x".to_owned(), 3_u64);
	let back: (u64, String, u64) = from_str(&to_string(&triple)).unwrap();
	assert_eq!(back, triple);
}

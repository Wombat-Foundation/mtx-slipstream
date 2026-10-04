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

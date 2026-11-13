//! Direct `OwnedValue` → bytes conversion.
//!
//! The pipeline for federation PDUs is:
//! ```text
//! DB (JSON text) → rezzy-json parse → OwnedValue → patch → canonical bytes
//! ```
//!
//! Parsing, the value model and the strict canonical writer all live in
//! `rezzy-json`; this module only adapts them to `BytesMut` and the PDU
//! helpers below.

use std::io;

use crate::slipstream_json as sjson;
use bytes::BytesMut;

use alloc::string::String;
use alloc::vec::Vec;

/// Parse trusted JSON bytes (permissive: a repeated object key keeps its last
/// value).
///
/// Do not use this at a trust boundary; use [`parse_pdu_json`], which rejects
/// duplicate keys.
///
/// # Errors
///
/// Returns `sjson::Error` if the input is not valid JSON.
#[inline]
pub fn parse_json_bytes(buf: &[u8]) -> Result<sjson::OwnedValue, sjson::Error> {
	sjson::to_owned_value(buf)
}

/// Parse a trusted JSON string (permissive; see [`parse_json_bytes`]).
///
/// # Errors
///
/// Returns `sjson::Error` if the input is not valid JSON.
#[inline]
pub fn parse_json_str(s: &str) -> Result<sjson::OwnedValue, sjson::Error> {
	sjson::to_owned_value(s.as_bytes())
}

/// Parse PDU JSON bytes, rejecting duplicate object keys.
///
/// A permissive parse collapses `{"a":1,"a":2}` to `{"a":2}` before the value
/// is hashed or signed, so ingress must reject ambiguous input here: the
/// parsed value can no longer tell that a key was repeated.
///
/// # Errors
///
/// Returns `sjson::Error::DuplicateKey` for a repeated key, and an error if
/// the input is not valid UTF-8 or not valid JSON.
pub fn parse_pdu_json(buf: &[u8]) -> Result<sjson::OwnedValue, sjson::Error> {
	// `parse_bytes_strict` is the eventual direct entry point, but this tree
	// pins a rezzy revision that predates it; UTF-8 + `parse_strict` gives the
	// same duplicate-key rejection until the dependency is bumped.
	let text = core::str::from_utf8(buf).map_err(|_| sjson::Error::InvalidString)?;
	sjson::OwnedValue::parse_strict(text)
}

/// Appends `pdu` to `out` as canonical JSON.
///
/// The signed portion is validated with the strict Matrix number rules
/// (compact output, sorted keys, no floats, integers within ±(2^53 - 1)).
/// Only the PDU's top-level `unsigned` field is excluded from signing, so it
/// is not number-validated and is re-emitted unchanged; nested fields are
/// validated as usual.
///
/// Validation happens before anything is appended, so `out` is unchanged when
/// this returns an error.
///
/// # Errors
///
/// Returns `io::Error` if a number in the signed portion is a float or an
/// integer outside the canonical safe range.
pub fn write_canonical_into(pdu: &sjson::OwnedValue, out: &mut BytesMut) -> io::Result<()> {
	validate_signed_numbers(pdu)?;
	let text = rezzy::json::write_string_value(pdu)
		.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
	out.extend_from_slice(text.as_bytes());
	Ok(())
}

/// Validates the numbers in a PDU's signed portion without serializing it.
///
/// The signature covers everything except the top-level `unsigned` key, so
/// that key alone is skipped; any `unsigned` nested further down is still
/// part of the signed data and is validated.
fn validate_signed_numbers(pdu: &sjson::OwnedValue) -> io::Result<()> {
	if let sjson::OwnedValue::Object(object) = pdu {
		for (key, value) in object {
			if key != "unsigned" {
				validate_numbers(value)?;
			}
		}
		return Ok(());
	}
	validate_numbers(pdu)
}

fn validate_numbers(value: &sjson::OwnedValue) -> io::Result<()> {
	match value {
		sjson::OwnedValue::Number(number) if number.is_canonical_integer() => Ok(()),
		sjson::OwnedValue::Number(_) => {
			Err(io::Error::new(io::ErrorKind::InvalidData, "non-canonical number in signed PDU"))
		}
		sjson::OwnedValue::Array(items) => items.iter().try_for_each(validate_numbers),
		sjson::OwnedValue::Object(object) => object.values().try_for_each(validate_numbers),
		sjson::OwnedValue::Null | sjson::OwnedValue::Bool(_) | sjson::OwnedValue::String(_) => {
			Ok(())
		}
	}
}

/// Serialize an `OwnedValue` directly to a `BytesMut` buffer.
///
/// # Errors
///
/// Returns `io::Error` if a number in the signed portion is a float or an
/// integer outside the canonical safe range.
pub fn canonical_to_bytes(pdu: &sjson::OwnedValue) -> io::Result<BytesMut> {
	let mut bytes = BytesMut::new();
	write_canonical_into(pdu, &mut bytes)?;
	Ok(bytes)
}

/// Serialize an `OwnedValue` to a `String`.
///
/// # Errors
///
/// Returns `io::Error` if serialization fails.
///
/// # Panics
///
/// Panics if the serialized JSON is not valid UTF-8 (should never happen).
pub fn canonical_to_string(pdu: &sjson::OwnedValue) -> Result<String, std::io::Error> {
	let bytes = canonical_to_bytes(pdu)?;
	Ok(String::from_utf8(bytes.to_vec()).expect("JSON serialization produces valid UTF-8"))
}

/// Serialize an `OwnedValue` directly, removing specified fields.
///
/// Clones the value, removes the listed fields from the top-level object,
/// then serializes the result.
///
/// # Errors
///
/// Returns `io::Error` if serialization fails.
pub fn canonical_to_bytes_without(
	pdu: &sjson::OwnedValue,
	skip_fields: &[&str],
) -> io::Result<BytesMut> {
	let mut val = pdu.clone();
	if let Some(obj) = val.as_object_mut() {
		for field in skip_fields {
			obj.remove(*field);
		}
	}
	canonical_to_bytes(&val)
}

/// Remove fields from a JSON value in place.
///
/// If `pdu` is an object, each field name in `skip_fields` is removed.
/// Non-object values are left unchanged.
pub fn remove_fields(pdu: &mut sjson::OwnedValue, skip_fields: &[&str]) {
	if let Some(obj) = pdu.as_object_mut() {
		for field in skip_fields {
			obj.remove(*field);
		}
	}
}

/// Insert top-level fields into a raw JSON object's byte representation
/// without parsing it into an `OwnedValue` tree.
///
/// This is the fast path for the common "patch a couple of known keys into
/// an already-serialized PDU" case (e.g. injecting `unsigned`/`age` before
/// forwarding a PDU read straight from the database) — it avoids the
/// allocation cost of `to_owned_value` → mutate → re-serialize entirely.
///
/// `raw` must be a JSON object with no leading/trailing whitespace (i.e.
/// bytes ending in `}`), such as the output of [`canonical_to_bytes`] or a
/// PDU fetched verbatim from storage. `fields` are `(key, value)` pairs
/// where `value` is **already-serialized** JSON (a literal, string, or
/// nested object/array) and `key` contains no characters that require JSON
/// string escaping (safe for the fixed field names this is meant for, e.g.
/// `"unsigned"`, `"age"`).
///
/// Returns `raw` unchanged (copied) if `fields` is empty.
///
/// If a key in `fields` already exists as a top-level key in `raw`, that
/// entry is left untouched and skipped — this fast path only supports
/// *inserting* new keys, not replacing existing ones (which would require
/// locating and removing the old value, at which point a real parse is
/// cheaper and less error-prone).
///
/// # Panics
///
/// Debug-asserts that `raw` ends with `}` — the caller is expected to only
/// pass well-formed JSON objects.
#[must_use]
pub fn splice_insert_fields(raw: &[u8], fields: &[(&str, &[u8])]) -> Vec<u8> {
	debug_assert!(raw.last() == Some(&b'}'), "splice_insert_fields requires a JSON object");

	if fields.is_empty() {
		return raw.to_vec();
	}

	let body = raw.get(1..raw.len().saturating_sub(1)).unwrap_or(&[]);
	// An object with no existing fields (`{}`, or `{ }` with only
	// whitespace between the braces) needs no leading comma before the
	// first inserted field.
	let body_is_empty = body.iter().all(u8::is_ascii_whitespace);

	let extra: usize =
		fields.iter().map(|(k, v)| k.len().saturating_add(v.len()).saturating_add(4)).sum();
	let mut out = Vec::with_capacity(raw.len().saturating_add(extra));
	out.extend_from_slice(&raw[..raw.len().saturating_sub(1)]);

	let mut wrote_any = false;
	for (key, value) in fields {
		if has_top_level_key(body, key) {
			continue;
		}
		if !body_is_empty || wrote_any {
			out.push(b',');
		}
		out.push(b'"');
		out.extend_from_slice(key.as_bytes());
		out.extend_from_slice(b"\":");
		out.extend_from_slice(value);
		wrote_any = true;
	}
	out.push(b'}');
	out
}

/// Checks whether `key` appears as a top-level (depth-0) key in the body of
/// a JSON object (the bytes strictly between its outer `{` and `}`).
///
/// This is a lightweight scan, not a full parser: it tracks string/escape
/// state and brace/bracket nesting depth just enough to avoid mistaking a
/// key name that appears inside a nested value or a string literal for a
/// real top-level key.
fn has_top_level_key(body: &[u8], key: &str) -> bool {
	let key = key.as_bytes();

	let mut depth: i32 = 0;
	let mut in_string = false;
	let mut escaped = false;
	let mut index = 0;
	while index < body.len() {
		let byte = body[index];
		if in_string {
			if escaped {
				escaped = false;
			} else if byte == b'\\' {
				escaped = true;
			} else if byte == b'"' {
				in_string = false;
			}
			index = index.saturating_add(1);
			continue;
		}
		match byte {
			b'"' => {
				if depth == 0 {
					let after_quote = index.saturating_add(1);
					let after_key = after_quote.saturating_add(key.len());
					if body.get(after_quote..after_key) == Some(key)
						&& body.get(after_key..after_key.saturating_add(2))
							== Some(b"\":".as_slice())
					{
						return true;
					}
				}
				in_string = true;
			}
			b'{' | b'[' => depth = depth.saturating_add(1),
			b'}' | b']' => depth = depth.saturating_sub(1),
			_ => {}
		}
		index = index.saturating_add(1);
	}
	false
}
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
	use sjson::json;

	use super::*;

	#[test]
	fn test_parse_json_bytes() {
		let input = br#"{"event_id":"$abc","type":"m.room.create"}"#;
		let val = parse_json_bytes(input).unwrap();
		assert_eq!(val["event_id"], "$abc");
	}

	#[test]
	fn test_parse_json_str() {
		let val = parse_json_str(r#"{"key":"value"}"#).unwrap();
		assert_eq!(val["key"], "value");
	}

	#[test]
	fn test_parse_pdu_json() {
		let input = br#"{"event_id":"$abc","content":{}}"#;
		let val = parse_pdu_json(input).unwrap();
		assert_eq!(val["event_id"], "$abc");
	}

	#[test]
	fn test_parse_pdu_json_rejects_duplicate_keys() {
		assert!(matches!(parse_pdu_json(br#"{"a":1,"a":2}"#), Err(sjson::Error::DuplicateKey)));
		assert!(matches!(
			parse_pdu_json(br#"{"content":{"b":1,"b":1}}"#),
			Err(sjson::Error::DuplicateKey)
		));
		// The permissive parser keeps the last value.
		assert_eq!(parse_json_bytes(br#"{"a":1,"a":2}"#).unwrap()["a"].as_i64(), Some(2));
	}

	#[test]
	fn test_canonical_to_bytes() {
		let obj = json!({
			"event_id": "$abc",
			"type": "m.room.create",
			"content": {"creator": "@user:example.com"}
		});
		let bytes = canonical_to_bytes(&obj).unwrap();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
	}

	#[test]
	fn test_canonical_to_string() {
		let obj = json!({"key": "value"});
		let s = canonical_to_string(&obj).unwrap();
		let input = s.into_bytes();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["key"], "value");
	}

	#[test]
	fn test_canonical_to_bytes_without() {
		let obj = json!({
			"event_id": "$abc",
			"unsigned": {"transaction_id": "t1"},
			"type": "m.room.create"
		});
		let bytes = canonical_to_bytes_without(&obj, &["unsigned"]).unwrap();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
		assert!(parsed.get("unsigned").is_none());
	}

	#[test]
	fn test_canonical_output_sorts_nested_keys() {
		let value = json!({"z": {"b": 2, "a": 1}, "a": 0});

		assert_eq!(canonical_to_bytes(&value).unwrap().as_ref(), br#"{"a":0,"z":{"a":1,"b":2}}"#,);
		assert_eq!(canonical_to_string(&value).unwrap(), r#"{"a":0,"z":{"a":1,"b":2}}"#,);
	}

	#[test]
	fn test_canonical_rejects_float() {
		let value = json!({"depth": 1.5});
		let err = canonical_to_bytes(&value).unwrap_err();
		assert_eq!(err.kind(), io::ErrorKind::InvalidData);
	}

	#[test]
	fn test_canonical_rejects_negative_zero_float() {
		let value = json!({"x": -0.0});
		let err = canonical_to_bytes(&value).unwrap_err();
		assert_eq!(err.kind(), io::ErrorKind::InvalidData);
	}

	#[test]
	fn test_canonical_rejects_integer_above_safe_range() {
		let value = json!({"count": 9_007_199_254_740_992_u64}); // 2^53
		assert!(canonical_to_bytes(&value).is_err());
	}

	#[test]
	fn test_canonical_rejects_integer_below_safe_range() {
		let value = json!({"count": -9_007_199_254_740_992_i64}); // -(2^53)
		assert!(canonical_to_bytes(&value).is_err());
	}

	#[test]
	fn test_canonical_accepts_boundary_safe_integers() {
		let value = json!({
			"max": 9_007_199_254_740_991_i64,
			"min": -9_007_199_254_740_991_i64
		});
		assert!(canonical_to_bytes(&value).is_ok());
	}

	#[test]
	fn test_canonical_rejects_float_nested_in_array() {
		let value = json!({"list": [1, 2, 2.5]});
		assert!(canonical_to_bytes(&value).is_err());
	}

	#[test]
	fn test_canonical_rejects_float_nested_in_object() {
		let value = json!({"content": {"ratio": 0.5}});
		assert!(canonical_to_bytes(&value).is_err());
	}

	#[test]
	fn test_canonical_allows_float_in_top_level_unsigned() {
		let value = json!({"content": {"x": 1}, "unsigned": {"age": 1.5}});
		assert!(canonical_to_bytes(&value).is_ok());
	}

	#[test]
	fn test_canonical_rejects_float_in_nested_unsigned() {
		let value = json!({"content": {"unsigned": {"age": 1.5}}});
		assert!(canonical_to_bytes(&value).is_err());
	}

	#[test]
	fn test_splice_insert_fields_into_populated_object() {
		let raw = br#"{"event_id":"$abc","type":"m.room.message"}"#;
		let out = splice_insert_fields(raw, &[("unsigned", br#"{"age":42}"#)]);

		let input = out.clone();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["event_id"].as_str(), Some("$abc"));
		assert_eq!(parsed["unsigned"]["age"].as_i64(), Some(42));
	}

	#[test]
	fn test_splice_insert_fields_into_empty_object() {
		let raw = br"{}";
		let out = splice_insert_fields(raw, &[("age", b"42")]);

		let input = out.clone();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["age"].as_i64(), Some(42));
	}

	#[test]
	fn test_splice_insert_fields_multiple() {
		let raw = br#"{"event_id":"$abc"}"#;
		let out = splice_insert_fields(raw, &[("age", b"1"), ("transaction_id", br#""t1""#)]);

		let input = out.clone();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["event_id"].as_str(), Some("$abc"));
		assert_eq!(parsed["age"].as_i64(), Some(1));
		assert_eq!(parsed["transaction_id"].as_str(), Some("t1"));
	}

	#[test]
	fn test_splice_insert_fields_empty_fields_is_noop() {
		let raw = br#"{"event_id":"$abc"}"#;
		let out = splice_insert_fields(raw, &[]);
		assert_eq!(out, raw);
	}

	#[test]
	fn test_splice_insert_fields_into_whitespace_only_object() {
		// `{ }` has no top-level fields, but the byte before the closing
		// brace isn't `{` -- must not emit a leading comma.
		let raw = b"{ }";
		let out = splice_insert_fields(raw, &[("age", b"42")]);

		let input = out.clone();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["age"].as_i64(), Some(42));
	}

	#[test]
	fn test_splice_insert_fields_skips_existing_key() {
		let raw = br#"{"event_id":"$abc","age":1}"#;
		let out = splice_insert_fields(raw, &[("age", b"999"), ("new_field", b"7")]);

		let input = out.clone();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		// Existing "age" is left untouched, not duplicated or overwritten.
		assert_eq!(parsed["age"].as_i64(), Some(1));
		assert_eq!(parsed["new_field"].as_i64(), Some(7));
		assert_eq!(out.windows(6).filter(|w| *w == b"\"age\":").count(), 1);
	}

	#[test]
	fn test_splice_insert_fields_ignores_key_inside_nested_value() {
		// "age" appears inside a nested object's value here, not as a
		// top-level key -- it must not be mistaken for an existing field.
		let raw = br#"{"content":{"age":5}}"#;
		let out = splice_insert_fields(raw, &[("age", b"42")]);

		let input = out.clone();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed["age"].as_i64(), Some(42));
		assert_eq!(parsed["content"]["age"].as_i64(), Some(5));
	}

	#[test]
	fn test_remove_fields() {
		let mut obj = json!({
			"event_id": "$abc",
			"unsigned": {"transaction_id": "t1"},
			"type": "m.room.create"
		});
		remove_fields(&mut obj, &["unsigned", "event_id"]);
		assert!(obj.get("unsigned").is_none());
		assert!(obj.get("event_id").is_none());
		assert_eq!(obj["type"], "m.room.create");
	}
}

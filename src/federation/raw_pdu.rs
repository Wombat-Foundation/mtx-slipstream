//! Direct `OwnedValue` → bytes conversion.
//!
//! The current pipeline for federation PDUs is:
//! ```text
//! DB (JSON text) → simd_json::from_slice → OwnedValue → patch → simd-json → bytes
//! ```
//!
//! This module provides:
//! 1. **simd-json parsing** — use `simd_json::from_slice` instead of
//!    `serde_json::from_slice` for SIMD-accelerated deserialization of PDUs
//!    from the database.
//! 2. **Direct serialization** — serialize `OwnedValue` directly to bytes,
//!    skipping the `Box<RawValue>` intermediate allocation.

use std::io;

use bytes::{BufMut, BytesMut};
use simd_json::{OwnedValue, prelude::*};

use crate::writer::BufWriter;

/// Parse JSON bytes using simd-json (SIMD-accelerated).
///
/// The input buffer is mutated in-place during parsing (simd-json's
/// `ScratchSpace` strategy), so the caller should not rely on the buffer
/// contents after this call.
///
/// # Errors
///
/// Returns `simd_json::Error` if the input is not valid JSON.
#[inline]
pub fn parse_jsonsimd(buf: &mut [u8]) -> Result<simd_json::OwnedValue, simd_json::Error> {
	simd_json::to_owned_value(buf)
}

/// Parse a JSON string using simd-json.
///
/// # Errors
///
/// Returns `simd_json::Error` if the input is not valid JSON.
#[inline]
pub fn parse_jsonsimd_str(s: &mut str) -> Result<simd_json::OwnedValue, simd_json::Error> {
	let mut buf = s.as_bytes().to_vec();
	simd_json::to_owned_value(&mut buf)
}

/// Parse JSON bytes from the database using simd-json.
///
/// This is optimized for the PDU reading path where we fetch JSON text from
/// `RocksDB` and need to deserialize it into an `OwnedValue`.
///
/// # Arguments
///
/// * `buf` - The raw JSON bytes from the database. **Mutated in-place** by
///   simd-json's parsing strategy.
///
/// # Errors
///
/// Returns `simd_json::Error` if the input is not valid JSON.
pub fn parse_pdu_json(buf: &mut [u8]) -> Result<simd_json::OwnedValue, simd_json::Error> {
	simd_json::to_owned_value(buf)
}

/// Serialize an `OwnedValue` directly to a `BytesMut` buffer.
///
/// # Errors
///
/// Returns `io::Error` if serialization fails.
pub fn canonical_to_bytes(pdu: &simd_json::OwnedValue) -> io::Result<BytesMut> {
	let mut buf = BytesMut::with_capacity(2048);
	write_canonical_value(&mut buf, pdu)?;
	Ok(buf)
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
pub fn canonical_to_string(pdu: &simd_json::OwnedValue) -> Result<String, std::io::Error> {
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
	pdu: &simd_json::OwnedValue,
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

/// Matrix canonical JSON requires integers to round-trip exactly through an
/// IEEE-754 double, i.e. within ±(2^53 - 1). See the Matrix spec's appendix
/// on canonical JSON.
const CANONICAL_MAX_SAFE_INT: i64 = 9_007_199_254_740_991;
const CANONICAL_MIN_SAFE_INT: i64 = -9_007_199_254_740_991;

fn write_canonical_value(buf: &mut BytesMut, value: &simd_json::OwnedValue) -> io::Result<()> {
	if let Some(array) = value.as_array() {
		buf.put_u8(b'[');
		for (index, item) in array.iter().enumerate() {
			if index > 0 {
				buf.put_u8(b',');
			}
			write_canonical_value(buf, item)?;
		}
		buf.put_u8(b']');
	} else if let Some(object) = value.as_object() {
		let mut entries: Vec<_> = object.iter().collect();
		entries.sort_unstable_by_key(|(left, _)| *left);

		buf.put_u8(b'{');
		for (index, (key, item)) in entries.iter().enumerate() {
			if index > 0 {
				buf.put_u8(b',');
			}
			OwnedValue::from(key.as_str()).write(&mut BufWriter(buf))?;
			buf.put_u8(b':');
			write_canonical_value(buf, item)?;
		}
		buf.put_u8(b'}');
	} else {
		// Matrix canonical JSON forbids floating-point values outright, and
		// restricts integers to the range that round-trips exactly through
		// an f64. Reject violations here rather than silently emitting
		// non-canonical bytes that would fail interop event-hash checks
		// against other homeservers.
		if value.is_f64() {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"canonical JSON forbids floating-point values",
			));
		}
		let out_of_range = value
			.as_i64()
			.map(|i| !(CANONICAL_MIN_SAFE_INT..=CANONICAL_MAX_SAFE_INT).contains(&i))
			.or_else(|| {
				value
					.as_u64()
					.map(|u| u > u64::try_from(CANONICAL_MAX_SAFE_INT).unwrap_or(u64::MAX))
			})
			.unwrap_or(false);
		if out_of_range {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"integer exceeds canonical JSON safe range (+/-(2^53-1))",
			));
		}
		value.write(&mut BufWriter(buf))?;
	}

	Ok(())
}

/// Remove fields from a JSON value in place.
///
/// If `pdu` is an object, each field name in `skip_fields` is removed.
/// Non-object values are left unchanged.
pub fn remove_fields(pdu: &mut simd_json::OwnedValue, skip_fields: &[&str]) {
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

	let extra: usize = fields
		.iter()
		.map(|(k, v)| k.len().saturating_add(v.len()).saturating_add(4))
		.sum();
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
			| b'"' => {
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
			},
			| b'{' | b'[' => depth = depth.saturating_add(1),
			| b'}' | b']' => depth = depth.saturating_sub(1),
			| _ => {},
		}
		index = index.saturating_add(1);
	}
	false
}
#[cfg(test)]
#[coverage(off)]
mod tests {
	use simd_json::json;

	use super::*;

	#[test]
	fn test_parse_jsonsimd() {
		let mut input = br#"{"event_id":"$abc","type":"m.room.create"}"#.to_vec();
		let val = parse_jsonsimd(&mut input).unwrap();
		assert_eq!(val["event_id"], "$abc");
	}

	#[test]
	fn test_parse_jsonsimd_str() {
		let mut input = r#"{"key":"value"}"#.to_string();
		let val = parse_jsonsimd_str(&mut input).unwrap();
		assert_eq!(val["key"], "value");
	}

	#[test]
	fn test_parse_pdu_json() {
		let mut input = br#"{"event_id":"$abc","content":{}}"#.to_vec();
		let val = parse_pdu_json(&mut input).unwrap();
		assert_eq!(val["event_id"], "$abc");
	}

	#[test]
	fn test_canonical_to_bytes() {
		let obj = json!({
			"event_id": "$abc",
			"type": "m.room.create",
			"content": {"creator": "@user:example.com"}
		});
		let bytes = canonical_to_bytes(&obj).unwrap();
		let mut input = bytes.to_vec();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
	}

	#[test]
	fn test_canonical_to_string() {
		let obj = json!({"key": "value"});
		let s = canonical_to_string(&obj).unwrap();
		let mut input = s.into_bytes();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
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
		let mut input = bytes.to_vec();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
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
	fn test_splice_insert_fields_into_populated_object() {
		let raw = br#"{"event_id":"$abc","type":"m.room.message"}"#;
		let out = splice_insert_fields(raw, &[("unsigned", br#"{"age":42}"#)]);

		let mut input = out.clone();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
		assert_eq!(parsed["unsigned"]["age"], 42);
	}

	#[test]
	fn test_splice_insert_fields_into_empty_object() {
		let raw = br"{}";
		let out = splice_insert_fields(raw, &[("age", b"42")]);

		let mut input = out.clone();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		assert_eq!(parsed["age"], 42);
	}

	#[test]
	fn test_splice_insert_fields_multiple() {
		let raw = br#"{"event_id":"$abc"}"#;
		let out = splice_insert_fields(raw, &[("age", b"1"), ("transaction_id", br#""t1""#)]);

		let mut input = out.clone();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
		assert_eq!(parsed["age"], 1);
		assert_eq!(parsed["transaction_id"], "t1");
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

		let mut input = out.clone();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		assert_eq!(parsed["age"], 42);
	}

	#[test]
	fn test_splice_insert_fields_skips_existing_key() {
		let raw = br#"{"event_id":"$abc","age":1}"#;
		let out = splice_insert_fields(raw, &[("age", b"999"), ("new_field", b"7")]);

		let mut input = out.clone();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		// Existing "age" is left untouched, not duplicated or overwritten.
		assert_eq!(parsed["age"], 1);
		assert_eq!(parsed["new_field"], 7);
		assert_eq!(out.windows(6).filter(|w| *w == b"\"age\":").count(), 1);
	}

	#[test]
	fn test_splice_insert_fields_ignores_key_inside_nested_value() {
		// "age" appears inside a nested object's value here, not as a
		// top-level key -- it must not be mistaken for an existing field.
		let raw = br#"{"content":{"age":5}}"#;
		let out = splice_insert_fields(raw, &[("age", b"42")]);

		let mut input = out.clone();
		let parsed: simd_json::OwnedValue = simd_json::to_owned_value(&mut input).unwrap();
		assert_eq!(parsed["age"], 42);
		assert_eq!(parsed["content"]["age"], 5);
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

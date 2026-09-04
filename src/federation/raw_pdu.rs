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

	// An object with no existing fields (`{}`) needs no leading comma before
	// the first inserted field.
	let body_is_empty = raw.len() >= 2 && raw[raw.len().saturating_sub(2)] == b'{';

	let extra: usize = fields
		.iter()
		.map(|(k, v)| k.len().saturating_add(v.len()).saturating_add(4))
		.sum();
	let mut out = Vec::with_capacity(raw.len().saturating_add(extra));
	out.extend_from_slice(&raw[..raw.len().saturating_sub(1)]);

	for (index, (key, value)) in fields.iter().enumerate() {
		if !body_is_empty || index > 0 {
			out.push(b',');
		}
		out.push(b'"');
		out.extend_from_slice(key.as_bytes());
		out.extend_from_slice(b"\":");
		out.extend_from_slice(value);
	}
	out.push(b'}');
	out
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

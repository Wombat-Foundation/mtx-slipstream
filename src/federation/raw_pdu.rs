//! Direct `CanonicalJsonObject` → bytes conversion.
//!
//! The current pipeline for federation PDUs is:
//! ```text
//! DB (JSON text) → serde_json::from_slice → CanonicalJsonObject → to_raw_value → Box<RawValue>
//! ```
//!
//! This module provides two optimizations:
//! 1. **simd-json parsing** — use `simd_json::from_slice` instead of
//!    `serde_json::from_slice` for SIMD-accelerated deserialization of PDUs
//!    from the database.
//! 2. **Direct serialization** — serialize `CanonicalJsonObject` directly to
//!    bytes, skipping the `Box<RawValue>` intermediate allocation.

use std::io;

use bytes::BytesMut;
use serde::Serialize;

/// Parse JSON bytes using simd-json (SIMD-accelerated).
///
/// This is a drop-in replacement for `serde_json::from_slice` that uses
/// SIMD instructions for significantly faster parsing of large JSON payloads.
///
/// The input buffer is mutated in-place during parsing (simd-json's
/// `ScratchSpace` strategy), so the caller should not rely on the buffer
/// contents after this call.
#[inline]
pub fn parse_jsonsimd<'a, T>(buf: &'a mut [u8]) -> Result<T, simd_json::Error>
where
	T: serde::Deserialize<'a>,
{
	simd_json::from_slice(buf)
}

/// Parse a JSON string using simd-json.
#[inline]
pub fn parse_jsonsimd_str<'a, T>(s: &'a mut str) -> Result<T, simd_json::Error>
where
	T: serde::Deserialize<'a>,
{
	// SAFETY: simd_json::from_str requires the input to be valid UTF-8,
	// which &str guarantees.
	unsafe { simd_json::from_str(s) }
}

/// Parse JSON bytes from the database using simd-json.
///
/// This is optimized for the PDU reading path where we fetch JSON text from
/// RocksDB and need to deserialize it into a `CanonicalJsonObject` or
/// `serde_json::Value`.
///
/// # Arguments
/// * `buf` - The raw JSON bytes from the database. **Mutated in-place** by
///   simd-json's parsing strategy.
pub fn parse_pdu_json(buf: &mut [u8]) -> Result<serde_json::Value, simd_json::Error> {
	simd_json::from_slice(buf)
}

/// Serialize a value directly to a `BytesMut` buffer.
///
/// Uses `serde_json::to_writer` internally (simd-json's serialization
/// delegates to serde_json — the performance win is on the parse side).
pub fn canonical_to_bytes<T: Serialize>(pdu: &T) -> Result<BytesMut, serde_json::Error> {
	let mut buf = BytesMut::with_capacity(2048);
	let mut writer = BufWriter(&mut buf);
	serde_json::to_writer(&mut writer, pdu)?;
	Ok(buf)
}

/// Serialize a value to a `String`.
pub fn canonical_to_string<T: Serialize>(pdu: &T) -> Result<String, serde_json::Error> {
	serde_json::to_string(pdu)
}

/// Serialize a value directly, removing specified fields.
///
/// Parses with simd-json, removes fields, then serializes with serde_json.
pub fn canonical_to_bytes_without<T: Serialize>(
	pdu: &T,
	skip_fields: &[&str],
) -> Result<BytesMut, serde_json::Error> {
	let mut val: serde_json::Value = serde_json::to_value(pdu)?;
	if let Some(obj) = val.as_object_mut() {
		for field in skip_fields {
			obj.remove(*field);
		}
	}
	let mut buf = BytesMut::with_capacity(2048);
	let mut writer = BufWriter(&mut buf);
	serde_json::to_writer(&mut writer, &val)?;
	Ok(buf)
}

/// Remove fields from a JSON value in place.
pub fn remove_fields(pdu: &mut serde_json::Value, skip_fields: &[&str]) {
	if let Some(obj) = pdu.as_object_mut() {
		for field in skip_fields {
			obj.remove(*field);
		}
	}
}

struct BufWriter<'a>(&'a mut BytesMut);

impl io::Write for BufWriter<'_> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.0.extend_from_slice(buf);
		Ok(buf.len())
	}

	fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn test_parse_jsonsimd() {
		let mut input = br#"{"event_id":"$abc","type":"m.room.create"}"#.to_vec();
		let val: serde_json::Value = parse_jsonsimd(&mut input).unwrap();
		assert_eq!(val["event_id"], "$abc");
	}

	#[test]
	fn test_parse_jsonsimd_str() {
		let mut input = r#"{"key":"value"}"#.to_string();
		let val: serde_json::Value = parse_jsonsimd_str(&mut input).unwrap();
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
		let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
	}

	#[test]
	fn test_canonical_to_string() {
		let obj = json!({"key": "value"});
		let s = canonical_to_string(&obj).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
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
		let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
		assert_eq!(parsed["event_id"], "$abc");
		assert!(parsed.get("unsigned").is_none());
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

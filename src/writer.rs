//! Thin adapter for writing simd-json values into a [`BytesMut`] buffer.

use std::io;

use bytes::BytesMut;
use simd_json::OwnedValue;

/// Adapter that implements [`io::Write`] for a [`BytesMut`] buffer.
///
/// Used with [`simd_json::to_writer`] to serialize values directly into
/// a growable byte buffer without intermediate heap allocations.
pub struct BufWriter<'a>(pub &'a mut BytesMut);

impl io::Write for BufWriter<'_> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.0.extend_from_slice(buf);
		Ok(buf.len())
	}

	fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

/// Serialize an `OwnedValue` directly to a `BytesMut` buffer.
///
/// Allocates an 8 KiB buffer, serializes the value via simd-json's native
/// serializer, and returns the result.
///
/// # Errors
///
/// Returns `simd_json::Error` if serialization fails.
#[inline]
pub fn to_bytes(value: &OwnedValue) -> Result<BytesMut, simd_json::Error> {
	let mut buf = BytesMut::with_capacity(8192);
	let mut writer = BufWriter(&mut buf);
	simd_json::to_writer(&mut writer, value)?;
	Ok(buf)
}

#[cfg(test)]
#[coverage(off)]
mod tests {
	use simd_json::json;

	use super::*;

	#[test]
	fn test_to_bytes_primitives() {
		let bytes = to_bytes(&json!(true)).unwrap();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed, json!(true));

		let bytes = to_bytes(&json!(42)).unwrap();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed, json!(42));

		let bytes = to_bytes(&json!("hello")).unwrap();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed, json!("hello"));
	}

	#[test]
	fn test_to_bytes_map() {
		let bytes = to_bytes(&json!({"a": 1, "b": "two"})).unwrap();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed, json!({"a": 1, "b": "two"}));
	}

	#[test]
	fn test_to_bytes_nested() {
		let bytes = to_bytes(&json!({
			"rooms": {
				"join": {
					"!room:example.com": {
						"timeline": {"events": [{"type": "m.room.message"}]}
					}
				}
			}
		}))
		.unwrap();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(
			parsed["rooms"]["join"]["!room:example.com"]["timeline"]["events"][0]["type"],
			"m.room.message"
		);
	}
}

//! Streaming JSON writer that serializes directly to a byte buffer.

use bytes::{BufMut, BytesMut};
use simd_json::{OwnedValue, prelude::*};

pub struct JsonWriter {
	buf: BytesMut,
}

impl JsonWriter {
	#[inline]
	pub fn with_capacity(cap: usize) -> Self { Self { buf: BytesMut::with_capacity(cap) } }

	#[inline]
	pub fn into_bytes(self) -> BytesMut { self.buf }

	#[inline]
	pub fn as_bytes(&self) -> &[u8] { &self.buf }

	/// Write an `OwnedValue` as JSON directly into the buffer.
	pub fn write_value(&mut self, value: &OwnedValue) -> Result<(), simd_json::Error> {
		if let Some(s) = value.as_str() {
			self.write_escaped_string(s);
		} else if value.is_null() {
			self.write_raw("null")?;
		} else if let Some(b) = value.as_bool() {
			self.write_raw(if b { "true" } else { "false" })?;
		} else if let Some(n) = value.as_u64() {
			let mut buf = itoa::Buffer::new();
			self.write_raw(buf.format(n))?;
		} else if let Some(n) = value.as_i64() {
			let mut buf = itoa::Buffer::new();
			self.write_raw(buf.format(n))?;
		} else if let Some(n) = value.as_f64() {
			let mut buf = ryu::Buffer::new();
			self.write_raw(buf.format(n))?;
		} else if let Some(arr) = value.as_array() {
			self.write_byte(b'[');
			for (i, item) in arr.iter().enumerate() {
				if i > 0 {
					self.write_byte(b',');
				}
				self.write_value(item)?;
			}
			self.write_byte(b']');
		} else if let Some(obj) = value.as_object() {
			self.write_byte(b'{');
			for (i, (key, val)) in obj.iter().enumerate() {
				if i > 0 {
					self.write_byte(b',');
				}
				self.write_escaped_string(key);
				self.write_byte(b':');
				self.write_value(val)?;
			}
			self.write_byte(b'}');
		}
		Ok(())
	}

	#[inline]
	pub fn write_raw(&mut self, raw: &str) -> Result<(), simd_json::Error> {
		self.buf.put_slice(raw.as_bytes());
		Ok(())
	}

	#[inline]
	pub fn write_byte(&mut self, b: u8) { self.buf.put_u8(b); }

	pub fn write_escaped_string(&mut self, s: &str) {
		self.write_byte(b'"');
		for c in s.chars() {
			match c {
				| '"' => self.buf.put_slice(b"\\\""),
				| '\\' => self.buf.put_slice(b"\\\\"),
				| '\n' => self.buf.put_slice(b"\\n"),
				| '\r' => self.buf.put_slice(b"\\r"),
				| '\t' => self.buf.put_slice(b"\\t"),
				| '\u{08}' => self.buf.put_slice(b"\\b"),
				| '\u{0c}' => self.buf.put_slice(b"\\f"),
				| c if c.is_control() => {
					self.buf
						.put_slice(format!("\\u{:04x}", c as u32).as_bytes());
				},
				| c => self.buf.put_slice(c.encode_utf8(&mut [0u8; 4]).as_bytes()),
			}
		}
		self.write_byte(b'"');
	}
}

/// Serialize an `OwnedValue` directly to a `BytesMut` buffer.
#[inline]
pub fn to_bytes(value: &OwnedValue) -> Result<BytesMut, simd_json::Error> {
	let mut writer = JsonWriter::with_capacity(8192);
	writer.write_value(value)?;
	Ok(writer.into_bytes())
}

#[cfg(test)]
#[coverage(off)]
mod tests {
	use simd_json::json;

	use super::*;

	#[test]
	fn test_write_primitives() {
		let mut w = JsonWriter::with_capacity(64);
		w.write_value(&json!(true)).unwrap();
		assert_eq!(w.as_bytes(), b"true");

		let mut w = JsonWriter::with_capacity(64);
		w.write_value(&json!(42)).unwrap();
		assert_eq!(w.as_bytes(), b"42");

		let mut w = JsonWriter::with_capacity(64);
		w.write_value(&json!("hello")).unwrap();
		assert_eq!(w.as_bytes(), b"\"hello\"");
	}

	#[test]
	fn test_write_map() {
		let mut w = JsonWriter::with_capacity(64);
		w.write_value(&json!({"a": 1, "b": "two"})).unwrap();
		let result = String::from_utf8(w.into_bytes().to_vec()).unwrap();
		let mut parsed_input = result.into_bytes();
		let parsed: simd_json::OwnedValue = simd_json::from_slice(&mut parsed_input).unwrap();
		assert_eq!(parsed, json!({"a": 1, "b": "two"}));
	}

	#[test]
	fn test_write_nested() {
		let mut w = JsonWriter::with_capacity(256);
		w.write_value(&json!({
			"rooms": {
				"join": {
					"!room:example.com": {
						"timeline": {"events": [{"type": "m.room.message"}]}
					}
				}
			}
		}))
		.unwrap();
		let result = String::from_utf8(w.into_bytes().to_vec()).unwrap();
		let mut parsed_input = result.into_bytes();
		let parsed: simd_json::OwnedValue = simd_json::from_slice(&mut parsed_input).unwrap();
		assert_eq!(
			parsed["rooms"]["join"]["!room:example.com"]["timeline"]["events"][0]["type"],
			"m.room.message"
		);
	}

	#[test]
	fn test_write_raw() {
		let mut w = JsonWriter::with_capacity(64);
		w.write_raw(r#"{"key":"value"}"#).unwrap();
		assert_eq!(w.as_bytes(), r#"{"key":"value"}"#.as_bytes());
	}

	#[test]
	fn test_to_bytes_matches_simd_json() {
		let value = json!({
			"next_batch": "s12345",
			"rooms": {
				"join": {
					"!abc:example.com": {
						"timeline": {
							"events": [
								{"type": "m.room.message", "content": {"body": "hello"}}
							],
							"limited": false,
							"prev_batch": "t123"
						},
						"state": {"events": []},
						"ephemeral": {"events": []},
						"account_data": {"events": []}
					}
				}
			}
		});

		let simd_bytes = simd_json::to_vec(&value).unwrap();
		let custom_bytes = to_bytes(&value).unwrap();

		let mut simd_input = simd_bytes.clone();
		let simd_parsed: simd_json::OwnedValue = simd_json::from_slice(&mut simd_input).unwrap();
		let mut custom_input = custom_bytes.to_vec();
		let custom_parsed: simd_json::OwnedValue =
			simd_json::from_slice(&mut custom_input).unwrap();
		assert_eq!(simd_parsed, custom_parsed);
	}
}

//! Streaming JSON writer that serializes directly to a byte buffer.
//!
//! This avoids intermediate `serde_json::Value` allocations by writing JSON
//! bytes directly during serialization.

use std::fmt;

use bytes::{BufMut, BytesMut};
use serde::ser::{self, Serialize};

/// A streaming JSON writer that builds a JSON byte buffer directly.
pub struct JsonWriter {
	buf: BytesMut,
}

impl JsonWriter {
	/// Create a new writer with the given initial capacity.
	#[inline]
	pub fn with_capacity(cap: usize) -> Self { Self { buf: BytesMut::with_capacity(cap) } }

	/// Consume the writer and return the accumulated bytes.
	#[inline]
	pub fn into_bytes(self) -> BytesMut { self.buf }

	/// Get a reference to the underlying buffer.
	#[inline]
	pub fn as_bytes(&self) -> &[u8] { &self.buf }

	/// Serialize a value directly into the buffer.
	#[inline]
	pub fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<(), serde_json::Error> {
		let mut ser = Serializer { writer: self };
		value.serialize(&mut ser)
	}

	/// Write a raw JSON string directly (no escaping, no wrapping).
	#[inline]
	pub fn write_raw(&mut self, raw: &str) { self.buf.put_slice(raw.as_bytes()); }

	#[inline]
	fn write_byte(&mut self, b: u8) { self.buf.put_u8(b); }

	fn write_escaped_string(&mut self, s: &str) {
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
					let mut buf = [0u8; 6];
					write!(&mut buf[..], "\\u{:04x}", c as u32).unwrap();
					self.buf.put_slice(&buf);
				},
				| c => self.buf.put_slice(c.encode_utf8(&mut [0u8; 4]).as_bytes()),
			}
		}
		self.write_byte(b'"');
	}
}

struct Serializer<'a> {
	writer: &'a mut JsonWriter,
}

impl<'a> ser::Serializer for Serializer<'a> {
	type Error = WriterError;
	type Ok = ();
	type SerializeMap = MapSerializer<'a>;
	type SerializeSeq = SeqSerializer<'a>;
	type SerializeStruct = MapSerializer<'a>;
	type SerializeStructVariant = StructVariantSerializer<'a>;
	type SerializeTuple = SeqSerializer<'a>;
	type SerializeTupleStruct = SeqSerializer<'a>;
	type SerializeTupleVariant = SeqSerializer<'a>;

	fn serialize_bool(self, v: bool) -> Result<(), WriterError> {
		self.writer.write_raw(if v { "true" } else { "false" });
		Ok(())
	}

	fn serialize_i8(self, v: i8) -> Result<(), WriterError> { self.serialize_i64(i64::from(v)) }

	fn serialize_i16(self, v: i16) -> Result<(), WriterError> { self.serialize_i64(i64::from(v)) }

	fn serialize_i32(self, v: i32) -> Result<(), WriterError> { self.serialize_i64(i64::from(v)) }

	fn serialize_i64(self, v: i64) -> Result<(), WriterError> {
		let mut buf = itoa::Buffer::new();
		self.writer.write_raw(buf.format(v));
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<(), WriterError> { self.serialize_u64(u64::from(v)) }

	fn serialize_u16(self, v: u16) -> Result<(), WriterError> { self.serialize_u64(u64::from(v)) }

	fn serialize_u32(self, v: u32) -> Result<(), WriterError> { self.serialize_u64(u64::from(v)) }

	fn serialize_u64(self, v: u64) -> Result<(), WriterError> {
		let mut buf = itoa::Buffer::new();
		self.writer.write_raw(buf.format(v));
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<(), WriterError> { self.serialize_f64(f64::from(v)) }

	fn serialize_f64(self, v: f64) -> Result<(), WriterError> {
		let mut buf = ryu::Buffer::new();
		self.writer.write_raw(buf.format(v));
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<(), WriterError> {
		let mut s = String::with_capacity(6);
		s.push(v);
		self.writer.write_escaped_string(&s);
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<(), WriterError> {
		self.writer.write_escaped_string(v);
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<(), WriterError> {
		use ser::SerializeSeq;
		let mut seq = self.serialize_seq(Some(v.len()))?;
		for byte in v {
			seq.serialize_element(byte)?;
		}
		seq.end()
	}

	fn serialize_none(self) -> Result<(), WriterError> {
		self.writer.write_raw("null");
		Ok(())
	}

	fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), WriterError> {
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<(), WriterError> {
		self.writer.write_raw("null");
		Ok(())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<(), WriterError> {
		self.writer.write_raw("null");
		Ok(())
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<(), WriterError> {
		self.writer.write_escaped_string(variant);
		Ok(())
	}

	fn serialize_newtype_struct<T: ?Sized + Serialize>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<(), WriterError> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized + Serialize>(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<(), WriterError> {
		self.writer.write_byte(b'{');
		self.writer.write_escaped_string(variant);
		self.writer.write_byte(b':');
		value.serialize(Serializer { writer: self.writer })?;
		self.writer.write_byte(b'}');
		Ok(())
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<SeqSerializer<'a>, WriterError> {
		self.writer.write_byte(b'[');
		Ok(SeqSerializer { writer: self.writer, len, count: 0 })
	}

	fn serialize_tuple(self, len: usize) -> Result<SeqSerializer<'a>, WriterError> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<SeqSerializer<'a>, WriterError> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<SeqSerializer<'a>, WriterError> {
		self.writer.write_byte(b'{');
		self.writer.write_escaped_string(variant);
		self.writer.write_byte(b':');
		self.serialize_seq(Some(len))
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<MapSerializer<'a>, WriterError> {
		self.writer.write_byte(b'{');
		Ok(MapSerializer { writer: self.writer, first: true })
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<MapSerializer<'a>, WriterError> {
		self.serialize_map(None)
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<StructVariantSerializer<'a>, WriterError> {
		self.writer.write_byte(b'{');
		self.writer.write_escaped_string(variant);
		self.writer.write_byte(b':');
		self.writer.write_byte(b'{');
		Ok(StructVariantSerializer { writer: self.writer, first: true })
	}
}

struct SeqSerializer<'a> {
	writer: &'a mut JsonWriter,
	len: Option<usize>,
	count: usize,
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), WriterError> {
		if self.count > 0 {
			self.writer.write_byte(b',');
		}
		self.count += 1;
		value.serialize(Serializer { writer: self.writer })
	}

	fn end(self) -> Result<(), WriterError> {
		self.writer.write_byte(b']');
		Ok(())
	}
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), WriterError> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<(), WriterError> { ser::SerializeSeq::end(self) }
}

impl<'a> ser::SerializeTupleStruct for SeqSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), WriterError> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<(), WriterError> { ser::SerializeSeq::end(self) }
}

impl<'a> ser::SerializeTupleVariant for SeqSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), WriterError> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<(), WriterError> {
		self.writer.write_byte(b'}');
		ser::SerializeSeq::end(self)
	}
}

struct MapSerializer<'a> {
	writer: &'a mut JsonWriter,
	first: bool,
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), WriterError> {
		if !self.first {
			self.writer.write_byte(b',');
		}
		self.first = false;
		key.serialize(Serializer { writer: self.writer })
	}

	fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), WriterError> {
		self.writer.write_byte(b':');
		value.serialize(Serializer { writer: self.writer })
	}

	fn end(self) -> Result<(), WriterError> {
		self.writer.write_byte(b'}');
		Ok(())
	}
}

impl<'a> ser::SerializeStruct for MapSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), WriterError> {
		ser::SerializeMap::serialize_key(self, key)?;
		ser::SerializeMap::serialize_value(self, value)
	}

	fn end(self) -> Result<(), WriterError> { ser::SerializeMap::end(self) }
}

struct StructVariantSerializer<'a> {
	writer: &'a mut JsonWriter,
	first: bool,
}

impl<'a> ser::SerializeStructVariant for StructVariantSerializer<'a> {
	type Error = WriterError;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), WriterError> {
		ser::SerializeMap::serialize_key(self, key)?;
		ser::SerializeMap::serialize_value(self, value)
	}

	fn end(self) -> Result<(), WriterError> {
		self.writer.write_byte(b'}');
		ser::SerializeMap::end(self)
	}
}

/// Error type for the JSON writer.
#[derive(Debug)]
pub struct WriterError;

impl fmt::Display for WriterError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("JSON writer error") }
}

impl std::error::Error for WriterError {}

impl ser::Error for WriterError {
	fn custom<T: fmt::Display>(_msg: T) -> Self { WriterError }
}

/// Serialize a value directly to a `BytesMut` buffer.
#[inline]
pub fn to_bytes<T: Serialize>(value: &T) -> Result<BytesMut, WriterError> {
	let mut writer = JsonWriter::with_capacity(8192);
	writer.serialize_value(value)?;
	Ok(writer.into_bytes())
}

/// Serialize a value to a `String`.
#[inline]
pub fn to_string_buf<T: Serialize>(value: &T) -> Result<String, WriterError> {
	let bytes = to_bytes(value)?;
	// SAFETY: we only wrote valid UTF-8 (JSON is UTF-8)
	Ok(unsafe { String::from_utf8_unchecked(bytes.to_vec()) })
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn test_serialize_primitives() {
		let mut w = JsonWriter::with_capacity(64);
		w.serialize_value(&true).unwrap();
		assert_eq!(w.as_bytes(), b"true");

		let mut w = JsonWriter::with_capacity(64);
		w.serialize_value(&42u64).unwrap();
		assert_eq!(w.as_bytes(), b"42");

		let mut w = JsonWriter::with_capacity(64);
		w.serialize_value(&"hello").unwrap();
		assert_eq!(w.as_bytes(), b"\"hello\"");
	}

	#[test]
	fn test_serialize_map() {
		let mut w = JsonWriter::with_capacity(64);
		w.serialize_value(&json!({"a": 1, "b": "two"})).unwrap();
		let result = String::from_utf8(w.into_bytes().to_vec()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
		assert_eq!(parsed, json!({"a": 1, "b": "two"}));
	}

	#[test]
	fn test_serialize_nested() {
		let mut w = JsonWriter::with_capacity(256);
		w.serialize_value(&json!({
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
		let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
		assert_eq!(
			parsed["rooms"]["join"]["!room:example.com"]["timeline"]["events"][0]["type"],
			"m.room.message"
		);
	}

	#[test]
	fn test_write_raw() {
		let mut w = JsonWriter::with_capacity(64);
		w.write_raw(r#"{"key":"value"}"#);
		assert_eq!(w.as_bytes(), r#"{"key":"value"}"#);
	}

	#[test]
	fn test_to_bytes_matches_serde_json() {
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

		let serde_bytes = serde_json::to_vec(&value).unwrap();
		let custom_bytes = to_bytes(&value).unwrap();

		let serde_parsed: serde_json::Value = serde_json::from_slice(&serde_bytes).unwrap();
		let custom_parsed: serde_json::Value = serde_json::from_slice(&custom_bytes).unwrap();
		assert_eq!(serde_parsed, custom_parsed);
	}
}

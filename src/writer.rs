//! Streaming JSON writer that serializes directly to a byte buffer.

use bytes::{BufMut, BytesMut};
use simd_json::serde::ser::{self, Serialize};

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

	#[inline]
	pub fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<(), simd_json::Error> {
		let ser = Serializer { writer: self };
		value.serialize(ser)
	}

	#[inline]
	pub fn write_raw(&mut self, raw: &str) { self.buf.put_slice(raw.as_bytes()); }

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

struct Serializer<'a> {
	writer: &'a mut JsonWriter,
}

impl<'a> ser::Serializer for Serializer<'a> {
	type Error = simd_json::Error;
	type Ok = ();
	type SerializeMap = MapSerializer<'a>;
	type SerializeSeq = SeqSerializer<'a>;
	type SerializeStruct = MapSerializer<'a>;
	type SerializeStructVariant = StructVariantSerializer<'a>;
	type SerializeTuple = SeqSerializer<'a>;
	type SerializeTupleStruct = SeqSerializer<'a>;
	type SerializeTupleVariant = SeqSerializer<'a>;

	fn serialize_bool(self, v: bool) -> Result<(), simd_json::Error> {
		self.writer.write_raw(if v { "true" } else { "false" });
		Ok(())
	}

	fn serialize_i8(self, v: i8) -> Result<(), simd_json::Error> {
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i16(self, v: i16) -> Result<(), simd_json::Error> {
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i32(self, v: i32) -> Result<(), simd_json::Error> {
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i64(self, v: i64) -> Result<(), simd_json::Error> {
		let mut buf = itoa::Buffer::new();
		self.writer.write_raw(buf.format(v));
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<(), simd_json::Error> {
		self.serialize_u64(u64::from(v))
	}

	fn serialize_u16(self, v: u16) -> Result<(), simd_json::Error> {
		self.serialize_u64(u64::from(v))
	}

	fn serialize_u32(self, v: u32) -> Result<(), simd_json::Error> {
		self.serialize_u64(u64::from(v))
	}

	fn serialize_u64(self, v: u64) -> Result<(), simd_json::Error> {
		let mut buf = itoa::Buffer::new();
		self.writer.write_raw(buf.format(v));
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<(), simd_json::Error> {
		self.serialize_f64(f64::from(v))
	}

	fn serialize_f64(self, v: f64) -> Result<(), simd_json::Error> {
		let mut buf = ryu::Buffer::new();
		self.writer.write_raw(buf.format(v));
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<(), simd_json::Error> {
		let mut s = String::with_capacity(6);
		s.push(v);
		self.writer.write_escaped_string(&s);
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<(), simd_json::Error> {
		self.writer.write_escaped_string(v);
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<(), simd_json::Error> {
		use ser::SerializeSeq;
		let mut seq = self.serialize_seq(Some(v.len()))?;
		for byte in v {
			seq.serialize_element(byte)?;
		}
		seq.end()
	}

	fn serialize_none(self) -> Result<(), simd_json::Error> {
		self.writer.write_raw("null");
		Ok(())
	}

	fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), simd_json::Error> {
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<(), simd_json::Error> {
		self.writer.write_raw("null");
		Ok(())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<(), simd_json::Error> {
		self.writer.write_raw("null");
		Ok(())
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<(), simd_json::Error> {
		self.writer.write_escaped_string(variant);
		Ok(())
	}

	fn serialize_newtype_struct<T: ?Sized + Serialize>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<(), simd_json::Error> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized + Serialize>(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<(), simd_json::Error> {
		self.writer.write_byte(b'{');
		self.writer.write_escaped_string(variant);
		self.writer.write_byte(b':');
		value.serialize(Serializer { writer: self.writer })?;
		self.writer.write_byte(b'}');
		Ok(())
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<SeqSerializer<'a>, simd_json::Error> {
		self.writer.write_byte(b'[');
		Ok(SeqSerializer { writer: self.writer, count: 0 })
	}

	fn serialize_tuple(self, len: usize) -> Result<SeqSerializer<'a>, simd_json::Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<SeqSerializer<'a>, simd_json::Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<SeqSerializer<'a>, simd_json::Error> {
		self.writer.write_byte(b'{');
		self.writer.write_escaped_string(variant);
		self.writer.write_byte(b':');
		self.serialize_seq(Some(len))
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<MapSerializer<'a>, simd_json::Error> {
		self.writer.write_byte(b'{');
		Ok(MapSerializer { writer: self.writer, first: true })
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<MapSerializer<'a>, simd_json::Error> {
		self.serialize_map(None)
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<StructVariantSerializer<'a>, simd_json::Error> {
		self.writer.write_byte(b'{');
		self.writer.write_escaped_string(variant);
		self.writer.write_byte(b':');
		self.writer.write_byte(b'{');
		Ok(StructVariantSerializer { writer: self.writer, first: true })
	}
}

struct SeqSerializer<'a> {
	writer: &'a mut JsonWriter,
	count: usize,
}

impl ser::SerializeSeq for SeqSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_element<T: ?Sized + Serialize>(
		&mut self,
		value: &T,
	) -> Result<(), simd_json::Error> {
		if self.count > 0 {
			self.writer.write_byte(b',');
		}
		self.count += 1;
		value.serialize(Serializer { writer: self.writer })
	}

	fn end(self) -> Result<(), simd_json::Error> {
		self.writer.write_byte(b']');
		Ok(())
	}
}

impl ser::SerializeTuple for SeqSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_element<T: ?Sized + Serialize>(
		&mut self,
		value: &T,
	) -> Result<(), simd_json::Error> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<(), simd_json::Error> { ser::SerializeSeq::end(self) }
}

impl ser::SerializeTupleStruct for SeqSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(
		&mut self,
		value: &T,
	) -> Result<(), simd_json::Error> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<(), simd_json::Error> { ser::SerializeSeq::end(self) }
}

impl ser::SerializeTupleVariant for SeqSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(
		&mut self,
		value: &T,
	) -> Result<(), simd_json::Error> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<(), simd_json::Error> {
		self.writer.write_byte(b'}');
		ser::SerializeSeq::end(self)
	}
}

struct MapSerializer<'a> {
	writer: &'a mut JsonWriter,
	first: bool,
}

impl ser::SerializeMap for MapSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), simd_json::Error> {
		if !self.first {
			self.writer.write_byte(b',');
		}
		self.first = false;
		key.serialize(Serializer { writer: self.writer })
	}

	fn serialize_value<T: ?Sized + Serialize>(
		&mut self,
		value: &T,
	) -> Result<(), simd_json::Error> {
		self.writer.write_byte(b':');
		value.serialize(Serializer { writer: self.writer })
	}

	fn end(self) -> Result<(), simd_json::Error> {
		self.writer.write_byte(b'}');
		Ok(())
	}
}

impl ser::SerializeStruct for MapSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), simd_json::Error> {
		ser::SerializeMap::serialize_key(self, key)?;
		ser::SerializeMap::serialize_value(self, value)
	}

	fn end(self) -> Result<(), simd_json::Error> { ser::SerializeMap::end(self) }
}

struct StructVariantSerializer<'a> {
	writer: &'a mut JsonWriter,
	first: bool,
}

impl ser::SerializeStructVariant for StructVariantSerializer<'_> {
	type Error = simd_json::Error;
	type Ok = ();

	fn serialize_field<T: ?Sized + Serialize>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), simd_json::Error> {
		if !self.first {
			self.writer.write_byte(b',');
		}
		self.first = false;
		self.writer.write_escaped_string(key);
		self.writer.write_byte(b':');
		value.serialize(Serializer { writer: self.writer })
	}

	fn end(self) -> Result<(), simd_json::Error> {
		self.writer.write_byte(b'}');
		self.writer.write_byte(b'}');
		Ok(())
	}
}

/// Serialize a value directly to a `BytesMut` buffer.
#[inline]
pub fn to_bytes<T: Serialize>(value: &T) -> Result<BytesMut, simd_json::Error> {
	let mut writer = JsonWriter::with_capacity(8192);
	writer.serialize_value(value)?;
	Ok(writer.into_bytes())
}

#[cfg(test)]
mod tests {
	use simd_json::json;

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
		let mut parsed_input = result.into_bytes();
		let parsed: simd_json::OwnedValue = simd_json::from_slice(&mut parsed_input).unwrap();
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
		w.write_raw(r#"{"key":"value"}"#);
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

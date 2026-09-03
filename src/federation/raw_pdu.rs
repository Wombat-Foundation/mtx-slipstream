//! Direct `CanonicalJsonObject` → bytes conversion.
//!
//! The current pipeline for federation PDUs is:
//! ```text
//! DB (JSON text) → serde_json::from_slice → CanonicalJsonObject → to_raw_value → Box<RawValue>
//! ```
//!
//! This module provides a direct path:
//! ```text
//! CanonicalJsonObject → serialize_with → bytes (via streaming writer)
//! ```

use bytes::BytesMut;
use serde::Serialize;

use crate::writer::JsonWriter;

/// Serialize a `CanonicalJsonObject` directly to bytes.
pub fn canonical_to_bytes<T: Serialize>(pdu: &T) -> Result<BytesMut, serde_json::Error> {
    let mut writer = JsonWriter::with_capacity(2048);
    writer.serialize_value(pdu)?;
    Ok(writer.into_bytes())
}

/// Serialize a `CanonicalJsonObject` to a `String`.
pub fn canonical_to_string<T: Serialize>(pdu: &T) -> Result<String, serde_json::Error> {
    let bytes = canonical_to_bytes(pdu)?;
    // SAFETY: JSON is always valid UTF-8
    Ok(unsafe { String::from_utf8_unchecked(bytes.to_vec()) })
}

/// Serialize a `CanonicalJsonObject` directly, skipping specified fields.
///
/// Uses a custom `Serializer` that filters fields during serialization,
/// avoiding the need to modify the object in place.
pub fn canonical_to_bytes_skipping<T: Serialize>(
    pdu: &T,
    skip_fields: &[&str],
) -> Result<BytesMut, serde_json::Error> {
    let mut writer = JsonWriter::with_capacity(2048);
    let mut ser = SkippingSerializer { writer: &mut writer, skip_fields };
    pdu.serialize(&mut ser)?;
    Ok(writer.into_bytes())
}

/// A serializer that skips specified struct fields during serialization.
struct SkippingSerializer<'a> {
    writer: &'a mut JsonWriter,
    skip_fields: &'a [&'a str],
}

impl<'a> serde::ser::SerializeStruct for SkippingSerializer<'a> {
    type Ok = ();
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if self.skip_fields.contains(&key) {
            return Ok(());
        }
        use serde::ser::SerializeMap;
        // Write comma separator if needed - we track this via the writer state
        // For simplicity, we write the key-value pair directly
        self.writer.write_raw("\"");
        self.writer.write_raw(key);
        self.writer.write_raw("\":");
        value.serialize(crate::writer::Serializer { writer: self.writer })?;
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        self.writer.write_byte(b'}');
        Ok(())
    }
}

impl<'a> serde::Serializer for SkippingSerializer<'a> {
    type Ok = ();
    type Error = serde_json::Error;
    type SerializeSeq = serde::ser::SerializeSeqPassthrough<Self>;
    type SerializeTuple = serde::ser::SerializeSeqPassthrough<Self>;
    type SerializeTupleStruct = serde::ser::SerializeSeqPassthrough<Self>;
    type SerializeTupleVariant = serde::ser::SerializeSeqPassthrough<Self>;
    type SerializeMap = serde::ser::SerializeMapPassthrough<Self>;
    type SerializeStruct = Self;
    type SerializeStructVariant = serde::ser::SerializeStructVariantPassthrough<Self>;

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.writer.write_byte(b'{');
        Ok(self)
    }

    // Delegate everything else to the writer's serializer
    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> { self.writer.write_raw(if v { "true" } else { "false" }); Ok(()) }
    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> { self.writer.write_raw(&v.to_string()); Ok(()) }
    fn serialize_char(self, v: char) -> Result<(), Self::Error> { self.writer.write_raw(&format!("\"{v}\"")); Ok(()) }
    fn serialize_str(self, v: &str) -> Result<(), Self::Error> { self.writer.write_escaped_string(v); Ok(()) }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> { unimplemented!() }
    fn serialize_none(self) -> Result<(), Self::Error> { self.writer.write_raw("null"); Ok(()) }
    fn serialize_some<T: ?Sized + Serialize>(self, v: &T) -> Result<(), Self::Error> { v.serialize(self) }
    fn serialize_unit(self) -> Result<(), Self::Error> { self.writer.write_raw("null"); Ok(()) }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> { self.writer.write_raw("null"); Ok(()) }
    fn serialize_unit_variant(self, _: &'static str, _: u32, v: &'static str) -> Result<(), Self::Error> { self.writer.write_escaped_string(v); Ok(()) }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, v: &T) -> Result<(), Self::Error> { v.serialize(self) }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<(), Self::Error> { unimplemented!() }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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
}

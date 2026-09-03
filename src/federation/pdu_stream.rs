//! Streaming PDU serialization for federation responses.
//!
//! Instead of materializing all PDUs into a `Vec<Box<RawValue>>` before
//! serializing, this module provides a `PduStreamWriter` that serializes
//! PDUs incrementally into a growing byte buffer.

use bytes::{BufMut, BytesMut};
use serde::Serialize;

use crate::writer::JsonWriter;

/// A streaming writer for arrays of serialized PDUs.
pub struct PduStreamWriter {
	buf: BytesMut,
	count: usize,
}

impl PduStreamWriter {
	#[inline]
	pub fn with_capacity(estimated_pdus: usize) -> Self {
		let cap = estimated_pdus * 2048;
		let mut buf = BytesMut::with_capacity(cap);
		buf.put_u8(b'[');
		Self { buf, count: 0 }
	}

	/// Write a single pre-serialized PDU (raw JSON bytes).
	#[inline]
	pub fn write_raw_pdu(&mut self, pdu_json: &str) {
		if self.count > 0 {
			self.buf.put_u8(b',');
		}
		self.buf.put_slice(pdu_json.as_bytes());
		self.count += 1;
	}

	/// Write a serializable PDU value.
	pub fn write_pdu<T: Serialize>(&mut self, pdu: &T) -> Result<(), simd_json::Error> {
		if self.count > 0 {
			self.buf.put_u8(b',');
		}
		let mut writer = JsonWriter::with_capacity(2048);
		writer.serialize_value(pdu)?;
		self.buf.put_slice(writer.as_bytes());
		self.count += 1;
		Ok(())
	}

	#[inline]
	pub fn finish(mut self) -> BytesMut {
		self.buf.put_u8(b']');
		self.buf
	}

	#[inline]
	pub fn len(&self) -> usize { self.count }

	#[inline]
	pub fn is_empty(&self) -> bool { self.count == 0 }
}

/// Streaming writer for federation responses with `state` and `auth_chain`
/// arrays.
pub struct FederationResponseWriter {
	buf: BytesMut,
	state_count: usize,
	auth_chain_count: usize,
	phase: ResponsePhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponsePhase {
	State,
	AuthChain,
}

impl FederationResponseWriter {
	pub fn with_capacity(estimated_state_pdus: usize, estimated_auth_chain_pdus: usize) -> Self {
		let cap = (estimated_state_pdus + estimated_auth_chain_pdus) * 2048 + 128;
		let mut buf = BytesMut::with_capacity(cap);
		buf.put_slice(b"{\"state\":[");
		Self {
			buf,
			state_count: 0,
			auth_chain_count: 0,
			phase: ResponsePhase::State,
		}
	}

	pub fn write_raw_state_pdu(&mut self, pdu_json: &str) {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		if self.state_count > 0 {
			self.buf.put_u8(b',');
		}
		self.buf.put_slice(pdu_json.as_bytes());
		self.state_count += 1;
	}

	pub fn write_state_pdu<T: Serialize>(&mut self, pdu: &T) -> Result<(), simd_json::Error> {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		if self.state_count > 0 {
			self.buf.put_u8(b',');
		}
		let mut writer = JsonWriter::with_capacity(2048);
		writer.serialize_value(pdu)?;
		self.buf.put_slice(writer.as_bytes());
		self.state_count += 1;
		Ok(())
	}

	pub fn begin_auth_chain(&mut self) {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		self.buf.put_slice(b"],\"auth_chain\":[");
		self.phase = ResponsePhase::AuthChain;
	}

	pub fn write_raw_auth_chain_pdu(&mut self, pdu_json: &str) {
		debug_assert_eq!(self.phase, ResponsePhase::AuthChain);
		if self.auth_chain_count > 0 {
			self.buf.put_u8(b',');
		}
		self.buf.put_slice(pdu_json.as_bytes());
		self.auth_chain_count += 1;
	}

	pub fn write_auth_chain_pdu<T: Serialize>(
		&mut self,
		pdu: &T,
	) -> Result<(), simd_json::Error> {
		debug_assert_eq!(self.phase, ResponsePhase::AuthChain);
		if self.auth_chain_count > 0 {
			self.buf.put_u8(b',');
		}
		let mut writer = JsonWriter::with_capacity(2048);
		writer.serialize_value(pdu)?;
		self.buf.put_slice(writer.as_bytes());
		self.auth_chain_count += 1;
		Ok(())
	}

	pub fn finish(mut self) -> BytesMut {
		self.buf
			.put_slice(b"],\"event\":null,\"members_omitted\":false}");
		self.buf
	}

	#[inline]
	pub fn state_len(&self) -> usize { self.state_count }

	#[inline]
	pub fn auth_chain_len(&self) -> usize { self.auth_chain_count }
}

#[cfg(test)]
mod tests {
	use simd_json::{OwnedValue, json, prelude::*};

	use super::*;

	#[test]
	fn test_pdu_stream_writer() {
		let mut stream = PduStreamWriter::with_capacity(3);
		stream.write_raw_pdu(r#"{"event_id":"$a","type":"m.room.create"}"#);
		stream.write_raw_pdu(r#"{"event_id":"$b","type":"m.room.member"}"#);
		stream.write_raw_pdu(r#"{"event_id":"$c","type":"m.room.message"}"#);
		let bytes = stream.finish();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert!(parsed.is_array());
		assert_eq!(parsed.as_array().unwrap().len(), 3);
	}

	#[test]
	fn test_pdu_stream_writer_serializable() {
		let mut stream = PduStreamWriter::with_capacity(2);
		stream.write_pdu(&json!({"event_id": "$a"})).unwrap();
		stream.write_pdu(&json!({"event_id": "$b"})).unwrap();
		let bytes = stream.finish();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed[0]["event_id"], "$a");
		assert_eq!(parsed[1]["event_id"], "$b");
	}

	#[test]
	fn test_federation_response_writer() {
		let mut writer = FederationResponseWriter::with_capacity(2, 1);
		writer.write_raw_state_pdu(r#"{"event_id":"$s1"}"#);
		writer.write_raw_state_pdu(r#"{"event_id":"$s2"}"#);
		writer.begin_auth_chain();
		writer.write_raw_auth_chain_pdu(r#"{"event_id":"$a1"}"#);
		let bytes = writer.finish();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed["state"][0]["event_id"], "$s1");
		assert_eq!(parsed["state"][1]["event_id"], "$s2");
		assert_eq!(parsed["auth_chain"][0]["event_id"], "$a1");
	}

	#[test]
	fn test_empty_stream() {
		let stream = PduStreamWriter::with_capacity(0);
		let bytes = stream.finish();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed, json!([]));
	}
}

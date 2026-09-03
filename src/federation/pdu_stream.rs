//! Streaming PDU serialization for federation responses.
//!
//! Instead of materializing all PDUs into a `Vec<Box<RawValue>>` before
//! serializing, this module provides a `PduStreamWriter` that serializes
//! PDUs incrementally into a growing byte buffer.

use bytes::{BufMut, BytesMut};
use simd_json::OwnedValue;

use crate::writer::JsonWriter;

/// A streaming writer for arrays of serialized PDUs.
pub struct PduStreamWriter {
	buf: BytesMut,
	count: usize,
}

impl PduStreamWriter {
	/// Create a new stream writer with pre-allocated capacity for
	/// `estimated_pdus` entries.
	///
	/// Each PDU is estimated at 2048 bytes. The buffer starts with an opening
	/// `[` bracket.
	#[inline]
	#[must_use]
	pub fn with_capacity(estimated_pdus: usize) -> Self {
		let cap = estimated_pdus.saturating_mul(2048);
		let mut buf = BytesMut::with_capacity(cap);
		buf.put_u8(b'[');
		Self { buf, count: 0 }
	}

	/// Write a single pre-serialized PDU (raw JSON bytes).
	///
	/// A comma separator is inserted before the PDU if the stream is not
	/// empty.
	#[inline]
	pub fn write_raw_pdu(&mut self, pdu_json: &str) {
		if self.count > 0 {
			self.buf.put_u8(b',');
		}
		self.buf.put_slice(pdu_json.as_bytes());
		self.count = self.count.saturating_add(1);
	}

	/// Serialize an `OwnedValue` PDU into the stream.
	///
	/// The value is serialized via [`JsonWriter`] and a comma separator is
	/// inserted before the PDU if the stream is not empty.
	///
	/// # Errors
	///
	/// Returns `simd_json::Error` if the value cannot be serialized.
	pub fn write_pdu(&mut self, pdu: &OwnedValue) -> Result<(), simd_json::Error> {
		if self.count > 0 {
			self.buf.put_u8(b',');
		}
		let mut writer = JsonWriter::with_capacity(2048);
		writer.write_value(pdu)?;
		self.buf.put_slice(writer.as_bytes());
		self.count = self.count.saturating_add(1);
		Ok(())
	}

	/// Finalize the stream by appending a closing `]` bracket and returning
	/// the accumulated buffer.
	#[inline]
	#[must_use]
	pub fn finish(mut self) -> BytesMut {
		self.buf.put_u8(b']');
		self.buf
	}

	/// Returns the number of PDUs written to the stream.
	#[inline]
	#[must_use]
	pub fn len(&self) -> usize { self.count }

	/// Returns `true` if no PDUs have been written.
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool { self.count == 0 }
}

/// Streaming writer for federation responses with `state` and `auth_chain`
/// arrays.
///
/// Produces a JSON object of the form:
/// ```json
/// {"state":[...],"auth_chain":[...],"event":null,"members_omitted":false}
/// ```
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
	/// Create a new response writer with pre-allocated capacity for the
	/// given number of state and `auth_chain` PDUs.
	///
	/// Each PDU is estimated at 2048 bytes, plus 128 bytes for the JSON
	/// envelope (`{"state":[` ... `]}` etc.).
	#[must_use]
	pub fn with_capacity(estimated_state_pdus: usize, estimated_auth_chain_pdus: usize) -> Self {
		let cap = estimated_state_pdus
			.saturating_add(estimated_auth_chain_pdus)
			.saturating_mul(2048)
			.saturating_add(128);
		let mut buf = BytesMut::with_capacity(cap);
		buf.put_slice(b"{\"state\":[");
		Self {
			buf,
			state_count: 0,
			auth_chain_count: 0,
			phase: ResponsePhase::State,
		}
	}

	/// Write a pre-serialized state PDU (raw JSON bytes).
	///
	/// # Panics
	///
	/// Panics if called after [`begin_auth_chain`].
	pub fn write_raw_state_pdu(&mut self, pdu_json: &str) {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		if self.state_count > 0 {
			self.buf.put_u8(b',');
		}
		self.buf.put_slice(pdu_json.as_bytes());
		self.state_count = self.state_count.saturating_add(1);
	}

	/// Serialize an `OwnedValue` state PDU into the stream.
	///
	/// # Errors
	///
	/// Returns `simd_json::Error` if the value cannot be serialized.
	///
	/// # Panics
	///
	/// Panics if called after [`begin_auth_chain`].
	pub fn write_state_pdu(&mut self, pdu: &OwnedValue) -> Result<(), simd_json::Error> {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		if self.state_count > 0 {
			self.buf.put_u8(b',');
		}
		let mut writer = JsonWriter::with_capacity(2048);
		writer.write_value(pdu)?;
		self.buf.put_slice(writer.as_bytes());
		self.state_count = self.state_count.saturating_add(1);
		Ok(())
	}

	/// Transition from the state phase to the `auth_chain` phase by emitting
	/// `],"auth_chain":[`.
	///
	/// # Panics
	///
	/// Panics if already in the `auth_chain` phase.
	pub fn begin_auth_chain(&mut self) {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		self.buf.put_slice(b"],\"auth_chain\":[");
		self.phase = ResponsePhase::AuthChain;
	}

	/// Write a pre-serialized `auth_chain` PDU (raw JSON bytes).
	///
	/// # Panics
	///
	/// Panics if called before [`begin_auth_chain`].
	pub fn write_raw_auth_chain_pdu(&mut self, pdu_json: &str) {
		debug_assert_eq!(self.phase, ResponsePhase::AuthChain);
		if self.auth_chain_count > 0 {
			self.buf.put_u8(b',');
		}
		self.buf.put_slice(pdu_json.as_bytes());
		self.auth_chain_count = self.auth_chain_count.saturating_add(1);
	}

	/// Serialize an `OwnedValue` `auth_chain` PDU into the stream.
	///
	/// # Errors
	///
	/// Returns `simd_json::Error` if the value cannot be serialized.
	///
	/// # Panics
	///
	/// Panics if called before [`begin_auth_chain`].
	pub fn write_auth_chain_pdu(&mut self, pdu: &OwnedValue) -> Result<(), simd_json::Error> {
		debug_assert_eq!(self.phase, ResponsePhase::AuthChain);
		if self.auth_chain_count > 0 {
			self.buf.put_u8(b',');
		}
		let mut writer = JsonWriter::with_capacity(2048);
		writer.write_value(pdu)?;
		self.buf.put_slice(writer.as_bytes());
		self.auth_chain_count = self.auth_chain_count.saturating_add(1);
		Ok(())
	}

	/// Finalize the response by appending the closing JSON envelope and
	/// returning the accumulated buffer.
	#[must_use]
	pub fn finish(mut self) -> BytesMut {
		self.buf
			.put_slice(b"],\"event\":null,\"members_omitted\":false}");
		self.buf
	}

	/// Returns the number of state PDUs written.
	#[inline]
	#[must_use]
	pub fn state_len(&self) -> usize { self.state_count }

	/// Returns the number of `auth_chain` PDUs written.
	#[inline]
	#[must_use]
	pub fn auth_chain_len(&self) -> usize { self.auth_chain_count }
}

#[cfg(test)]
#[coverage(off)]
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
	fn test_pdu_stream_writer_owned_value() {
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

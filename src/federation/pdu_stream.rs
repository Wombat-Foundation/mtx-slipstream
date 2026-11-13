//! Streaming PDU serialization for federation responses.
//!
//! Instead of materializing all PDUs into a `Vec<Box<RawValue>>` before
//! serializing, this module provides a `PduStreamWriter` that serializes
//! PDUs incrementally into a growing byte buffer.

use crate::federation::raw_pdu::{parse_pdu_json, write_canonical_into};
use crate::slipstream_json as sjson;
use bytes::{BufMut, BytesMut};

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
		Self {
			buf,
			count: 0,
		}
	}

	/// Write a PDU from raw JSON text.
	///
	/// The input is parsed strictly (rejecting duplicate keys) and serialized
	/// through the canonical writer, so floats and out-of-range integers in
	/// the signed portion are rejected instead of being emitted non-canonically.
	/// A comma separator is inserted before the PDU if the stream is not empty.
	///
	/// # Errors
	///
	/// Returns `io::Error` if the input is malformed, contains a duplicate
	/// key, or has a float or out-of-range integer in the signed portion.
	pub fn write_raw_pdu(&mut self, pdu_json: &str) -> std::io::Result<()> {
		let pdu = parse_pdu_json(pdu_json.as_bytes())
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
		self.write_pdu(&pdu)
	}

	/// Serialize an `OwnedValue` PDU into the stream.
	///
	/// The value is serialized by the canonical writer and a comma separator
	/// is inserted before the PDU if the stream is not empty. The stream is
	/// unchanged when serialization fails.
	///
	/// # Errors
	///
	/// Returns `io::Error` if a number in the signed portion is a float or an
	/// integer outside the canonical safe range.
	pub fn write_pdu(&mut self, pdu: &sjson::OwnedValue) -> std::io::Result<()> {
		self.write_canonical(pdu)
	}

	fn write_canonical(&mut self, pdu: &sjson::OwnedValue) -> std::io::Result<()> {
		let start = self.buf.len();
		if self.count > 0 {
			self.buf.put_u8(b',');
		}
		if let Err(error) = write_canonical_into(pdu, &mut self.buf) {
			self.buf.truncate(start);
			return Err(error);
		}
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
	pub fn len(&self) -> usize {
		self.count
	}

	/// Returns `true` if no PDUs have been written.
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.count == 0
	}
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

	/// Write a state PDU from raw JSON text.
	///
	/// The input is parsed strictly and canonicalized; see
	/// [`PduStreamWriter::write_raw_pdu`] for the rejection rules.
	///
	/// # Errors
	///
	/// Returns `io::Error` if the input is malformed, contains a duplicate
	/// key, or has a float or out-of-range integer in the signed portion.
	///
	/// # Panics
	///
	/// Panics if called after [`Self::begin_auth_chain`].
	pub fn write_raw_state_pdu(&mut self, pdu_json: &str) -> std::io::Result<()> {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		let pdu = parse_pdu_json(pdu_json.as_bytes())
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
		self.write_state_pdu(&pdu)
	}

	/// Serialize an `OwnedValue` state PDU into the stream.
	///
	/// The stream is unchanged when serialization fails.
	///
	/// # Errors
	///
	/// Returns `io::Error` if a number in the signed portion is a float or an
	/// integer outside the canonical safe range.
	///
	/// # Panics
	///
	/// Panics if called after [`Self::begin_auth_chain`].
	pub fn write_state_pdu(&mut self, pdu: &sjson::OwnedValue) -> std::io::Result<()> {
		debug_assert_eq!(self.phase, ResponsePhase::State);
		self.write_state_canonical(pdu)
	}

	fn write_state_canonical(&mut self, pdu: &sjson::OwnedValue) -> std::io::Result<()> {
		let start = self.buf.len();
		if self.state_count > 0 {
			self.buf.put_u8(b',');
		}
		if let Err(error) = write_canonical_into(pdu, &mut self.buf) {
			self.buf.truncate(start);
			return Err(error);
		}
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

	/// Write an `auth_chain` PDU from raw JSON text.
	///
	/// The input is parsed strictly and canonicalized; see
	/// [`PduStreamWriter::write_raw_pdu`] for the rejection rules.
	///
	/// # Errors
	///
	/// Returns `io::Error` if the input is malformed, contains a duplicate
	/// key, or has a float or out-of-range integer in the signed portion.
	///
	/// # Panics
	///
	/// Panics if called before [`Self::begin_auth_chain`].
	pub fn write_raw_auth_chain_pdu(&mut self, pdu_json: &str) -> std::io::Result<()> {
		debug_assert_eq!(self.phase, ResponsePhase::AuthChain);
		let pdu = parse_pdu_json(pdu_json.as_bytes())
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
		self.write_auth_chain_pdu(&pdu)
	}

	/// Serialize an `OwnedValue` `auth_chain` PDU into the stream.
	///
	/// The stream is unchanged when serialization fails.
	///
	/// # Errors
	///
	/// Returns `io::Error` if a number in the signed portion is a float or an
	/// integer outside the canonical safe range.
	///
	/// # Panics
	///
	/// Panics if called before [`Self::begin_auth_chain`].
	pub fn write_auth_chain_pdu(&mut self, pdu: &sjson::OwnedValue) -> std::io::Result<()> {
		debug_assert_eq!(self.phase, ResponsePhase::AuthChain);
		self.write_auth_chain_canonical(pdu)
	}

	fn write_auth_chain_canonical(&mut self, pdu: &sjson::OwnedValue) -> std::io::Result<()> {
		let start = self.buf.len();
		if self.auth_chain_count > 0 {
			self.buf.put_u8(b',');
		}
		if let Err(error) = write_canonical_into(pdu, &mut self.buf) {
			self.buf.truncate(start);
			return Err(error);
		}
		self.auth_chain_count = self.auth_chain_count.saturating_add(1);
		Ok(())
	}

	/// Finalize the response by appending the closing JSON envelope and
	/// returning the accumulated buffer.
	#[must_use]
	pub fn finish(mut self) -> BytesMut {
		self.buf.put_slice(b"],\"event\":null,\"members_omitted\":false}");
		self.buf
	}

	/// Returns the number of state PDUs written.
	#[inline]
	#[must_use]
	pub fn state_len(&self) -> usize {
		self.state_count
	}

	/// Returns the number of `auth_chain` PDUs written.
	#[inline]
	#[must_use]
	pub fn auth_chain_len(&self) -> usize {
		self.auth_chain_count
	}
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
	use sjson::json;

	use super::*;

	#[test]
	fn test_pdu_stream_writer() {
		let mut stream = PduStreamWriter::with_capacity(3);
		stream.write_raw_pdu(r#"{"event_id":"$a","type":"m.room.create"}"#).unwrap();
		stream.write_raw_pdu(r#"{"event_id":"$b","type":"m.room.member"}"#).unwrap();
		stream.write_raw_pdu(r#"{"event_id":"$c","type":"m.room.message"}"#).unwrap();
		let bytes = stream.finish();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert!(parsed.is_array());
		assert_eq!(parsed.as_array().unwrap().len(), 3);
	}

	#[test]
	fn test_pdu_stream_writer_owned_value() {
		let mut stream = PduStreamWriter::with_capacity(2);
		stream.write_pdu(&json!({"event_id": "$a"})).unwrap();
		stream.write_pdu(&json!({"event_id": "$b"})).unwrap();
		let bytes = stream.finish();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		let items = parsed.as_array().unwrap();
		assert_eq!(items[0]["event_id"].as_str(), Some("$a"));
		assert_eq!(items[1]["event_id"].as_str(), Some("$b"));
	}

	#[test]
	fn write_pdu_rejects_non_canonical_numbers_without_changing_stream() {
		let float: sjson::OwnedValue =
			sjson::to_owned_value(br#"{"content":{"ratio":1.5}}"#).unwrap();
		let big: sjson::OwnedValue =
			sjson::to_owned_value(br#"{"depth":9007199254740992}"#).unwrap();

		let mut stream = PduStreamWriter::with_capacity(1);
		assert!(stream.write_pdu(&float).is_err());
		assert!(stream.write_pdu(&big).is_err());
		assert!(stream.is_empty());
		stream.write_pdu(&json!({"event_id":"$ok"})).unwrap();
		assert_eq!(stream.len(), 1);

		let mut writer = FederationResponseWriter::with_capacity(1, 1);
		assert!(writer.write_state_pdu(&float).is_err());
		assert_eq!(writer.state_len(), 0);
		writer.begin_auth_chain();
		assert!(writer.write_auth_chain_pdu(&big).is_err());
		assert_eq!(writer.auth_chain_len(), 0);
	}

	#[test]
	fn raw_pdu_rejects_duplicate_keys_and_canonicalizes() {
		let mut stream = PduStreamWriter::with_capacity(1);
		assert!(stream.write_raw_pdu(r#"{"a":1,"a":2}"#).is_err());
		assert!(stream.is_empty());
		assert!(stream.write_raw_pdu(r#"{"z":1,"a":{"b":2}}"#).is_ok());
		let bytes = stream.finish();
		assert_eq!(bytes.as_ref(), br#"[{"a":{"b":2},"z":1}]"#);
	}

	#[test]
	fn unsigned_numbers_are_not_subject_to_signed_rules() {
		let pdu: sjson::OwnedValue =
			sjson::to_owned_value(br#"{"content":{"x":1},"unsigned":{"age":1.5}}"#).unwrap();
		let mut stream = PduStreamWriter::with_capacity(1);
		stream.write_pdu(&pdu).unwrap();
		let bytes = stream.finish();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		let items = parsed.as_array().unwrap();
		assert_eq!(items[0]["unsigned"]["age"].as_f64(), Some(1.5));
	}

	#[test]
	fn test_federation_response_writer() {
		let mut writer = FederationResponseWriter::with_capacity(2, 1);
		writer.write_raw_state_pdu(r#"{"event_id":"$s1"}"#).unwrap();
		writer.write_raw_state_pdu(r#"{"event_id":"$s2"}"#).unwrap();
		writer.begin_auth_chain();
		writer.write_raw_auth_chain_pdu(r#"{"event_id":"$a1"}"#).unwrap();
		let bytes = writer.finish();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		let state = parsed["state"].as_array().unwrap();
		let auth_chain = parsed["auth_chain"].as_array().unwrap();
		assert_eq!(state[0]["event_id"].as_str(), Some("$s1"));
		assert_eq!(state[1]["event_id"].as_str(), Some("$s2"));
		assert_eq!(auth_chain[0]["event_id"].as_str(), Some("$a1"));
	}

	#[test]
	fn test_empty_stream() {
		let stream = PduStreamWriter::with_capacity(0);
		let bytes = stream.finish();
		let input = bytes.to_vec();
		let parsed: sjson::OwnedValue = sjson::to_owned_value(&input).unwrap();
		assert_eq!(parsed, json!([]));
	}
}

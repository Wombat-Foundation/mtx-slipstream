//! Event and JSON signing and verification, backed by Rezzy and
//! consensus-compatible Ed25519 (`ed25519-consensus`).

use alloc::{
	collections::BTreeMap,
	string::{String, ToString},
	vec::Vec,
};
use core::fmt;

use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use ed25519_consensus::{Signature as RawSignature, SigningKey, VerificationKey};
use rezzy::signing::{SignatureVerifier, verify_event_signatures};

use crate::{
	CanonicalJsonObject, OwnedServerName, OwnedServerSigningKeyId, RoomVersionId,
	json::{self, Value},
	sswire::Base64,
};

/// Maximum canonical PDU size in bytes.
const MAX_PDU_BYTES: usize = 65_535;

/// Errors from signing and verification.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
	/// The canonical PDU exceeds the size limit.
	PduSize,
	/// The JSON is malformed or has the wrong shape.
	Json(String),
	/// A key could not be parsed or generated.
	Key(String),
	/// A signature was missing or did not verify.
	Verification(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::PduSize => f.write_str("PDU is larger than the maximum allowed size"),
			Self::Json(e) => write!(f, "invalid JSON: {e}"),
			Self::Key(e) => write!(f, "invalid key: {e}"),
			Self::Verification(e) => write!(f, "signature verification failed: {e}"),
		}
	}
}
impl core::error::Error for Error {}

/// What was verified about an event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verified {
	/// Signatures and content hash both verified.
	All,
	/// Signatures verified but the content hash did not (a redacted event).
	Signatures,
}

/// Public keys for one server, by key ID.
pub type PublicKeySet = BTreeMap<OwnedServerSigningKeyId, Base64>;
/// Public keys by server name.
pub type PublicKeyMap = BTreeMap<OwnedServerName, PublicKeySet>;
/// Signing key IDs each server must provide for an event.
pub type RequiredKeys = BTreeMap<OwnedServerName, Vec<OwnedServerSigningKeyId>>;

/// An Ed25519 signing key pair with a version label.
pub struct Ed25519KeyPair {
	key: SigningKey,
	version: String,
}

impl fmt::Debug for Ed25519KeyPair {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Ed25519KeyPair").field("version", &self.version).finish_non_exhaustive()
	}
}

/// A detached signature.
#[derive(Debug, Eq, PartialEq)]
pub struct Signature(pub Vec<u8>);

impl Signature {
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

impl Ed25519KeyPair {
	/// Generates a new key and returns it as PKCS#8 DER.
	///
	/// # Errors
	///
	/// Currently infallible; the `Result` mirrors ruma's signature.
	pub fn generate() -> Result<Vec<u8>, Error> {
		let key = SigningKey::new(rand_core::OsRng);
		Ok(encode_pkcs8(&key.to_bytes()))
	}

	/// Loads a key from PKCS#8 DER (version 1 or 2).
	///
	/// # Errors
	///
	/// Returns an error if `der` is not a valid Ed25519 private key.
	pub fn from_der(der: &[u8], version: String) -> Result<Self, Error> {
		let key = SigningKey::from(parse_pkcs8_seed(der)?);
		Ok(Self {
			key,
			version,
		})
	}

	#[must_use]
	pub fn public_key(&self) -> [u8; 32] {
		self.key.verification_key().to_bytes()
	}

	#[must_use]
	pub fn version(&self) -> &str {
		&self.version
	}

	#[must_use]
	pub fn sign(&self, message: &[u8]) -> Signature {
		Signature(self.key.sign(message).to_bytes().to_vec())
	}
}

/// PKCS#8 (RFC 8410) prefix for an Ed25519 private key; the 32-byte seed follows.
const PKCS8_PREFIX: [u8; 16] = [
	0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04,
	0x20,
];

/// DER encoding of the Ed25519 algorithm OID (1.3.101.112).
const ED25519_OID: [u8; 3] = [0x2b, 0x65, 0x70];

fn encode_pkcs8(seed: &[u8; 32]) -> Vec<u8> {
	let mut der = PKCS8_PREFIX.to_vec();
	der.extend_from_slice(seed);
	der
}

/// Splits one DER element into its tag, content and the remaining bytes.
fn read_tlv(input: &[u8]) -> Option<(u8, &[u8], &[u8])> {
	let (&tag, rest) = input.split_first()?;
	let (&first, rest) = rest.split_first()?;
	let (len, rest) = if first < 0x80 {
		(usize::from(first), rest)
	} else {
		let count = usize::from(first & 0x7f);
		if count == 0 || count > 2 || rest.len() < count {
			return None;
		}
		let (len_bytes, rest) = rest.split_at(count);
		(len_bytes.iter().fold(0_usize, |acc, &b| (acc << 8) | usize::from(b)), rest)
	};
	if rest.len() < len {
		return None;
	}
	let (content, rest) = rest.split_at(len);
	Some((tag, content, rest))
}

/// Extracts the Ed25519 seed from PKCS#8 DER, ignoring any trailing attributes.
fn parse_pkcs8_seed(der: &[u8]) -> Result<[u8; 32], Error> {
	let bad = || Error::Key("not a PKCS#8 Ed25519 private key".into());
	let (tag, body, _) = read_tlv(der).ok_or_else(bad)?;
	if tag != 0x30 {
		return Err(bad());
	}
	let (version_tag, _, body) = read_tlv(body).ok_or_else(bad)?;
	let (alg_tag, alg, body) = read_tlv(body).ok_or_else(bad)?;
	let (oid_tag, oid, _) = read_tlv(alg).ok_or_else(bad)?;
	if version_tag != 0x02 || alg_tag != 0x30 || oid_tag != 0x06 || oid != ED25519_OID {
		return Err(bad());
	}
	let (key_tag, key, _) = read_tlv(body).ok_or_else(bad)?;
	let (seed_tag, seed, _) = read_tlv(key).ok_or_else(bad)?;
	if key_tag != 0x04 || seed_tag != 0x04 {
		return Err(bad());
	}
	<[u8; 32]>::try_from(seed).map_err(|_| bad())
}

fn to_value(object: &CanonicalJsonObject) -> Value {
	Value::Object(object.clone())
}

/// Canonical JSON of `object` without `signatures` and `unsigned`.
fn signing_bytes(object: &CanonicalJsonObject) -> Result<String, Error> {
	let mut stripped = object.clone();
	stripped.remove("signatures");
	stripped.remove("unsigned");
	json::write_string_value(&Value::Object(stripped)).map_err(|e| Error::Json(e.to_string()))
}

/// Known public keys, by server name and key ID.
struct KeyRing(BTreeMap<(String, String), VerificationKey>);

impl KeyRing {
	fn new(keys: &PublicKeyMap) -> Result<Self, Error> {
		let mut ring = BTreeMap::new();
		for (server, set) in keys {
			for (key_id, key) in set {
				let key = VerificationKey::try_from(key.as_bytes())
					.map_err(|e| Error::Key(alloc::format!("{e:?}")))?;
				ring.insert((server.as_str().to_string(), key_id.as_str().to_string()), key);
			}
		}
		Ok(Self(ring))
	}

	/// Verifies a base64 signature by `(server, key_id)` over `message`.
	fn verify_base64(
		&self,
		server: &str,
		key_id: &str,
		message: &[u8],
		signature: &Value,
	) -> Result<(), Error> {
		let signature =
			signature.as_str().ok_or_else(|| Error::Json("signature is not a string".into()))?;
		let bytes = STANDARD_NO_PAD
			.decode(signature.trim_end_matches('='))
			.map_err(|e| Error::Json(e.to_string()))?;
		self.verify(server, key_id, message, &bytes).map_err(Error::Verification)
	}
}

impl SignatureVerifier for KeyRing {
	fn has_key(&self, server_name: &str, key_id: &str) -> bool {
		self.0.contains_key(&(server_name.to_string(), key_id.to_string()))
	}

	fn verify(
		&self,
		server_name: &str,
		key_id: &str,
		message: &[u8],
		signature: &[u8],
	) -> Result<(), String> {
		let key = self
			.0
			.get(&(server_name.to_string(), key_id.to_string()))
			.ok_or_else(|| alloc::format!("no key {key_id} for {server_name}"))?;
		let signature = RawSignature::try_from(signature).map_err(|e| alloc::format!("{e:?}"))?;
		key.verify(&signature, message)
			.map_err(|e| alloc::format!("{server_name} {key_id}: {e:?}"))
	}
}

fn insert_signature(
	object: &mut CanonicalJsonObject,
	entity: &str,
	keypair: &Ed25519KeyPair,
	signature: &Signature,
) {
	let mut signatures = match object.remove("signatures") {
		Some(Value::Object(map)) => map,
		_ => json::Object::new(),
	};
	let mut entry = match signatures.remove(entity) {
		Some(Value::Object(map)) => map,
		_ => json::Object::new(),
	};
	entry.insert(
		alloc::format!("ed25519:{}", keypair.version()),
		Value::String(STANDARD_NO_PAD.encode(signature.as_bytes())),
	);
	signatures.insert(entity.to_string(), Value::Object(entry));
	object.insert("signatures".to_string(), Value::Object(signatures));
}

/// Signs `object` as `entity_id`, adding to its `signatures`.
///
/// # Errors
///
/// Returns an error if the object cannot be canonicalized.
pub fn sign_json(
	entity_id: &str,
	keypair: &Ed25519KeyPair,
	object: &mut CanonicalJsonObject,
) -> Result<(), Error> {
	let message = signing_bytes(object)?;
	let signature = keypair.sign(message.as_bytes());
	insert_signature(object, entity_id, keypair, &signature);
	Ok(())
}

/// Adds the content hash to an event and signs it as `entity_id`.
///
/// # Errors
///
/// Returns an error if the event is too large, the room version is unsupported or the
/// event cannot be canonicalized.
pub fn hash_and_sign_event(
	entity_id: &str,
	keypair: &Ed25519KeyPair,
	object: &mut CanonicalJsonObject,
	version: &RoomVersionId,
) -> Result<(), Error> {
	object.remove("signatures");
	object.remove("hashes");
	let hash =
		rezzy::compute_content_hash(&to_value(object), version.as_str()).map_err(Error::Json)?;
	let mut hashes = json::Object::new();
	hashes.insert("sha256".to_string(), Value::String(hash));
	object.insert("hashes".to_string(), Value::Object(hashes));

	let message = rezzy::try_canonical_redacted_json(&to_value(object), version.as_str())
		.map_err(|e| Error::Json(e.clone()))?;
	if message.len() > MAX_PDU_BYTES {
		return Err(Error::PduSize);
	}
	let signature = keypair.sign(message.as_bytes());
	insert_signature(object, entity_id, keypair, &signature);
	Ok(())
}

/// Verifies the signatures in `object["signatures"]` for every server in `keys`.
///
/// # Errors
///
/// Returns an error if there is nothing to verify or any signature is invalid.
pub fn verify_json(
	keys: &PublicKeyMap,
	object: impl core::borrow::Borrow<CanonicalJsonObject>,
) -> Result<(), Error> {
	let object = object.borrow();
	let ring = KeyRing::new(keys)?;
	let message = signing_bytes(object)?;
	let signatures = object
		.get("signatures")
		.and_then(Value::as_object)
		.ok_or_else(|| Error::Verification("no signatures".into()))?;

	let mut checked = false;
	for (server, entry) in signatures {
		let Some(entry) = entry.as_object() else {
			continue;
		};
		for (key_id, signature) in entry {
			if ring.has_key(server, key_id) {
				ring.verify_base64(server, key_id, message.as_bytes(), signature)?;
				checked = true;
			}
		}
	}
	if checked {
		Ok(())
	} else {
		Err(Error::Verification("no signature matched a known key".into()))
	}
}

/// Verifies an event's signatures and content hash.
///
/// # Errors
///
/// Returns an error if the signatures do not verify.
pub fn verify_event(
	keys: &PublicKeyMap,
	object: &CanonicalJsonObject,
	version: &RoomVersionId,
) -> Result<Verified, Error> {
	let ring = KeyRing::new(keys)?;
	let value = to_value(object);
	verify_event_signatures(&value, version.as_str(), &ring).map_err(Error::Verification)?;
	Ok(match rezzy::verify_content_hash(&value, version.as_str()) {
		Ok(()) => Verified::All,
		Err(_) => Verified::Signatures,
	})
}

/// The server signing keys an event needs: the sender's server and any other signers.
///
/// # Errors
///
/// Returns an error if the event has no `signatures` object or no identifiable sender.
pub fn required_keys(
	object: &CanonicalJsonObject,
	version: &RoomVersionId,
) -> Result<RequiredKeys, Error> {
	let signatures = object
		.get("signatures")
		.and_then(Value::as_object)
		.ok_or_else(|| Error::Json("missing signatures".into()))?;
	let mut required = RequiredKeys::new();
	for (server, entry) in signatures {
		let ids = entry
			.as_object()
			.map(|entry| {
				entry.keys().map(|k| OwnedServerSigningKeyId::from_trusted(k.as_str())).collect()
			})
			.unwrap_or_default();
		required.insert(OwnedServerName::from_trusted(server.as_str()), ids);
	}
	let _ = version;
	if required.is_empty() {
		return Err(Error::Json("no signing servers".into()));
	}
	Ok(required)
}

/// Computes the event reference hash via Rezzy.
///
/// # Errors
///
/// Returns an error if the room version has no reference hash or the
/// canonical JSON writer rejects the object.
pub fn reference_hash(
	object: &CanonicalJsonObject,
	version: &RoomVersionId,
) -> Result<String, Error> {
	rezzy::reference_hash(&to_value(object), version.as_str()).map_err(Error::Json)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn keys_for(server: &str, pair: &Ed25519KeyPair) -> PublicKeyMap {
		let mut set = PublicKeySet::new();
		set.insert(
			OwnedServerSigningKeyId::from(alloc::format!("ed25519:{}", pair.version())),
			Base64::new(pair.public_key().to_vec()),
		);
		let mut map = PublicKeyMap::new();
		map.insert(OwnedServerName::from(server), set);
		map
	}

	fn pair() -> Ed25519KeyPair {
		Ed25519KeyPair::from_der(&Ed25519KeyPair::generate().unwrap(), "v1".into()).unwrap()
	}

	#[test]
	fn der_round_trips_and_accepts_version_two_keys() {
		let seed = [7_u8; 32];
		let v1 = encode_pkcs8(&seed);
		assert_eq!(v1.len(), 48);
		assert_eq!(parse_pkcs8_seed(&v1).unwrap(), seed);

		// Version 2 (as written by ring): attributes and the public key follow the seed.
		let mut v2 = alloc::vec![0x30, 0x53, 0x02, 0x01, 0x01];
		v2.extend_from_slice(&v1[5..]);
		v2.extend_from_slice(&[0xa1, 0x23, 0x03, 0x21, 0x00]);
		v2.extend_from_slice(&[9_u8; 32]);
		assert_eq!(parse_pkcs8_seed(&v2).unwrap(), seed);

		assert!(parse_pkcs8_seed(&v1[..40]).is_err());
		assert!(parse_pkcs8_seed(&[]).is_err());
	}

	#[test]
	fn rfc8032_test_vector_signs_and_verifies() {
		let seed: [u8; 32] = [
			0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec,
			0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03,
			0x1c, 0xae, 0x7f, 0x60,
		];
		let pair = Ed25519KeyPair::from_der(&encode_pkcs8(&seed), "t".into()).unwrap();
		assert_eq!(pair.public_key()[..4], [0xd7, 0x5a, 0x98, 0x01]);
		// RFC 8032 test 1: the empty message.
		assert_eq!(pair.sign(b"").as_bytes()[..4], [0xe5, 0x56, 0x43, 0x00]);
	}

	#[test]
	fn event_signed_by_another_server_is_rejected() {
		let pair = pair();
		let Value::Object(mut event) = Value::parse(
			r#"{"type":"m.room.message","sender":"@a:example.org","room_id":"!r:example.org","origin_server_ts":1,"depth":1,"content":{"body":"hi"},"prev_events":[],"auth_events":[]}"#,
		)
		.unwrap() else {
			panic!("object")
		};
		let version = RoomVersionId::V11;
		hash_and_sign_event("other.org", &pair, &mut event, &version).unwrap();
		let keys = keys_for("other.org", &pair);
		// The sender is on example.org, so a signature from other.org does not count.
		assert!(verify_event(&keys, &event, &version).is_err());
	}

	#[test]
	fn sign_and_verify_json_round_trip() {
		let pair = pair();
		let Value::Object(mut object) = Value::parse(r#"{"a":1,"b":"two"}"#).unwrap() else {
			panic!("object")
		};
		sign_json("example.org", &pair, &mut object).unwrap();
		let keys = keys_for("example.org", &pair);
		verify_json(&keys, object.clone()).unwrap();

		object.insert("a".into(), Value::parse("2").unwrap());
		assert!(verify_json(&keys, object).is_err());
	}

	#[test]
	fn hash_sign_and_verify_event_round_trip() {
		let pair = pair();
		let Value::Object(mut event) = Value::parse(
			r#"{"type":"m.room.message","sender":"@a:example.org","room_id":"!r:example.org","origin_server_ts":1,"depth":1,"content":{"body":"hi"},"prev_events":[],"auth_events":[]}"#,
		)
		.unwrap() else {
			panic!("object")
		};
		let version = RoomVersionId::V11;
		hash_and_sign_event("example.org", &pair, &mut event, &version).unwrap();
		let keys = keys_for("example.org", &pair);
		assert_eq!(verify_event(&keys, &event, &version).unwrap(), Verified::All);
		let required = required_keys(&event, &version).unwrap();
		assert!(required.contains_key(&OwnedServerName::from("example.org")));
	}
}

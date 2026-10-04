//! Event and JSON signing and verification, backed by Rezzy and Ed25519.

use alloc::{
	collections::BTreeMap,
	string::{String, ToString},
	vec::Vec,
};
use core::fmt;

use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use ed25519_dalek::{
	Signer as _, SigningKey,
	pkcs8::{DecodePrivateKey, EncodePrivateKey},
};
use rezzy::signing::{DalekVerifier, SignatureVerifier, verify_event_signatures};

use crate::{
	CanonicalJsonObject, OwnedServerName, OwnedServerSigningKeyId, RoomVersionId,
	json::{self, Value},
	serde::Base64,
};

/// Maximum canonical PDU size in bytes.
const MAX_PDU_BYTES: usize = 65_535;

/// Errors from signing and verification.
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
	/// Returns an error if the key cannot be encoded.
	pub fn generate() -> Result<Vec<u8>, Error> {
		let key = SigningKey::generate(&mut rand_core::OsRng);
		key.to_pkcs8_der()
			.map(|der| der.as_bytes().to_vec())
			.map_err(|e| Error::Key(e.to_string()))
	}

	/// Loads a key from PKCS#8 DER.
	///
	/// # Errors
	///
	/// Returns an error if `der` is not a valid Ed25519 private key.
	pub fn from_der(der: &[u8], version: String) -> Result<Self, Error> {
		let key = SigningKey::from_pkcs8_der(der).map_err(|e| Error::Key(e.to_string()))?;
		Ok(Self {
			key,
			version,
		})
	}

	#[must_use]
	pub fn public_key(&self) -> [u8; 32] {
		self.key.verifying_key().to_bytes()
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

fn verifier(keys: &PublicKeyMap) -> Result<DalekVerifier, Error> {
	let mut verifier = DalekVerifier::new();
	for (server, set) in keys {
		for (key_id, key) in set {
			verifier
				.insert_public_key(server.as_str(), key_id.as_str(), key.as_bytes())
				.map_err(Error::Key)?;
		}
	}
	Ok(verifier)
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
	let verifier = verifier(keys)?;
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
			if !verifier.has_key(server, key_id) {
				continue;
			}
			let signature = signature
				.as_str()
				.ok_or_else(|| Error::Json("signature is not a string".into()))?;
			let bytes = STANDARD_NO_PAD
				.decode(signature.trim_end_matches('='))
				.map_err(|e| Error::Json(e.to_string()))?;
			verifier
				.verify(server, key_id, message.as_bytes(), &bytes)
				.map_err(Error::Verification)?;
			checked = true;
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
	let verifier = verifier(keys)?;
	let value = to_value(object);
	verify_event_signatures(&value, version.as_str(), &verifier).map_err(Error::Verification)?;
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
				entry.keys().map(|k| OwnedServerSigningKeyId::from(k.as_str())).collect()
			})
			.unwrap_or_default();
		required.insert(OwnedServerName::from(server.as_str()), ids);
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

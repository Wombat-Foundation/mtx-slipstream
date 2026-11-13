//! Types used by the `X-Matrix` federation authorization header.

use alloc::{
	format,
	string::{String, ToString},
};

use http::{HeaderValue, Method};

use crate::{
	CanonicalJsonObject, CanonicalJsonValue, OwnedServerName, OwnedServerSigningKeyId,
	ServerName,
	signatures::{self, Ed25519KeyPair},
};

/// The parsed fields of an `X-Matrix` authorization header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XMatrix {
	pub origin: OwnedServerName,
	pub destination: Option<OwnedServerName>,
	pub key: OwnedServerSigningKeyId,
	pub sig: String,
}

impl crate::codec::Serialize for XMatrix {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(origin), crate::endpoint::enc(&self.origin)),
			(stringify!(destination), crate::endpoint::enc(&self.destination)),
			(stringify!(key), crate::endpoint::enc(&self.key)),
			(stringify!(sig), crate::endpoint::enc(&self.sig)),
		])
	}
}
impl crate::codec::Deserialize for XMatrix {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(XMatrix)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			origin: input.body(stringify!(origin))?,
			destination: input.body(stringify!(destination))?,
			key: input.body(stringify!(key))?,
			sig: input.body(stringify!(sig))?,
		})
	}
}

impl XMatrix {
	/// Sign a federation request and return its `X-Matrix` credentials.
	///
	/// `uri` must be the path and query sent on the wire. `content` is present
	/// only when the request carries a JSON body.
	///
	/// # Errors
	/// Returns a signing or key-ID error if the request cannot be signed.
	pub fn sign(
		keypair: &Ed25519KeyPair,
		origin: &ServerName,
		destination: &ServerName,
		method: &Method,
		uri: &str,
		content: Option<&CanonicalJsonValue>,
	) -> Result<Self, signatures::Error> {
		if keypair.version().is_empty()
			|| !keypair.version().bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
		{
			return Err(signatures::Error::Key(
				"signing key version must contain only ASCII letters, digits, or underscores"
					.into(),
			));
		}
		let key = OwnedServerSigningKeyId::parse(format!("ed25519:{}", keypair.version()))
			.map_err(|error| signatures::Error::Key(error.to_string()))?;
		let mut object = CanonicalJsonObject::new();
		object.insert("destination".into(), destination.as_str().into());
		object.insert("method".into(), method.as_str().into());
		object.insert("origin".into(), origin.as_str().into());
		object.insert("uri".into(), uri.into());
		if let Some(content) = content {
			object.insert("content".into(), content.clone());
		}
		signatures::sign_json(origin.as_str(), keypair, &mut object)?;
		let sig = object
			.get("signatures")
			.and_then(CanonicalJsonValue::as_object)
			.and_then(|signatures| signatures.get(origin.as_str()))
			.and_then(CanonicalJsonValue::as_object)
			.and_then(|signatures| signatures.get(key.as_str()))
			.and_then(CanonicalJsonValue::as_str)
			.ok_or_else(|| signatures::Error::Json("signed request lacks its signature".into()))?
			.to_string();
		Ok(Self {
			origin: origin.to_owned(),
			destination: Some(destination.to_owned()),
			key,
			sig,
		})
	}

	/// Decodes an `X-Matrix` authorization header.
	#[must_use]
	pub fn decode(value: &HeaderValue) -> Option<Self> {
		let text = core::str::from_utf8(value.as_bytes()).ok()?;
		let mut origin = None;
		let mut destination = None;
		let mut key = None;
		let mut sig = None;
		for part in text.strip_prefix("X-Matrix ")?.split(',') {
			let (name, value) = part.trim().split_once('=')?;
			let value = value.trim_matches('"');
			match name.trim() {
				"origin" => origin = Some(OwnedServerName::parse(value).ok()?),
				"destination" => destination = Some(OwnedServerName::parse(value).ok()?),
				"key" => key = Some(OwnedServerSigningKeyId::parse(value).ok()?),
				"sig" => sig = Some(value.into()),
				_ => {}
			}
		}
		Some(Self {
			origin: origin?,
			destination,
			key: key?,
			sig: sig?,
		})
	}

	/// Encodes this credential as an `X-Matrix` authorization header.
	///
	/// # Panics
	///
	/// Panics if the credential fields produce an invalid HTTP header value.
	#[must_use]
	pub fn encode(&self) -> HeaderValue {
		let destination = self
			.destination
			.as_ref()
			.map(|d| alloc::format!(",destination=\"{d}\""))
			.unwrap_or_default();
		HeaderValue::from_str(&alloc::format!(
			"X-Matrix origin=\"{}\"{},key=\"{}\",sig=\"{}\"",
			self.origin,
			destination,
			self.key,
			self.sig
		))
		.expect("X-Matrix fields must be valid header values")
	}
}

#[cfg(test)]
mod tests {
	use super::XMatrix;
	use crate::{
		CanonicalJsonObject, CanonicalJsonValue, OwnedServerName, OwnedServerSigningKeyId, codec,
		signatures::{self, Ed25519KeyPair, PublicKeyMap, PublicKeySet},
		sswire::Base64,
	};
	use http::Method;

	#[test]
	fn x_matrix_round_trips() {
		let value = XMatrix {
			origin: OwnedServerName::parse("origin.example").unwrap(),
			destination: Some(OwnedServerName::parse("destination.example").unwrap()),
			key: OwnedServerSigningKeyId::parse("ed25519:auto").unwrap(),
			sig: "signature".into(),
		};
		assert_eq!(codec::from_value::<XMatrix>(&codec::to_value(&value)).unwrap(), value);
	}

	#[test]
	fn x_matrix_omits_optional_destination() {
		let value = XMatrix {
			origin: OwnedServerName::parse("origin.example").unwrap(),
			destination: None,
			key: OwnedServerSigningKeyId::parse("ed25519:auto").unwrap(),
			sig: "signature".into(),
		};
		let encoded = value.encode();
		assert!(!encoded.to_str().unwrap().contains("destination="));
		assert_eq!(XMatrix::decode(&encoded), Some(value));
	}

	#[test]
	fn signed_federation_request_verifies_and_binds_uri() {
		let pair = Ed25519KeyPair::from_der(&Ed25519KeyPair::generate().unwrap(), "test".into())
			.unwrap();
		let origin = OwnedServerName::parse("origin.example").unwrap();
		let destination = OwnedServerName::parse("destination.example").unwrap();
		let method = Method::PUT;
		let uri = "/_matrix/federation/v2/send_join/room/event?omit_members=false";
		let content = CanonicalJsonValue::Object(CanonicalJsonObject::new());
		let auth =
			XMatrix::sign(&pair, &origin, &destination, &method, uri, Some(&content)).unwrap();
		assert_eq!(XMatrix::decode(&auth.encode()), Some(auth.clone()));

		let mut object = CanonicalJsonObject::new();
		object.insert("content".into(), content);
		object.insert("destination".into(), destination.as_str().into());
		object.insert("method".into(), method.as_str().into());
		object.insert("origin".into(), origin.as_str().into());
		object.insert("uri".into(), uri.into());
		let mut key_signatures = CanonicalJsonObject::new();
		key_signatures.insert(auth.key.as_str().into(), auth.sig.clone().into());
		let mut signatures = CanonicalJsonObject::new();
		signatures.insert(origin.as_str().into(), CanonicalJsonValue::Object(key_signatures));
		object.insert("signatures".into(), CanonicalJsonValue::Object(signatures));

		let mut keys = PublicKeyMap::new();
		let mut origin_keys = PublicKeySet::new();
		origin_keys.insert(auth.key.clone(), Base64::new(pair.public_key().to_vec()));
		keys.insert(origin.clone(), origin_keys);
		signatures::verify_json(&keys, object.clone()).unwrap();
		object.insert("uri".into(), "/_matrix/federation/v2/send_join/other/event".into());
		assert!(signatures::verify_json(&keys, object).is_err());
	}
}

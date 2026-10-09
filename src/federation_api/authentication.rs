//! Types used by the `X-Matrix` federation authorization header.

use alloc::string::String;

use http::HeaderValue;

use crate::{OwnedServerName, OwnedServerSigningKeyId};

/// The parsed fields of an `X-Matrix` authorization header.
#[derive(Debug, Eq, PartialEq)]
pub struct XMatrix {
	pub origin: OwnedServerName,
	pub destination: Option<OwnedServerName>,
	pub key: OwnedServerSigningKeyId,
	pub sig: String,
}

crate::impl_codec_struct!(XMatrix {
	origin: OwnedServerName,
	destination: Option<OwnedServerName>,
	key: OwnedServerSigningKeyId,
	sig: String,
});

impl XMatrix {
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
	use crate::{OwnedServerName, OwnedServerSigningKeyId, codec};

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
}

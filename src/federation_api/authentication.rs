//! Types used by the `X-Matrix` federation authorization header.

use alloc::string::String;

use headers::authorization::Credentials;
use http::HeaderValue;

use crate::{OwnedServerName, OwnedServerSigningKeyId};

/// The parsed fields of an `X-Matrix` authorization header.
#[derive(Clone, Debug, Eq, PartialEq)]
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

impl Credentials for XMatrix {
	const SCHEME: &'static str = "X-Matrix";

	fn decode(value: &HeaderValue) -> Option<Self> {
		let text = core::str::from_utf8(value.as_bytes()).ok()?;
		let mut origin = None;
		let mut destination = None;
		let mut key = None;
		let mut sig = None;
		for part in text.strip_prefix("X-Matrix ")?.split(',') {
			let (name, value) = part.trim().split_once('=')?;
			let value = value.trim_matches('"');
			match name.trim() {
				"origin" => origin = Some(OwnedServerName::from(value)),
				"destination" => destination = Some(OwnedServerName::from(value)),
				"key" => key = Some(OwnedServerSigningKeyId::from(value)),
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

	fn encode(&self) -> HeaderValue {
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
	use headers::authorization::Credentials;

	#[test]
	fn x_matrix_round_trips() {
		let value = XMatrix {
			origin: OwnedServerName::from("origin.example"),
			destination: Some(OwnedServerName::from("destination.example")),
			key: OwnedServerSigningKeyId::from("ed25519:auto"),
			sig: "signature".into(),
		};
		assert_eq!(codec::from_value::<XMatrix>(&codec::to_value(&value)).unwrap(), value);
	}

	#[test]
	fn x_matrix_omits_optional_destination() {
		let value = XMatrix {
			origin: OwnedServerName::from("origin.example"),
			destination: None,
			key: OwnedServerSigningKeyId::from("ed25519:auto"),
			sig: "signature".into(),
		};
		let encoded = value.encode();
		assert!(!encoded.to_str().unwrap().contains("destination="));
		assert_eq!(XMatrix::decode(&encoded), Some(value));
	}
}

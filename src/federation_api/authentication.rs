//! Types used by the `X-Matrix` federation authorization header.

use alloc::string::String;

use headers::authorization::Credentials;
use http::HeaderValue;

use crate::{OwnedServerName, OwnedServerSigningKeyId};

/// The parsed fields of an `X-Matrix` authorization header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XMatrix {
	pub origin: OwnedServerName,
	pub destination: OwnedServerName,
	pub key: OwnedServerSigningKeyId,
	pub sig: String,
}

crate::impl_codec_struct!(XMatrix {
	origin: OwnedServerName,
	destination: OwnedServerName,
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
		Some(Self { origin: origin?, destination: destination?, key: key?, sig: sig? })
	}

	fn encode(&self) -> HeaderValue {
		HeaderValue::from_str(&alloc::format!(
			"X-Matrix origin=\"{}\",destination=\"{}\",key=\"{}\",sig=\"{}\"",
			self.origin, self.destination, self.key, self.sig
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
			origin: OwnedServerName::from("origin.example"),
			destination: OwnedServerName::from("destination.example"),
			key: OwnedServerSigningKeyId::from("ed25519:auto"),
			sig: "signature".into(),
		};
		assert_eq!(codec::from_value::<XMatrix>(&codec::to_value(&value)).unwrap(), value);
	}
}

//! End-to-end encryption key types.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{
	OwnedDeviceId, OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
	impl_codec_enum, impl_codec_struct,
	json::Value,
};

/// Signatures of a key, by signing user and then by key ID.
pub type KeySignatures = BTreeMap<OwnedUserId, BTreeMap<String, String>>;

/// What a cross-signing key may be used for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KeyUsage {
	Master,
	SelfSigning,
	UserSigning,
}

impl_codec_enum!(KeyUsage {
	Master => "master",
	SelfSigning => "self_signing",
	UserSigning => "user_signing",
});

/// Extra information about a device that is not covered by its signatures.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct UnsignedDeviceInfo {
	pub device_display_name: Option<String>,
}

impl_codec_struct!(UnsignedDeviceInfo {} default { device_display_name: Option<String> });

/// Identity keys of a device.
#[derive(Debug, Eq, PartialEq)]
pub struct DeviceKeys {
	pub user_id: OwnedUserId,
	pub device_id: OwnedDeviceId,
	pub algorithms: Vec<String>,
	pub keys: BTreeMap<String, String>,
	pub signatures: KeySignatures,
	pub unsigned: UnsignedDeviceInfo,
}

impl DeviceKeys {
	#[must_use]
	pub fn new(
		user_id: OwnedUserId,
		device_id: OwnedDeviceId,
		algorithms: Vec<String>,
		keys: BTreeMap<String, String>,
		signatures: KeySignatures,
	) -> Self {
		Self {
			user_id,
			device_id,
			algorithms,
			keys,
			signatures,
			unsigned: UnsignedDeviceInfo::default(),
		}
	}
}

impl_codec_struct!(DeviceKeys {
	user_id: OwnedUserId,
	device_id: OwnedDeviceId,
	algorithms: Vec<String>,
	keys: BTreeMap<String, String>,
} default {
	signatures: KeySignatures,
	unsigned: UnsignedDeviceInfo,
});

/// A cross-signing key.
#[derive(Debug, Eq, PartialEq)]
pub struct CrossSigningKey {
	pub user_id: OwnedUserId,
	pub usage: Vec<KeyUsage>,
	pub keys: BTreeMap<String, String>,
	pub signatures: KeySignatures,
}

impl CrossSigningKey {
	#[must_use]
	pub fn new(
		user_id: OwnedUserId,
		usage: Vec<KeyUsage>,
		keys: BTreeMap<String, String>,
		signatures: KeySignatures,
	) -> Self {
		Self {
			user_id,
			usage,
			keys,
			signatures,
		}
	}
}

impl_codec_struct!(CrossSigningKey {
	user_id: OwnedUserId,
	usage: Vec<KeyUsage>,
	keys: BTreeMap<String, String>,
} default {
	signatures: KeySignatures,
});

/// A one-time or fallback key, kept as the JSON the client uploaded.
#[derive(Debug, Eq, PartialEq)]
pub struct OneTimeKey(pub Value);

impl Serialize for OneTimeKey {
	fn to_json(&self) -> Value {
		self.0.clone()
	}
}

impl Deserialize for OneTimeKey {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self(value.clone()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn device_keys_drop_unknown_fields() {
		let keys: DeviceKeys = from_str(
			r#"{"user_id":"@a:b","device_id":"D","algorithms":["x"],"keys":{"ed25519:D":"k"},"dehydrated":true}"#,
		)
		.unwrap();
		assert!(!to_string(&keys).contains("dehydrated"));
		assert_eq!(keys.keys.values().next().map(String::as_str), Some("k"));
	}

	#[test]
	fn cross_signing_key_round_trips() {
		let text = r#"{"user_id":"@a:b","usage":["master"],"keys":{"ed25519:K":"k"}}"#;
		let key: CrossSigningKey = from_str(text).unwrap();
		assert_eq!(key.usage, [KeyUsage::Master]);
		assert_eq!(from_str::<CrossSigningKey>(&to_string(&key)).unwrap(), key);
	}
}

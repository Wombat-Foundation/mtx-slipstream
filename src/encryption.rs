//! End-to-end encryption key types.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{
	OwnedDeviceId, OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
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

impl crate::codec::Serialize for KeyUsage {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Master => "master",
			Self::SelfSigning => "self_signing",
			Self::UserSigning => "user_signing",
		}))
	}
}
impl crate::codec::Deserialize for KeyUsage {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("master") => Ok(Self::Master),
			Some("self_signing") => Ok(Self::SelfSigning),
			Some("user_signing") => Ok(Self::UserSigning),
			_ => Err(crate::codec::DeError::expected(stringify!(KeyUsage))),
		}
	}
}

/// Extra information about a device that is not covered by its signatures.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct UnsignedDeviceInfo {
	pub device_display_name: Option<String>,
}

impl crate::codec::Serialize for UnsignedDeviceInfo {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(device_display_name),
			crate::endpoint::enc(&self.device_display_name),
		)])
	}
}
impl crate::codec::Deserialize for UnsignedDeviceInfo {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(UnsignedDeviceInfo)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			device_display_name: input.body_or_default(stringify!(device_display_name))?,
		})
	}
}

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

impl crate::codec::Serialize for DeviceKeys {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
			(stringify!(device_id), crate::endpoint::enc(&self.device_id)),
			(stringify!(algorithms), crate::endpoint::enc(&self.algorithms)),
			(stringify!(keys), crate::endpoint::enc(&self.keys)),
			(stringify!(signatures), crate::endpoint::enc(&self.signatures)),
			(stringify!(unsigned), crate::endpoint::enc(&self.unsigned)),
		])
	}
}
impl crate::codec::Deserialize for DeviceKeys {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(DeviceKeys)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			user_id: input.body(stringify!(user_id))?,
			device_id: input.body(stringify!(device_id))?,
			algorithms: input.body(stringify!(algorithms))?,
			keys: input.body(stringify!(keys))?,
			signatures: input.body_or_default(stringify!(signatures))?,
			unsigned: input.body_or_default(stringify!(unsigned))?,
		})
	}
}

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

impl crate::codec::Serialize for CrossSigningKey {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
			(stringify!(usage), crate::endpoint::enc(&self.usage)),
			(stringify!(keys), crate::endpoint::enc(&self.keys)),
			(stringify!(signatures), crate::endpoint::enc(&self.signatures)),
		])
	}
}
impl crate::codec::Deserialize for CrossSigningKey {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(CrossSigningKey)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			user_id: input.body(stringify!(user_id))?,
			usage: input.body(stringify!(usage))?,
			keys: input.body(stringify!(keys))?,
			signatures: input.body_or_default(stringify!(signatures))?,
		})
	}
}

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

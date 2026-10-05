//! Key identifiers of the form `<algorithm>:<name>`.

use alloc::string::{String, ToString};
use core::{cmp::Ordering, fmt, hash::Hash, marker::PhantomData, ops::Deref};

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

/// A key ID such as `signed_curve25519:AAAAHg`, tagged with its algorithm and
/// name types.
pub struct OwnedKeyId<A, K>(String, PhantomData<fn() -> (A, K)>);

/// Borrowed form of [`OwnedKeyId`]; identifiers are always owned here.
pub type KeyId<A, K> = OwnedKeyId<A, K>;

impl<A, K> OwnedKeyId<A, K> {
	/// Parses a key ID, which must contain a `:` between algorithm and name.
	///
	/// # Errors
	///
	/// Returns an error if `value` has no `:`.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, crate::MatrixIdParseError> {
		let value = value.as_ref();
		if value.contains(':') {
			Ok(Self(value.to_string(), PhantomData))
		} else {
			Err(crate::MatrixIdParseError)
		}
	}

	/// The full `<algorithm>:<name>` string.
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}

	/// The part before the first colon.
	#[must_use]
	pub fn algorithm(&self) -> A
	where
		A: for<'a> From<&'a str>,
	{
		A::from(self.0.split_once(':').map_or(self.0.as_str(), |(algorithm, _)| algorithm))
	}

	/// The part after the first colon.
	#[must_use]
	pub fn key_name(&self) -> K
	where
		K: for<'a> From<&'a str>,
	{
		K::from(self.0.split_once(':').map_or("", |(_, name)| name))
	}

	/// Builds a key ID from its algorithm and name.
	#[must_use]
	pub fn from_parts(algorithm: &A, name: &K) -> Self
	where
		A: AsRef<str>,
		K: AsRef<str>,
	{
		Self(alloc::format!("{}:{}", algorithm.as_ref(), name.as_ref()), PhantomData)
	}
}

impl<A, K> Clone for OwnedKeyId<A, K> {
	fn clone(&self) -> Self {
		Self(self.0.clone(), PhantomData)
	}
}
impl<A, K> PartialEq for OwnedKeyId<A, K> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}
impl<A, K> Eq for OwnedKeyId<A, K> {}
impl<A, K> PartialOrd for OwnedKeyId<A, K> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl<A, K> Ord for OwnedKeyId<A, K> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}
impl<A, K> Hash for OwnedKeyId<A, K> {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}
}
impl<A, K> fmt::Debug for OwnedKeyId<A, K> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("OwnedKeyId").field(&self.0).finish()
	}
}
impl<A, K> fmt::Display for OwnedKeyId<A, K> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}
impl<A, K> Deref for OwnedKeyId<A, K> {
	type Target = str;

	fn deref(&self) -> &str {
		&self.0
	}
}
impl<A, K> AsRef<str> for OwnedKeyId<A, K> {
	fn as_ref(&self) -> &str {
		&self.0
	}
}
impl<A, K> core::borrow::Borrow<str> for OwnedKeyId<A, K> {
	fn borrow(&self) -> &str {
		&self.0
	}
}
impl<A, K> From<&str> for OwnedKeyId<A, K> {
	fn from(value: &str) -> Self {
		Self(value.to_string(), PhantomData)
	}
}
impl<A, K> From<String> for OwnedKeyId<A, K> {
	fn from(value: String) -> Self {
		Self(value, PhantomData)
	}
}
impl<A, K> Serialize for OwnedKeyId<A, K> {
	fn to_json(&self) -> Value {
		Value::String(self.0.clone())
	}
}
impl<A, K> Deserialize for OwnedKeyId<A, K> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value.as_str().map(Self::from).ok_or_else(|| DeError::expected("key ID"))
	}
}

/// An algorithm for one-time keys.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OneTimeKeyAlgorithm {
	Curve25519,
	SignedCurve25519,
	/// Any other algorithm.
	Custom(String),
}

impl OneTimeKeyAlgorithm {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Curve25519 => "curve25519",
			Self::SignedCurve25519 => "signed_curve25519",
			Self::Custom(other) => other,
		}
	}
}

impl AsRef<str> for OneTimeKeyAlgorithm {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Display for OneTimeKeyAlgorithm {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl From<&str> for OneTimeKeyAlgorithm {
	fn from(value: &str) -> Self {
		match value {
			"curve25519" => Self::Curve25519,
			"signed_curve25519" => Self::SignedCurve25519,
			other => Self::Custom(other.to_string()),
		}
	}
}

/// The name part of a one-time key ID.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OneTimeKeyName(pub String);

impl AsRef<str> for OneTimeKeyName {
	fn as_ref(&self) -> &str {
		&self.0
	}
}
impl fmt::Display for OneTimeKeyName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}
impl From<&str> for OneTimeKeyName {
	fn from(value: &str) -> Self {
		Self(value.to_string())
	}
}

/// A one-time key ID.
pub type OwnedOneTimeKeyId = OwnedKeyId<OneTimeKeyAlgorithm, OneTimeKeyName>;
/// Borrowed form of [`OwnedOneTimeKeyId`].
pub type OneTimeKeyId = OwnedOneTimeKeyId;

/// An algorithm for signing keys.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SigningKeyAlgorithm {
	Ed25519,
	/// Any other algorithm.
	Custom(String),
}

impl SigningKeyAlgorithm {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Ed25519 => "ed25519",
			Self::Custom(other) => other,
		}
	}
}

impl AsRef<str> for SigningKeyAlgorithm {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Display for SigningKeyAlgorithm {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl From<&str> for SigningKeyAlgorithm {
	fn from(value: &str) -> Self {
		match value {
			"ed25519" => Self::Ed25519,
			other => Self::Custom(other.to_string()),
		}
	}
}

macro_rules! key_name {
	($(#[$doc:meta])* $name:ident) => {
		$(#[$doc])*
		#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
		pub struct $name(pub String);

		impl AsRef<str> for $name {
			fn as_ref(&self) -> &str {
				&self.0
			}
		}
		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.write_str(&self.0)
			}
		}
		impl From<&str> for $name {
			fn from(value: &str) -> Self {
				Self(value.to_string())
			}
		}
	};
}

key_name!(
	/// The name part of a key ID that is a base64 public key.
	Base64PublicKey
);
key_name!(
	/// The version part of a server signing key ID.
	ServerSigningKeyVersion
);

impl crate::codec::Serialize for OneTimeKeyAlgorithm {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(self.as_str().into())
	}
}
impl crate::codec::Deserialize for OneTimeKeyAlgorithm {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		value
			.as_str()
			.map(Self::from)
			.ok_or_else(|| crate::codec::DeError::expected("algorithm string"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn key_id_splits_into_algorithm_and_name() {
		let id: OwnedOneTimeKeyId = OwnedKeyId::from("signed_curve25519:AAAAHg");
		assert_eq!(id.algorithm(), OneTimeKeyAlgorithm::SignedCurve25519);
		assert_eq!(id.key_name(), OneTimeKeyName("AAAAHg".into()));
		assert_eq!(OwnedOneTimeKeyId::from_parts(&id.algorithm(), &id.key_name()), id);
		assert_eq!(from_str::<OwnedOneTimeKeyId>(&to_string(&id)).unwrap(), id);
	}
}

//! Serde-free (de)serialization traits.
//!
//! These mirror the shape of `serde::{Serialize, Deserialize}` but operate
//! directly on the Rezzy JSON [`Value`], so no `serde` dependency is needed.

use alloc::{
	collections::BTreeMap,
	string::{String, ToString},
	vec::Vec,
};
use core::fmt;

use crate::json::Value;

/// Error produced when a JSON value does not match the target type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeError(pub String);

impl DeError {
	#[must_use]
	pub fn expected(what: &str) -> Self {
		Self(alloc::format!("expected {what}"))
	}
}

impl fmt::Display for DeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl core::error::Error for DeError {}

pub trait Serialize {
	fn to_json(&self) -> Value;
}

pub trait Deserialize: Sized {
	/// # Errors
	///
	/// Returns [`DeError`] if `value` does not match the target type.
	fn from_json(value: &Value) -> Result<Self, DeError>;
}

/// Serializes `value` into a JSON [`Value`].
pub fn to_value<T: Serialize + ?Sized>(value: &T) -> Value {
	value.to_json()
}

/// Deserializes a `T` from a JSON [`Value`].
///
/// # Errors
///
/// Returns [`DeError`] if `value` does not match `T`.
pub fn from_value<T: Deserialize>(value: &Value) -> Result<T, DeError> {
	T::from_json(value)
}

/// Serializes `value` to a JSON string.
#[must_use]
pub fn to_string<T: Serialize + ?Sized>(value: &T) -> String {
	crate::json::write_string_value(&value.to_json()).unwrap_or_default()
}

/// Serializes `value` to UTF-8 JSON bytes.
#[must_use]
pub fn to_vec<T: Serialize + ?Sized>(value: &T) -> Vec<u8> {
	to_string(value).into_bytes()
}

/// Parses `input` and deserializes a `T` from it.
///
/// # Errors
///
/// Returns [`DeError`] on a parse failure or type mismatch.
pub fn from_str<T: Deserialize>(input: &str) -> Result<T, DeError> {
	let value = Value::parse(input).map_err(|e| DeError(e.to_string()))?;
	T::from_json(&value)
}

/// Parses UTF-8 JSON bytes and deserializes a `T` from them.
///
/// # Errors
///
/// Returns [`DeError`] on invalid UTF-8, JSON, or type mismatch.
pub fn from_slice<T: Deserialize>(input: &[u8]) -> Result<T, DeError> {
	let input = core::str::from_utf8(input).map_err(|e| DeError(e.to_string()))?;
	from_str(input)
}

impl<T: Serialize + ?Sized> Serialize for &T {
	fn to_json(&self) -> Value {
		(**self).to_json()
	}
}

impl Serialize for Value {
	fn to_json(&self) -> Value {
		self.clone()
	}
}
impl Deserialize for Value {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(value.clone())
	}
}

impl Serialize for String {
	fn to_json(&self) -> Value {
		Value::String(self.clone())
	}
}
impl Serialize for str {
	fn to_json(&self) -> Value {
		Value::String(self.to_string())
	}
}
impl Deserialize for String {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value.as_str().map(ToString::to_string).ok_or_else(|| DeError::expected("string"))
	}
}

impl Serialize for bool {
	fn to_json(&self) -> Value {
		Value::Bool(*self)
	}
}
impl Deserialize for bool {
	/// An absent or null boolean is `false`, as most Matrix flags default to it.
	fn from_json(value: &Value) -> Result<Self, DeError> {
		if value.is_null() {
			return Ok(false);
		}
		value.as_bool().ok_or_else(|| DeError::expected("bool"))
	}
}

macro_rules! int_impl {
	($($t:ty => $as:ident),*) => {$(
		impl Serialize for $t {
			fn to_json(&self) -> Value {
				Value::parse(&self.to_string()).unwrap_or_default()
			}
		}
		impl Deserialize for $t {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				value
					.$as()
					.and_then(|n| <$t>::try_from(n).ok())
					.ok_or_else(|| DeError::expected("integer"))
			}
		}
	)*};
}
int_impl!(i64 => as_i64, u64 => as_u64, i32 => as_i64, u32 => as_u64, u16 => as_u64, i16 => as_i64, u8 => as_u64);

impl Serialize for usize {
	fn to_json(&self) -> Value {
		u64::try_from(*self).expect("usize fits in u64").to_json()
	}
}
impl Deserialize for usize {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		usize::try_from(u64::from_json(value)?).map_err(|_| DeError::expected("usize"))
	}
}

impl Serialize for f64 {
	fn to_json(&self) -> Value {
		crate::json::Number::from_f64(*self).map_or(Value::Null, Value::Number)
	}
}

impl Deserialize for f64 {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value.as_f64().ok_or_else(|| DeError::expected("number"))
	}
}

impl<T: Serialize> Serialize for Option<T> {
	fn to_json(&self) -> Value {
		self.as_ref().map_or(Value::Null, T::to_json)
	}
}
impl<T: Deserialize> Deserialize for Option<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		if value.is_null() {
			Ok(None)
		} else {
			T::from_json(value).map(Some)
		}
	}
}

impl<T: Serialize> Serialize for Vec<T> {
	fn to_json(&self) -> Value {
		Value::Array(self.iter().map(T::to_json).collect())
	}
}
impl<T: Deserialize> Deserialize for Vec<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_array()
			.ok_or_else(|| DeError::expected("array"))?
			.iter()
			.map(T::from_json)
			.collect()
	}
}

impl<T: Serialize> Serialize for alloc::collections::BTreeSet<T> {
	fn to_json(&self) -> Value {
		Value::Array(self.iter().map(T::to_json).collect())
	}
}
impl<T: Deserialize + Ord> Deserialize for alloc::collections::BTreeSet<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_array()
			.ok_or_else(|| DeError::expected("array"))?
			.iter()
			.map(T::from_json)
			.collect()
	}
}

impl<K: Serialize, V: Serialize> Serialize for BTreeMap<K, V> {
	fn to_json(&self) -> Value {
		Value::Object(
			self.iter()
				.map(|(k, v)| (k.to_json().as_str().unwrap_or_default().to_string(), v.to_json()))
				.collect(),
		)
	}
}
impl<K: Deserialize + Ord, V: Deserialize> Deserialize for BTreeMap<K, V> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_object()
			.ok_or_else(|| DeError::expected("object"))?
			.iter()
			.map(|(k, v)| Ok((K::from_json(&Value::String(k.clone()))?, V::from_json(v)?)))
			.collect()
	}
}

/// Implements the traits for a type that is a transparent string wrapper.
#[macro_export]
macro_rules! impl_codec_string {
	($t:ty, |$s:ident| $from:expr, |$v:ident| $to:expr) => {
		impl $crate::codec::Serialize for $t {
			fn to_json(&self) -> $crate::json::Value {
				let $v = self;
				$crate::json::Value::String(::alloc::string::String::from($to))
			}
		}
		impl $crate::codec::Deserialize for $t {
			fn from_json(value: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				let $s =
					value.as_str().ok_or_else(|| $crate::codec::DeError::expected("string"))?;
				$from
			}
		}
	};
}

/// Implements the traits for a fieldless enum using fixed string names.
#[macro_export]
macro_rules! impl_codec_enum {
	($t:ty { $($variant:ident => $name:literal),* $(,)? }) => {
		impl $crate::codec::Serialize for $t {
			fn to_json(&self) -> $crate::json::Value {
				$crate::json::Value::String(::alloc::string::String::from(match self {
					$(Self::$variant => $name,)*
				}))
			}
		}
		impl $crate::codec::Deserialize for $t {
			fn from_json(
				value: &$crate::json::Value,
			) -> Result<Self, $crate::codec::DeError> {
				match value.as_str() {
					$(Some($name) => Ok(Self::$variant),)*
					_ => Err($crate::codec::DeError::expected(stringify!($t))),
				}
			}
		}
	};
}

/// Implements the codec traits for a struct of named fields.
///
/// Fields in the optional `default { .. }` block may be absent or null when
/// decoding, and then take their `Default` value.
#[macro_export]
macro_rules! impl_codec_struct {
	(
		$t:ident { $($field:ident : $ty:ty),* $(,)? }
		$(default { $($dfield:ident : $dty:ty),* $(,)? })?
	) => {
		impl $crate::codec::Serialize for $t {
			fn to_json(&self) -> $crate::json::Value {
				$crate::json::Value::Object($crate::endpoint::object_from(::alloc::vec![
					$((stringify!($field), $crate::codec::Serialize::to_json(&self.$field)),)*
					$($((stringify!($dfield), $crate::codec::Serialize::to_json(&self.$dfield)),)*)?
				]))
			}
		}
		impl $crate::codec::Deserialize for $t {
			fn from_json(value: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(&[], &[], Some(value));
				let parsed = Self {
					$($field: input.body::<$ty>(stringify!($field))?,)*
					$($($dfield: input.body_or_default::<$dty>(stringify!($dfield))?,)*)?
				};
				input.finish()?;
				Ok(parsed)
			}
		}
	};
}

/// Implements the codec traits for a struct with explicit JSON keys.
///
/// Each field names its key: `field: Type = ("key")`. A field written
/// `("key", omit)` is left out when it encodes to `null` and takes its
/// `Default` when absent or null; other fields are required. Unknown keys are
/// ignored on decoding.
///
/// ```text
/// codec_struct!(Pdu {
///     kind: TimelineEventType = ("type"),
///     redacts: Option<OwnedEventId> = ("redacts", omit),
/// });
/// ```
#[macro_export]
macro_rules! codec_struct {
	($t:ident { $($field:ident : $ty:ty = $spec:tt),* $(,)? }) => {
		impl $crate::codec::Serialize for $t {
			fn to_json(&self) -> $crate::json::Value {
				let mut object = $crate::json::Object::new();
				$($crate::codec_struct!(@put object, &self.$field, $spec);)*
				$crate::json::Value::Object(object)
			}
		}
		impl $crate::codec::Deserialize for $t {
			fn from_json(value: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(&[], &[], Some(value));
				Ok(Self {
					$($field: $crate::codec_struct!(@get input, $ty, $spec),)*
				})
			}
		}
	};
	(@put $object:ident, $value:expr, ($key:literal)) => {
		$object.insert($crate::alloc_string($key), $crate::codec::Serialize::to_json($value));
	};
	(@put $object:ident, $value:expr, ($key:literal, omit)) => {
		let encoded = $crate::codec::Serialize::to_json($value);
		if !encoded.is_null() {
			$object.insert($crate::alloc_string($key), encoded);
		}
	};
	(@get $input:ident, $ty:ty, ($key:literal)) => { $input.body::<$ty>($key)? };
	(@get $input:ident, $ty:ty, ($key:literal, omit)) => { $input.body_or_default::<$ty>($key)? };
}

impl<A: Serialize, B: Serialize, C: Serialize> Serialize for (A, B, C) {
	fn to_json(&self) -> Value {
		Value::Array(alloc::vec![self.0.to_json(), self.1.to_json(), self.2.to_json()])
	}
}
impl<A: Deserialize, B: Deserialize, C: Deserialize> Deserialize for (A, B, C) {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		match value.as_array().map(Vec::as_slice) {
			Some([a, b, c]) => Ok((A::from_json(a)?, B::from_json(b)?, C::from_json(c)?)),
			_ => Err(DeError::expected("three-element array")),
		}
	}
}

impl<A: Serialize, B: Serialize> Serialize for (A, B) {
	fn to_json(&self) -> Value {
		Value::Array(alloc::vec![self.0.to_json(), self.1.to_json()])
	}
}
impl<A: Deserialize, B: Deserialize> Deserialize for (A, B) {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		match value.as_array().map(Vec::as_slice) {
			Some([a, b]) => Ok((A::from_json(a)?, B::from_json(b)?)),
			_ => Err(DeError::expected("two-element array")),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Deserialize, from_value, to_value};

	#[test]
	fn usize_codec_validates_integer_input() {
		assert_eq!(usize::from_json(&to_value(&42_usize)).unwrap(), 42);
		assert!(usize::from_json(&crate::json::Value::parse("-1").unwrap()).is_err());
		assert!(usize::from_json(&crate::json::Value::parse("1.5").unwrap()).is_err());
		if usize::BITS == 64 {
			assert_eq!(from_value::<usize>(&to_value(&u64::MAX)).unwrap(), usize::MAX);
		}
	}
}

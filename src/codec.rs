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

/// Parses `input` and deserializes a `T` from it.
///
/// # Errors
///
/// Returns [`DeError`] on a parse failure or type mismatch.
pub fn from_str<T: Deserialize>(input: &str) -> Result<T, DeError> {
	let value = Value::parse(input).map_err(|e| DeError(e.to_string()))?;
	T::from_json(&value)
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
	fn from_json(value: &Value) -> Result<Self, DeError> {
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
int_impl!(i64 => as_i64, u64 => as_u64, i32 => as_i64, u32 => as_u64);

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
#[macro_export]
macro_rules! impl_codec_struct {
	($t:ident { $($field:ident : $ty:ty),* $(,)? }) => {
		impl $crate::codec::Serialize for $t {
			fn to_json(&self) -> $crate::json::Value {
				$crate::json::Value::Object($crate::endpoint::object_from(::alloc::vec![
					$((stringify!($field), $crate::codec::Serialize::to_json(&self.$field))),*
				]))
			}
		}
		impl $crate::codec::Deserialize for $t {
			fn from_json(value: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(&[], &[], Some(value));
				let parsed = Self {
					$($field: input.body::<$ty>(stringify!($field))?,)*
				};
				input.finish()?;
				Ok(parsed)
			}
		}
	};
}

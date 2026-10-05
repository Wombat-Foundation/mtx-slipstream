//! A JSON value that is present, explicitly `null`, or absent.

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

/// Distinguishes `null` from a missing field, as Matrix avatars require.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum JsOption<T> {
	Some(T),
	Null,
	#[default]
	Undefined,
}

impl<T> JsOption<T> {
	/// `Some` becomes present, `None` becomes an explicit `null`.
	#[must_use]
	pub fn from_option(value: Option<T>) -> Self {
		value.map_or(Self::Null, Self::Some)
	}

	/// Present values become `Some`; `null` and absent become `None`.
	#[must_use]
	pub fn into_option(self) -> Option<T> {
		match self {
			Self::Some(value) => Some(value),
			Self::Null | Self::Undefined => None,
		}
	}

	#[must_use]
	pub fn as_ref(&self) -> JsOption<&T> {
		match self {
			Self::Some(value) => JsOption::Some(value),
			Self::Null => JsOption::Null,
			Self::Undefined => JsOption::Undefined,
		}
	}

	#[must_use]
	pub fn is_some(&self) -> bool {
		matches!(self, Self::Some(_))
	}

	#[must_use]
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
	}

	#[must_use]
	pub fn is_undefined(&self) -> bool {
		matches!(self, Self::Undefined)
	}

	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> JsOption<U> {
		match self {
			Self::Some(value) => JsOption::Some(f(value)),
			Self::Null => JsOption::Null,
			Self::Undefined => JsOption::Undefined,
		}
	}
}

impl<T> From<Option<T>> for JsOption<T> {
	fn from(value: Option<T>) -> Self {
		Self::from_option(value)
	}
}

impl<T: Serialize> Serialize for JsOption<T> {
	fn to_json(&self) -> Value {
		match self {
			Self::Some(value) => value.to_json(),
			Self::Null | Self::Undefined => Value::Null,
		}
	}
}

impl<T: Deserialize> Deserialize for JsOption<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		if value.is_null() {
			Ok(Self::Null)
		} else {
			T::from_json(value).map(Self::Some)
		}
	}
}

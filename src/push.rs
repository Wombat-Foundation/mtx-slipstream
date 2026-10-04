use alloc::string::String;

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ruleset;
impl Ruleset {
	#[must_use]
	pub fn new() -> Self {
		Self
	}
	pub fn server_default<T>(_user: &T) -> Self {
		Self
	}
}

impl crate::codec::Serialize for Ruleset {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::Object(crate::json::Object::new())
	}
}

impl crate::codec::Deserialize for Ruleset {
	fn from_json(_: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		Ok(Self)
	}
}
/// A sound or highlight hint attached to a notification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tweak {
	Sound(String),
	Highlight(bool),
	Custom { name: String, value: Value },
}

impl Serialize for Tweak {
	fn to_json(&self) -> Value {
		let (name, value) = match self {
			Self::Sound(sound) => ("sound", Value::String(sound.clone())),
			Self::Highlight(highlight) => ("highlight", Value::Bool(*highlight)),
			Self::Custom {
				name,
				value,
			} => (name.as_str(), value.clone()),
		};
		let mut object = Object::new();
		object.insert("set_tweak".into(), Value::String(name.into()));
		object.insert("value".into(), value);
		Value::Object(object)
	}
}

impl Deserialize for Tweak {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("tweak object"))?;
		let name = object
			.get("set_tweak")
			.and_then(Value::as_str)
			.ok_or_else(|| DeError::expected("set_tweak"))?;
		let value = object.get("value");
		Ok(match name {
			"sound" => Self::Sound(
				value.and_then(Value::as_str).map_or_else(|| "default".into(), String::from),
			),
			"highlight" => Self::Highlight(value.and_then(Value::as_bool).unwrap_or(true)),
			other => Self::Custom {
				name: other.into(),
				value: value.cloned().unwrap_or_default(),
			},
		})
	}
}

/// What to do when a push rule matches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
	Notify,
	SetTweak(Tweak),
	/// An action this crate does not know, kept verbatim.
	Unknown(Value),
}

impl Serialize for Action {
	fn to_json(&self) -> Value {
		match self {
			Self::Notify => Value::String("notify".into()),
			Self::SetTweak(tweak) => tweak.to_json(),
			Self::Unknown(value) => value.clone(),
		}
	}
}

impl Deserialize for Action {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(match value.as_str() {
			Some("notify") => Self::Notify,
			Some(_) => Self::Unknown(value.clone()),
			None => Tweak::from_json(value)
				.map_or_else(|_| Self::Unknown(value.clone()), Self::SetTweak),
		})
	}
}

/// Payload format requested by a push gateway.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PushFormat {
	EventIdOnly,
}

crate::impl_codec_enum!(PushFormat { EventIdOnly => "event_id_only" });

#[derive(Clone, Debug, Default)]
pub struct PushConditionPowerLevelsCtx;
#[derive(Clone, Debug, Default)]
pub struct PushConditionRoomCtx;

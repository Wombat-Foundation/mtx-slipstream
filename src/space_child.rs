//! `m.space.child` event content.

use alloc::{string::String, vec::Vec};

use crate::{
	OwnedServerName,
	codec::{DeError, Deserialize, Serialize},
	impl_codec_struct,
	json::{Object, Value},
};

/// A room's membership of a space, as stated by the space.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SpaceChildEventContent {
	/// Servers to try when joining the child.
	pub via: Vec<OwnedServerName>,
	/// Ordering hint among siblings.
	pub order: Option<String>,
	pub suggested: bool,
}

impl SpaceChildEventContent {
	#[must_use]
	pub fn new(via: Vec<OwnedServerName>) -> Self {
		Self {
			via,
			order: None,
			suggested: false,
		}
	}
}

impl_codec_struct!(SpaceChildEventContent { via: Vec<OwnedServerName> } default {
	order: Option<String>,
	suggested: bool,
});

impl crate::events::EventContent for SpaceChildEventContent {
	type EventType = crate::events::StateEventType;

	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::SpaceChild
	}
}

/// The content of a redacted `m.space.child`: everything is stripped.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RedactedSpaceChildEventContent {}

impl Serialize for RedactedSpaceChildEventContent {
	fn to_json(&self) -> Value {
		Value::Object(Object::new())
	}
}

impl Deserialize for RedactedSpaceChildEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value.as_object().map(|_| Self {}).ok_or_else(|| DeError::expected("object"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn suggested_defaults_to_false() {
		let content: SpaceChildEventContent = from_str(r#"{"via":["a.org"]}"#).unwrap();
		assert!(!content.suggested);
		assert_eq!(from_str::<SpaceChildEventContent>(&to_string(&content)).unwrap(), content);
	}
}

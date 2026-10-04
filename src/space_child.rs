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

/// An `m.space.child` event as carried in a federation hierarchy response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HierarchySpaceChildEvent {
	pub content: SpaceChildEventContent,
	pub sender: crate::OwnedUserId,
	/// The child room.
	pub state_key: crate::OwnedRoomId,
	pub origin_server_ts: crate::MilliSecondsSinceUnixEpoch,
}

impl Serialize for HierarchySpaceChildEvent {
	fn to_json(&self) -> Value {
		Value::Object(crate::endpoint::object_from(alloc::vec![
			("type", Value::String("m.space.child".into())),
			("content", self.content.to_json()),
			("sender", self.sender.to_json()),
			("state_key", self.state_key.to_json()),
			("origin_server_ts", self.origin_server_ts.to_json()),
		]))
	}
}

impl Deserialize for HierarchySpaceChildEvent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("event object"))?;
		let get = |name: &str| {
			object.get(name).ok_or_else(|| DeError(alloc::format!("missing `{name}`")))
		};
		Ok(Self {
			content: SpaceChildEventContent::from_json(get("content")?)?,
			sender: crate::codec::from_value(get("sender")?)?,
			state_key: crate::codec::from_value(get("state_key")?)?,
			origin_server_ts: crate::codec::from_value(get("origin_server_ts")?)?,
		})
	}
}

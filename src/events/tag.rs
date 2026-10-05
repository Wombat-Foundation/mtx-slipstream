//! `m.tag`.

use alloc::{collections::BTreeMap, string::String};
use core::fmt;

use super::wrappers::RoomAccountDataEvent;
use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

/// The name of a room tag, such as `m.favourite` or `u.custom`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TagName(pub String);

impl TagName {
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for TagName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}
impl From<String> for TagName {
	fn from(value: String) -> Self {
		Self(value)
	}
}
impl From<&str> for TagName {
	fn from(value: &str) -> Self {
		Self(value.into())
	}
}
impl Serialize for TagName {
	fn to_json(&self) -> Value {
		Value::String(self.0.clone())
	}
}
impl Deserialize for TagName {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value.as_str().map(Self::from).ok_or_else(|| DeError::expected("tag name"))
	}
}

/// Extra data attached to a tag.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagInfo {
	pub order: Option<f64>,
}

impl TagInfo {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}

impl Serialize for TagInfo {
	fn to_json(&self) -> Value {
		let mut object = crate::json::Object::new();
		if let Some(order) = self.order {
			object.insert("order".into(), order.to_json());
		}
		Value::Object(object)
	}
}

/// Decodes `order` as a number, or as a numeric string: some clients wrote it
/// stringified, and the previous (ruma `compat-tag-info`) build accepted that.
/// An absent or `null` order is `None`; unknown fields are ignored.
impl Deserialize for TagInfo {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("object"))?;
		let order = match object.get("order") {
			None | Some(Value::Null) => None,
			Some(Value::String(text)) => {
				let order =
					text.parse::<f64>().map_err(|_| DeError::expected("numeric order"))?;
				if !order.is_finite() {
					return Err(DeError::expected("finite numeric order"));
				}
				Some(order)
			}
			Some(other) => Some(f64::from_json(other)?),
		};
		Ok(Self {
			order,
		})
	}
}

/// The tags a user has put on a room.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagEventContent {
	pub tags: BTreeMap<TagName, TagInfo>,
}

crate::impl_codec_struct!(TagEventContent { tags: BTreeMap<TagName, TagInfo> });

pub type TagEvent = RoomAccountDataEvent<TagEventContent>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn tag_event_round_trips() {
		let mut event = TagEvent::default();
		event.content.tags.insert(
			"m.favourite".into(),
			TagInfo {
				order: Some(0.25),
			},
		);
		event.content.tags.insert("u.work".into(), TagInfo::new());
		let json = to_string(&event);
		assert!(json.contains("0.25"));
		assert_eq!(from_str::<TagEvent>(&json).unwrap(), event);
	}
}

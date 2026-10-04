//! `m.mentions`: who a message is addressed to.

use alloc::collections::BTreeSet;

use crate::{
	OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
	endpoint::object_from,
	json::Value,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mentions {
	pub user_ids: BTreeSet<OwnedUserId>,
	pub room: bool,
}

impl Mentions {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn with_room_mention() -> Self {
		Self {
			user_ids: BTreeSet::new(),
			room: true,
		}
	}

	#[must_use]
	pub fn with_user_ids(user_ids: impl IntoIterator<Item = OwnedUserId>) -> Self {
		Self {
			user_ids: user_ids.into_iter().collect(),
			room: false,
		}
	}
}

impl Serialize for Mentions {
	fn to_json(&self) -> Value {
		let user_ids = (!self.user_ids.is_empty())
			.then(|| Value::Array(self.user_ids.iter().map(Serialize::to_json).collect()));
		Value::Object(object_from(alloc::vec![
			("user_ids", user_ids.unwrap_or_default()),
			("room", self.room.then_some(true).to_json()),
		]))
	}
}

impl Deserialize for Mentions {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("mentions object"))?;
		let user_ids = object
			.get("user_ids")
			.filter(|v| !v.is_null())
			.map(Vec::<OwnedUserId>::from_json)
			.transpose()?
			.unwrap_or_default();
		Ok(Self {
			user_ids: user_ids.into_iter().collect(),
			room: object.get("room").and_then(Value::as_bool).unwrap_or(false),
		})
	}
}

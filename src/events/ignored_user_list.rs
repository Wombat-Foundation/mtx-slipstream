//! `m.ignored_user_list`.

use alloc::collections::BTreeMap;

use super::wrappers::GlobalAccountDataEvent;
use crate::{
	OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

/// An ignored user. The spec defines no fields; any that appear are kept.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IgnoredUser {
	pub extra: Object,
}

impl Serialize for IgnoredUser {
	fn to_json(&self) -> Value {
		Value::Object(self.extra.clone())
	}
}

impl Deserialize for IgnoredUser {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			extra: value.as_object().cloned().ok_or_else(|| DeError::expected("object"))?,
		})
	}
}

/// The users a user is ignoring.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IgnoredUserListEventContent {
	pub ignored_users: BTreeMap<OwnedUserId, IgnoredUser>,
}

crate::impl_codec_struct!(IgnoredUserListEventContent {
	ignored_users: BTreeMap<OwnedUserId, IgnoredUser>,
});

pub type IgnoredUserListEvent = GlobalAccountDataEvent<IgnoredUserListEventContent>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn ignored_list_round_trips_through_event_wrapper() {
		let event: IgnoredUserListEvent =
			from_str(r#"{"content":{"ignored_users":{"@bad:b":{}}}}"#).unwrap();
		assert!(event.content.ignored_users.contains_key(&OwnedUserId::from("@bad:b")));
		assert_eq!(from_str::<IgnoredUserListEvent>(&to_string(&event)).unwrap(), event);
	}
}

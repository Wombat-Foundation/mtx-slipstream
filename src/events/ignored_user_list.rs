//! `m.ignored_user_list`.

use alloc::collections::BTreeMap;

use super::wrappers::GlobalAccountDataEvent;
use crate::{
	OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

/// An ignored user. The spec defines no fields; any that appear are kept.
#[derive(Debug, Default, Eq, PartialEq)]
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
#[derive(Debug, Default, Eq, PartialEq)]
pub struct IgnoredUserListEventContent {
	pub ignored_users: BTreeMap<OwnedUserId, IgnoredUser>,
}

impl crate::codec::Serialize for IgnoredUserListEventContent {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(ignored_users),
			crate::endpoint::enc(&self.ignored_users),
		)])
	}
}
impl crate::codec::Deserialize for IgnoredUserListEventContent {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(IgnoredUserListEventContent)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			ignored_users: input.body(stringify!(ignored_users))?,
		})
	}
}

pub type IgnoredUserListEvent = GlobalAccountDataEvent<IgnoredUserListEventContent>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn ignored_list_round_trips_through_event_wrapper() {
		let event: IgnoredUserListEvent =
			from_str(r#"{"content":{"ignored_users":{"@bad:b":{}}}}"#).unwrap();
		assert!(event.content.ignored_users.contains_key(&OwnedUserId::parse("@bad:b").unwrap()));
		assert_eq!(from_str::<IgnoredUserListEvent>(&to_string(&event)).unwrap(), event);
	}
}

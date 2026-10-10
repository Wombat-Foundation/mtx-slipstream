//! `m.typing`.

use alloc::vec::Vec;

use crate::OwnedUserId;

/// The users currently typing in a room.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct TypingEventContent {
	pub user_ids: Vec<OwnedUserId>,
}

impl TypingEventContent {
	#[must_use]
	pub fn new(user_ids: Vec<OwnedUserId>) -> Self {
		Self {
			user_ids,
		}
	}
}

impl crate::codec::Serialize for TypingEventContent {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(user_ids),
			crate::endpoint::enc(&self.user_ids),
		)])
	}
}
impl crate::codec::Deserialize for TypingEventContent {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(TypingEventContent)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			user_ids: input.body(stringify!(user_ids))?,
		})
	}
}

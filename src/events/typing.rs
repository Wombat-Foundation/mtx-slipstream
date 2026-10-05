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

crate::impl_codec_struct!(TypingEventContent { user_ids: Vec<OwnedUserId> });

//! Resolved power levels for permission checks.

use alloc::collections::BTreeMap;

use crate::{
	Int, OwnedUserId, UserId,
	events::{
		StateEventType, TimelineEventType, room::power_levels::RoomPowerLevelsEventContent,
	},
};

/// The power levels of a room, as used to decide who may do what.
#[derive(Clone, Debug)]
pub struct RoomPowerLevels {
	ban: Int,
	events: BTreeMap<TimelineEventType, Int>,
	events_default: Int,
	invite: Int,
	kick: Int,
	redact: Int,
	state_default: Int,
	users: BTreeMap<OwnedUserId, Int>,
	users_default: Int,
}

impl From<RoomPowerLevelsEventContent> for RoomPowerLevels {
	fn from(content: RoomPowerLevelsEventContent) -> Self {
		Self {
			ban: content.ban,
			events: content.events,
			events_default: content.events_default,
			invite: content.invite,
			kick: content.kick,
			redact: content.redact,
			state_default: content.state_default,
			users: content.users,
			users_default: content.users_default,
		}
	}
}

impl RoomPowerLevels {
	/// The power level of `user_id`.
	#[must_use]
	pub fn for_user(&self, user_id: &UserId) -> Int {
		self.users.get(user_id).copied().unwrap_or(self.users_default)
	}

	#[must_use]
	pub fn user_can_ban(&self, user_id: &UserId) -> bool {
		self.for_user(user_id) >= self.ban
	}

	#[must_use]
	pub fn user_can_invite(&self, user_id: &UserId) -> bool {
		self.for_user(user_id) >= self.invite
	}

	#[must_use]
	pub fn user_can_kick(&self, user_id: &UserId) -> bool {
		self.for_user(user_id) >= self.kick
	}

	/// Whether `user_id` may redact events sent by other users.
	#[must_use]
	pub fn user_can_redact_event_of_other(&self, user_id: &UserId) -> bool {
		self.for_user(user_id) >= self.redact
			&& self.user_can_send_message(user_id, TimelineEventType::RoomRedaction)
	}

	/// Whether `user_id` may redact events they sent themselves.
	#[must_use]
	pub fn user_can_redact_own_event(&self, user_id: &UserId) -> bool {
		self.for_user(user_id) >= self.event_level(&TimelineEventType::RoomRedaction, false)
	}

	#[must_use]
	pub fn user_can_send_state(&self, user_id: &UserId, event_type: impl AsRef<str>) -> bool {
		let kind = TimelineEventType::from(event_type.as_ref());
		self.for_user(user_id) >= self.event_level(&kind, true)
	}

	#[must_use]
	pub fn user_can_send_message(&self, user_id: &UserId, event_type: impl AsRef<str>) -> bool {
		let kind = TimelineEventType::from(event_type.as_ref());
		self.for_user(user_id) >= self.event_level(&kind, false)
	}

	/// Whether `user_id` may change the power level of `target`: they need
	/// permission to send `m.room.power_levels`, and may only touch users of
	/// lower power than their own, or themselves.
	#[must_use]
	pub fn user_can_change_user_power_level(&self, user_id: &UserId, target: &UserId) -> bool {
		self.user_can_send_state(user_id, StateEventType::RoomPowerLevels)
			&& (user_id == target || self.for_user(user_id) > self.for_user(target))
	}

	fn event_level(&self, kind: &TimelineEventType, state: bool) -> Int {
		self.events.get(kind).copied().unwrap_or(if state {
			self.state_default
		} else {
			self.events_default
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn levels() -> RoomPowerLevels {
		let mut content = RoomPowerLevelsEventContent::new();
		content.users.insert(OwnedUserId::from("@admin:x"), 100);
		content.users.insert(OwnedUserId::from("@mod:x"), 50);
		RoomPowerLevels::from(content)
	}

	#[test]
	fn admin_outranks_moderator() {
		let pl = levels();
		let (admin, moderator) = (OwnedUserId::from("@admin:x"), OwnedUserId::from("@mod:x"));
		assert!(pl.user_can_change_user_power_level(&admin, &moderator));
		assert!(!pl.user_can_change_user_power_level(&moderator, &admin));
		assert!(pl.user_can_change_user_power_level(&admin, &admin));
	}

	#[test]
	fn default_user_cannot_send_state() {
		let pl = levels();
		let nobody = OwnedUserId::from("@nobody:x");
		assert!(!pl.user_can_send_state(&nobody, StateEventType::RoomName));
		assert!(pl.user_can_send_state(&OwnedUserId::from("@mod:x"), StateEventType::RoomName));
	}
}

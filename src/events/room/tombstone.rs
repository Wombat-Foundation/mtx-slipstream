use alloc::string::String;

use crate::{OwnedRoomId, impl_codec_struct};

/// `m.room.tombstone`: the room has been replaced by another.
#[derive(Debug, Default)]
pub struct RoomTombstoneEventContent {
	pub body: String,
	pub replacement_room: OwnedRoomId,
}

impl RoomTombstoneEventContent {
	#[must_use]
	pub fn new(body: String, replacement_room: OwnedRoomId) -> Self {
		Self {
			body,
			replacement_room,
		}
	}
}

impl_codec_struct!(RoomTombstoneEventContent {
	body: String,
	replacement_room: OwnedRoomId,
});

impl crate::events::EventContent for RoomTombstoneEventContent {
	type EventType = crate::events::StateEventType;

	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomTombstone
	}
}

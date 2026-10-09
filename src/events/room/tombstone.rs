use alloc::string::String;

use crate::OwnedRoomId;

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

impl crate::codec::Serialize for RoomTombstoneEventContent {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(body), crate::endpoint::enc(&self.body)),
			(stringify!(replacement_room), crate::endpoint::enc(&self.replacement_room)),
		])
	}
}
impl crate::codec::Deserialize for RoomTombstoneEventContent {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(RoomTombstoneEventContent)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			body: input.body(stringify!(body))?,
			replacement_room: input.body(stringify!(replacement_room))?,
		})
	}
}

impl crate::events::EventContent for RoomTombstoneEventContent {
	type EventType = crate::events::StateEventType;

	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomTombstone
	}
}

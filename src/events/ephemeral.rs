//! Ephemeral room events as delivered in sync and federation.

use crate::{
	codec::{DeError, Deserialize, Serialize},
	events::{SyncEphemeralRoomEvent, receipt::ReceiptEventContent, typing::TypingEventContent},
	json::Value,
};

pub type SyncReceiptEvent = SyncEphemeralRoomEvent<ReceiptEventContent>;
pub type SyncTypingEvent = SyncEphemeralRoomEvent<TypingEventContent>;

/// An ephemeral event of any known type.
#[derive(Debug)]
pub enum AnySyncEphemeralRoomEvent {
	Receipt(SyncReceiptEvent),
	Typing(SyncTypingEvent),
	/// An event type this crate does not model, kept verbatim.
	Unknown(Value),
}

impl Default for AnySyncEphemeralRoomEvent {
	fn default() -> Self {
		Self::Unknown(Value::Null)
	}
}

impl Serialize for AnySyncEphemeralRoomEvent {
	fn to_json(&self) -> Value {
		match self {
			Self::Receipt(event) => event.to_json(),
			Self::Typing(event) => event.to_json(),
			Self::Unknown(value) => value.clone(),
		}
	}
}

impl Deserialize for AnySyncEphemeralRoomEvent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let kind = value
			.as_object()
			.ok_or_else(|| DeError::expected("event object"))?
			.get("type")
			.and_then(Value::as_str);
		Ok(match kind {
			Some("m.receipt") => Self::Receipt(Deserialize::from_json(value)?),
			Some("m.typing") => Self::Typing(Deserialize::from_json(value)?),
			_ => Self::Unknown(value.clone()),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn sync_typing_event_carries_its_type() {
		let event = SyncTypingEvent {
			content: TypingEventContent::new(alloc::vec![crate::OwnedUserId::from("@a:x")]),
		};
		let text = to_string(&event);
		assert!(text.contains("\"type\":\"m.typing\""), "{text}");
		assert!(matches!(
			from_str::<AnySyncEphemeralRoomEvent>(&text).unwrap(),
			AnySyncEphemeralRoomEvent::Typing(_)
		));
		assert!(matches!(
			from_str::<AnySyncEphemeralRoomEvent>(r#"{"type":"x","content":{}}"#).unwrap(),
			AnySyncEphemeralRoomEvent::Unknown(_)
		));
	}
}

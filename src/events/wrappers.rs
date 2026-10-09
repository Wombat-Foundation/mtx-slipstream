//! Generic event wrappers: a content value with the surrounding event envelope.

use crate::{
	OwnedRoomId,
	codec::{DeError, Deserialize, Serialize},
	endpoint::Input,
	json::{Object, Value},
};

/// Event content with a fixed `type`, written alongside it in the envelope.
pub trait StaticEventContent {
	const TYPE: &'static str;
}

impl StaticEventContent for crate::events::typing::TypingEventContent {
	const TYPE: &'static str = "m.typing";
}
impl StaticEventContent for crate::events::receipt::ReceiptEventContent {
	const TYPE: &'static str = "m.receipt";
}
impl StaticEventContent for crate::events::push_rules::PushRulesEventContent {
	const TYPE: &'static str = "m.push_rules";
}
impl StaticEventContent for crate::events::direct::DirectEventContent {
	const TYPE: &'static str = "m.direct";
}
impl StaticEventContent for crate::events::ignored_user_list::IgnoredUserListEventContent {
	const TYPE: &'static str = "m.ignored_user_list";
}
impl StaticEventContent for crate::events::tag::TagEventContent {
	const TYPE: &'static str = "m.tag";
}

/// A global account-data event.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct GlobalAccountDataEvent<T> {
	pub content: T,
}
impl<T: Serialize + StaticEventContent> Serialize for GlobalAccountDataEvent<T> {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("type".into(), Value::String(T::TYPE.into()));
		object.insert("content".into(), self.content.to_json());
		Value::Object(object)
	}
}
impl<T: Deserialize> Deserialize for GlobalAccountDataEvent<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let event = Self {
			content: input.body("content")?,
		};
		input.finish()?;
		Ok(event)
	}
}

/// A per-room account-data event.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct RoomAccountDataEvent<T> {
	pub content: T,
}
impl<T: Serialize + StaticEventContent> Serialize for RoomAccountDataEvent<T> {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("type".into(), Value::String(T::TYPE.into()));
		object.insert("content".into(), self.content.to_json());
		Value::Object(object)
	}
}
impl<T: Deserialize> Deserialize for RoomAccountDataEvent<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let event = Self {
			content: input.body("content")?,
		};
		input.finish()?;
		Ok(event)
	}
}

/// An ephemeral room event as delivered in sync.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct SyncEphemeralRoomEvent<T> {
	pub content: T,
}
impl<T: Serialize + StaticEventContent> Serialize for SyncEphemeralRoomEvent<T> {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("type".into(), Value::String(T::TYPE.into()));
		object.insert("content".into(), self.content.to_json());
		Value::Object(object)
	}
}
impl<T: Deserialize> Deserialize for SyncEphemeralRoomEvent<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let event = Self {
			content: input.body("content")?,
		};
		input.finish()?;
		Ok(event)
	}
}

/// An ephemeral room event with its room.
#[derive(Debug, Default)]
pub struct EphemeralRoomEvent<T> {
	pub content: T,
	pub room_id: OwnedRoomId,
}

impl<T: Serialize + StaticEventContent> Serialize for EphemeralRoomEvent<T> {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("type".into(), Value::String(T::TYPE.into()));
		object.insert("content".into(), self.content.to_json());
		object.insert("room_id".into(), self.room_id.to_json());
		Value::Object(object)
	}
}

impl<T: Deserialize> Deserialize for EphemeralRoomEvent<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let event = Self {
			content: input.body("content")?,
			room_id: input.body("room_id")?,
		};
		input.finish()?;
		Ok(event)
	}
}

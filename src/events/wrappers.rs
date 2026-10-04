//! Generic event wrappers: a content value with the surrounding event envelope.

use crate::{
	OwnedRoomId,
	codec::{DeError, Deserialize, Serialize},
	endpoint::Input,
	json::{Object, Value},
};

macro_rules! content_event {
	($(#[$doc:meta])* $name:ident) => {
		$(#[$doc])*
		#[derive(Clone, Debug, Default, Eq, PartialEq)]
		pub struct $name<T> {
			pub content: T,
		}

		impl<T: Serialize> Serialize for $name<T> {
			fn to_json(&self) -> Value {
				let mut object = Object::new();
				object.insert("content".into(), self.content.to_json());
				Value::Object(object)
			}
		}

		impl<T: Deserialize> Deserialize for $name<T> {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				let input = Input::new(&[], &[], Some(value));
				let event = Self { content: input.body("content")? };
				input.finish()?;
				Ok(event)
			}
		}
	};
}

content_event!(
	/// A global account-data event.
	GlobalAccountDataEvent
);
content_event!(
	/// A per-room account-data event.
	RoomAccountDataEvent
);
content_event!(
	/// An ephemeral room event as delivered in sync.
	SyncEphemeralRoomEvent
);

/// An ephemeral room event with its room.
#[derive(Clone, Debug, Default)]
pub struct EphemeralRoomEvent<T> {
	pub content: T,
	pub room_id: OwnedRoomId,
}

impl<T: Serialize> Serialize for EphemeralRoomEvent<T> {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
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

use crate::{
	OwnedEventId,
	codec::{DeError, Deserialize, Serialize},
	endpoint::Input,
	json::{Object, Value},
};

#[derive(Clone, Debug)]
pub struct FullyReadEventContent {
	pub event_id: OwnedEventId,
}
impl Serialize for FullyReadEventContent {
	fn to_json(&self) -> Value {
		Value::Object([(String::from("event_id"), self.event_id.to_json())].into_iter().collect())
	}
}
impl Deserialize for FullyReadEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let event_id = value
			.as_object()
			.and_then(|object| object.get("event_id"))
			.ok_or_else(|| DeError::expected("event_id"))?;
		Ok(Self {
			event_id: Deserialize::from_json(event_id)?,
		})
	}
}

#[derive(Clone, Debug)]
pub struct FullyReadEvent {
	pub content: FullyReadEventContent,
}
impl Serialize for FullyReadEvent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("type".into(), Value::String("m.fully_read".into()));
		object.insert("content".into(), self.content.to_json());
		Value::Object(object)
	}
}
impl Deserialize for FullyReadEvent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let event = Self {
			content: input.body("content")?,
		};
		input.finish()?;
		Ok(event)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};
	#[test]
	fn fully_read_event_round_trips_with_type() {
		let json = r#"{"content":{"event_id":"$event:example.org"},"type":"m.fully_read"}"#;
		let event: FullyReadEvent = from_str(json).unwrap();
		assert_eq!(to_string(&event), json);
	}
}

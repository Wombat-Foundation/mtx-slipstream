//! Delayed events (MSC4140): the client-visible data and the stored record.

use alloc::string::String;
use core::time::Duration;
use std::time::SystemTime;

use crate::{
	MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedUserId,
	api::client::error::StandardErrorBody,
	codec::{DeError, Deserialize, Serialize},
	endpoint::object_from,
	events::TimelineEventType,
	json::{Object, Value},
	sswire::Raw,
};

/// What a client asks to do with a scheduled delayed event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateAction {
	Restart,
	Send,
	Cancel,
}
crate::impl_codec_enum!(UpdateAction {
	Restart => "restart",
	Send => "send",
	Cancel => "cancel",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DelayedEventStatus {
	Scheduled,
	Send,
	Cancel,
	Error,
}
crate::impl_codec_enum!(DelayedEventStatus {
	Scheduled => "scheduled",
	Send => "send",
	Cancel => "cancel",
	Error => "error",
});

/// The content of an event waiting to be sent, kept as raw JSON.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct AnyTimelineEventContent(pub Value);

impl Serialize for AnyTimelineEventContent {
	fn to_json(&self) -> Value {
		self.0.clone()
	}
}
impl Deserialize for AnyTimelineEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self(value.clone()))
	}
}

fn required<'a>(object: &'a Object, name: &str) -> Result<&'a Value, DeError> {
	object.get(name).ok_or_else(|| DeError(alloc::format!("missing `{name}`")))
}

fn optional<T: Deserialize>(object: &Object, name: &str) -> Result<Option<T>, DeError> {
	object.get(name).filter(|v| !v.is_null()).map(T::from_json).transpose()
}

/// A delayed event as reported to clients.
#[derive(Debug)]
pub struct DelayedEventData {
	/// The ID of the delayed event.
	pub delay_id: String,
	/// The room the event will be sent to.
	pub room_id: OwnedRoomId,
	pub event_type: TimelineEventType,
	/// The state key if the event is a state event.
	pub state_key: Option<String>,
	pub content: Raw<AnyTimelineEventContent>,
	/// How long the server waits before sending.
	pub delay: Duration,
	/// When the event was scheduled or last restarted.
	pub running_since: MilliSecondsSinceUnixEpoch,
	/// Why it was not sent, for finalized events that failed.
	pub error: Option<StandardErrorBody>,
	/// The ID the event got, for events that were sent.
	pub event_id: Option<OwnedEventId>,
	/// When the event was finalized (sent, failed or cancelled).
	pub finalized_ts: Option<MilliSecondsSinceUnixEpoch>,
}

impl DelayedEventData {
	#[must_use]
	pub fn new(
		delay_id: String,
		room_id: OwnedRoomId,
		event_type: TimelineEventType,
		state_key: Option<String>,
		content: Raw<AnyTimelineEventContent>,
		delay: Duration,
		running_since: MilliSecondsSinceUnixEpoch,
	) -> Self {
		Self {
			delay_id,
			room_id,
			event_type,
			state_key,
			content,
			delay,
			running_since,
			error: None,
			event_id: None,
			finalized_ts: None,
		}
	}

	#[must_use]
	pub fn status(&self) -> DelayedEventStatus {
		if self.finalized_ts.is_none() {
			DelayedEventStatus::Scheduled
		} else if self.event_id.is_some() {
			DelayedEventStatus::Send
		} else if self.error.is_some() {
			DelayedEventStatus::Error
		} else {
			DelayedEventStatus::Cancel
		}
	}
}

impl Serialize for DelayedEventData {
	fn to_json(&self) -> Value {
		Value::Object(object_from(alloc::vec![
			("delay_id", Value::String(self.delay_id.clone())),
			("room_id", self.room_id.to_json()),
			("type", self.event_type.to_json()),
			("state_key", self.state_key.to_json()),
			("content", self.content.to_json()),
			("delay", self.delay.to_json()),
			("running_since", self.running_since.to_json()),
			("error", self.error.to_json()),
			("event_id", self.event_id.to_json()),
			("finalised_ts", self.finalized_ts.to_json()),
		]))
	}
}

impl Deserialize for DelayedEventData {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("delayed event"))?;
		Ok(Self {
			delay_id: String::from_json(required(object, "delay_id")?)?,
			room_id: OwnedRoomId::from_json(required(object, "room_id")?)?,
			event_type: TimelineEventType::from_json(required(object, "type")?)?,
			state_key: optional(object, "state_key")?,
			content: Raw::from_json(required(object, "content")?)?,
			delay: Duration::from_json(required(object, "delay")?)?,
			running_since: MilliSecondsSinceUnixEpoch::from_json(required(
				object,
				"running_since",
			)?)?,
			error: optional(object, "error")?,
			event_id: optional(object, "event_id")?,
			finalized_ts: optional(object, "finalised_ts")?,
		})
	}
}

/// A delayed event waiting in storage.
///
/// Times are stored in the same shape serde gives them: `running_since` as
/// `{secs_since_epoch, nanos_since_epoch}` and `delay` as `{secs, nanos}`.
#[derive(Debug)]
pub struct ScheduledDelayedEvent {
	pub event_type: TimelineEventType,
	pub state_key: Option<String>,
	pub content: Raw<AnyTimelineEventContent>,
	pub user_id: OwnedUserId,
	pub room_id: OwnedRoomId,
	pub running_since: SystemTime,
	pub delay: Duration,
}

fn parts(object: &Object, secs: &str, nanos: &str) -> Result<(u64, u32), DeError> {
	let secs = u64::from_json(required(object, secs)?)?;
	let nanos = u32::from_json(required(object, nanos)?)?;
	Ok((secs, nanos))
}

fn time_object(secs_name: &str, nanos_name: &str, secs: u64, nanos: u32) -> Value {
	let mut object = Object::new();
	object.insert(secs_name.into(), secs.to_json());
	object.insert(nanos_name.into(), nanos.to_json());
	Value::Object(object)
}

impl Serialize for ScheduledDelayedEvent {
	fn to_json(&self) -> Value {
		let since_epoch =
			self.running_since.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
		Value::Object(object_from(alloc::vec![
			("event_type", self.event_type.to_json()),
			("state_key", self.state_key.to_json()),
			("content", self.content.to_json()),
			("user_id", self.user_id.to_json()),
			("room_id", self.room_id.to_json()),
			(
				"running_since",
				time_object(
					"secs_since_epoch",
					"nanos_since_epoch",
					since_epoch.as_secs(),
					since_epoch.subsec_nanos(),
				),
			),
			(
				"delay",
				time_object("secs", "nanos", self.delay.as_secs(), self.delay.subsec_nanos())
			),
		]))
	}
}

impl Deserialize for ScheduledDelayedEvent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("scheduled event"))?;
		let since = required(object, "running_since")?
			.as_object()
			.ok_or_else(|| DeError::expected("running_since object"))?;
		let (since_secs, since_nanos) = parts(since, "secs_since_epoch", "nanos_since_epoch")?;
		let delay = required(object, "delay")?
			.as_object()
			.ok_or_else(|| DeError::expected("delay object"))?;
		let (delay_secs, delay_nanos) = parts(delay, "secs", "nanos")?;
		Ok(Self {
			event_type: TimelineEventType::from_json(required(object, "event_type")?)?,
			state_key: optional(object, "state_key")?,
			content: Raw::from_json(required(object, "content")?)?,
			user_id: OwnedUserId::from_json(required(object, "user_id")?)?,
			room_id: OwnedRoomId::from_json(required(object, "room_id")?)?,
			running_since: SystemTime::UNIX_EPOCH
				.checked_add(Duration::new(since_secs, since_nanos))
				.ok_or_else(|| DeError::expected("running_since within the system time range"))?,
			delay: Duration::new(delay_secs, delay_nanos),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	fn content() -> Raw<AnyTimelineEventContent> {
		Raw::from_json_text(r#"{"body":"hi"}"#).unwrap()
	}

	#[test]
	fn scheduled_event_round_trips_with_serde_shaped_times() {
		let event = ScheduledDelayedEvent {
			event_type: TimelineEventType::from("m.room.message"),
			state_key: None,
			content: content(),
			user_id: OwnedUserId::parse("@a:x").unwrap(),
			room_id: OwnedRoomId::parse("!r:x").unwrap(),
			running_since: SystemTime::UNIX_EPOCH + Duration::new(100, 5),
			delay: Duration::from_millis(1500),
		};
		let text = to_string(&event);
		assert!(
			text.contains(r#""secs_since_epoch":100"#) && text.contains(r#""nanos":500000000"#)
		);
		let again = from_str::<ScheduledDelayedEvent>(&text).unwrap();
		assert_eq!(again.running_since, event.running_since);
		assert_eq!(again.delay, event.delay);
	}

	#[test]
	fn data_uses_milliseconds_and_derives_status() {
		let mut data = DelayedEventData::new(
			"id".into(),
			OwnedRoomId::parse("!r:x").unwrap(),
			TimelineEventType::from("m.room.message"),
			None,
			content(),
			Duration::from_millis(2500),
			MilliSecondsSinceUnixEpoch(9),
		);
		assert_eq!(data.status(), DelayedEventStatus::Scheduled);
		let text = to_string(&data);
		assert!(text.contains(r#""delay":2500"#) && !text.contains("finalised_ts"));
		data.finalized_ts = Some(MilliSecondsSinceUnixEpoch(10));
		data.event_id = Some(OwnedEventId::parse("$e").unwrap());
		assert_eq!(data.status(), DelayedEventStatus::Send);
		assert!(to_string(&data).contains(r#""finalised_ts":10"#));
		let again = from_str::<DelayedEventData>(&to_string(&data)).unwrap();
		assert_eq!(again.delay, Duration::from_millis(2500));
	}
}

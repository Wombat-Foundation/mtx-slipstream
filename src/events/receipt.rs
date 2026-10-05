use alloc::collections::BTreeMap;

use crate::{
	OwnedEventId, OwnedRoomId, OwnedUserId, UInt,
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum ReceiptType {
	#[default]
	Read,
	ReadPrivate,
	FullyRead,
}
crate::impl_codec_enum!(ReceiptType { Read => "m.read", ReadPrivate => "m.read.private", FullyRead => "m.fully_read" });

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum ReceiptThread {
	#[default]
	Unthreaded,
	Main,
	Thread(OwnedEventId),
}

impl ReceiptThread {
	/// The `thread_id` value: `None` for unthreaded receipts.
	#[must_use]
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::Unthreaded => None,
			Self::Main => Some("main"),
			Self::Thread(root) => Some(root.as_str()),
		}
	}
}

#[derive(Debug, Default)]
pub struct Receipt {
	pub ts: Option<UInt>,
	pub thread: ReceiptThread,
}
#[derive(Debug, Default)]
pub struct ReceiptEventContent(
	pub BTreeMap<OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>>,
);
impl core::ops::Deref for ReceiptEventContent {
	type Target = BTreeMap<OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl core::ops::DerefMut for ReceiptEventContent {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl IntoIterator for ReceiptEventContent {
	type IntoIter = alloc::collections::btree_map::IntoIter<
		OwnedEventId,
		BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>,
	>;
	type Item = (OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>);

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl FromIterator<(OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>)>
	for ReceiptEventContent
{
	fn from_iter<
		I: IntoIterator<
			Item = (OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>),
		>,
	>(
		iter: I,
	) -> Self {
		Self(iter.into_iter().collect())
	}
}

#[derive(Debug)]
pub struct ReceiptEvent {
	pub content: ReceiptEventContent,
	pub room_id: OwnedRoomId,
}

impl Serialize for Receipt {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Some(ts) = &self.ts {
			object.insert("ts".into(), ts.to_json());
		}
		match &self.thread {
			ReceiptThread::Unthreaded => {}
			ReceiptThread::Main => {
				object.insert("thread_id".into(), Value::String("main".into()));
			}
			ReceiptThread::Thread(root) => {
				object.insert("thread_id".into(), root.to_json());
			}
		}
		Value::Object(object)
	}
}

impl Deserialize for Receipt {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("receipt object"))?;
		let ts = object.get("ts").filter(|v| !v.is_null()).map(UInt::from_json).transpose()?;
		let thread = match object.get("thread_id").and_then(Value::as_str) {
			None => ReceiptThread::Unthreaded,
			Some("main") => ReceiptThread::Main,
			Some(root) => ReceiptThread::Thread(OwnedEventId::from(root)),
		};
		Ok(Self {
			ts,
			thread,
		})
	}
}

impl Serialize for ReceiptEventContent {
	fn to_json(&self) -> Value {
		self.0.to_json()
	}
}

impl Deserialize for ReceiptEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Deserialize::from_json(value).map(Self)
	}
}

impl Serialize for ReceiptEvent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("content".into(), self.content.to_json());
		object.insert("room_id".into(), self.room_id.to_json());
		Value::Object(object)
	}
}

impl Deserialize for ReceiptEvent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("receipt event"))?;
		let content = object.get("content").ok_or_else(|| DeError::expected("content"))?;
		let room_id = object.get("room_id").ok_or_else(|| DeError::expected("room_id"))?;
		Ok(Self {
			content: ReceiptEventContent::from_json(content)?,
			room_id: OwnedRoomId::from_json(room_id)?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn receipt_event_round_trips() {
		let text = r#"{"content":{"$e":{"m.read":{"@u:x":{"thread_id":"main","ts":5}}}},"room_id":"!r:x"}"#;
		let event = from_str::<ReceiptEvent>(text).unwrap();
		let receipt = &event.content.0[&OwnedEventId::from("$e")][&ReceiptType::Read]
			[&OwnedUserId::from("@u:x")];
		assert_eq!(receipt.thread, ReceiptThread::Main);
		assert_eq!(receipt.ts, Some(5));
		assert_eq!(to_string(&event), text);
	}
}

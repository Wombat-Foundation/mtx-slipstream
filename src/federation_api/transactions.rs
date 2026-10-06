//! Federation transactions: `send_transaction_message` and its EDUs.

use alloc::string::String;

use crate::{
	codec::{DeError, Deserialize, Serialize},
	endpoint::Input,
	json::{Object, Value},
};

pub mod edu {
	use alloc::{collections::BTreeMap, string::String, vec::Vec};

	use crate::{
		OwnedDeviceId, OwnedEventId, OwnedRoomId, OwnedTransactionId, OwnedUserId, UInt,
		codec::{DeError, Deserialize, Serialize},
		endpoint::Input,
		events::{presence::PresenceState, receipt::Receipt},
		json::{Object, Value},
		sswire::Raw,
	};

	/// A user's presence update.
	#[derive(Debug)]
	pub struct PresenceUpdate {
		pub user_id: OwnedUserId,
		pub presence: PresenceState,
		pub status_msg: Option<String>,
		pub currently_active: bool,
		pub last_active_ago: UInt,
	}

	crate::impl_codec_struct!(PresenceUpdate {
		user_id: OwnedUserId,
		presence: PresenceState,
		status_msg: Option<String>,
		currently_active: bool,
		last_active_ago: UInt,
	});

	/// Presence updates from one server.
	#[derive(Debug, Default)]
	pub struct PresenceContent {
		pub push: Vec<PresenceUpdate>,
	}

	crate::impl_codec_struct!(PresenceContent { push: Vec<PresenceUpdate> });

	/// Read-receipt data for one user.
	#[derive(Debug)]
	pub struct ReceiptData {
		pub data: Receipt,
		pub event_ids: Vec<OwnedEventId>,
	}

	crate::impl_codec_struct!(ReceiptData { data: Receipt, event_ids: Vec<OwnedEventId> });

	/// Receipts in one room.
	#[derive(Debug, Default)]
	pub struct ReceiptMap {
		pub read: BTreeMap<OwnedUserId, ReceiptData>,
	}

	impl Serialize for ReceiptMap {
		fn to_json(&self) -> Value {
			let mut object = Object::new();
			object.insert("m.read".into(), self.read.to_json());
			Value::Object(object)
		}
	}

	impl Deserialize for ReceiptMap {
		fn from_json(value: &Value) -> Result<Self, DeError> {
			let input = Input::new(&[], &[], Some(value));
			let read: Option<BTreeMap<OwnedUserId, ReceiptData>> = input.body("m.read")?;
			Ok(Self {
				read: read.unwrap_or_default(),
			})
		}
	}

	/// Receipts across rooms.
	#[derive(Debug, Default)]
	pub struct ReceiptContent {
		pub receipts: BTreeMap<OwnedRoomId, ReceiptMap>,
	}

	impl Serialize for ReceiptContent {
		fn to_json(&self) -> Value {
			self.receipts.to_json()
		}
	}

	impl Deserialize for ReceiptContent {
		fn from_json(value: &Value) -> Result<Self, DeError> {
			Ok(Self {
				receipts: BTreeMap::from_json(value)?,
			})
		}
	}

	/// A user starting or stopping typing.
	#[derive(Debug)]
	pub struct TypingContent {
		pub room_id: OwnedRoomId,
		pub user_id: OwnedUserId,
		pub typing: bool,
	}

	impl TypingContent {
		#[must_use]
		pub fn new(room_id: OwnedRoomId, user_id: OwnedUserId, typing: bool) -> Self {
			Self {
				room_id,
				user_id,
				typing,
			}
		}
	}

	crate::impl_codec_struct!(TypingContent {
		room_id: OwnedRoomId,
		user_id: OwnedUserId,
		typing: bool,
	});

	/// A device list change.
	#[derive(Debug)]
	pub struct DeviceListUpdateContent {
		pub user_id: OwnedUserId,
		pub device_id: OwnedDeviceId,
		pub device_display_name: Option<String>,
		pub stream_id: UInt,
		pub prev_id: Vec<UInt>,
		pub deleted: Option<bool>,
		pub keys: Option<Raw<Value>>,
	}

	crate::impl_codec_struct!(DeviceListUpdateContent {
		user_id: OwnedUserId,
		device_id: OwnedDeviceId,
		device_display_name: Option<String>,
		stream_id: UInt,
		prev_id: Vec<UInt>,
		deleted: Option<bool>,
		keys: Option<Raw<Value>>,
	});

	/// A device targeted by a to-device message, or all of a user's devices.
	#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
	pub enum DeviceIdOrAllDevices {
		DeviceId(OwnedDeviceId),
		AllDevices,
	}

	impl Serialize for DeviceIdOrAllDevices {
		fn to_json(&self) -> Value {
			Value::String(match self {
				Self::DeviceId(id) => id.as_str().into(),
				Self::AllDevices => "*".into(),
			})
		}
	}

	impl Deserialize for DeviceIdOrAllDevices {
		fn from_json(value: &Value) -> Result<Self, DeError> {
			match value.as_str() {
				Some("*") => Ok(Self::AllDevices),
				Some(id) => Ok(Self::DeviceId(OwnedDeviceId::from(id))),
				None => Err(DeError::expected("device ID")),
			}
		}
	}

	/// Messages from `sender` to devices, by user and device.
	pub type DirectDeviceMessages =
		BTreeMap<OwnedUserId, BTreeMap<DeviceIdOrAllDevices, Raw<Value>>>;

	/// Direct-to-device messages.
	#[derive(Debug)]
	pub struct DirectDeviceContent {
		pub sender: OwnedUserId,
		pub ev_type: String,
		pub message_id: OwnedTransactionId,
		pub messages: DirectDeviceMessages,
	}

	crate::impl_codec_struct!(DirectDeviceContent {
		sender: OwnedUserId,
		ev_type: String,
		message_id: OwnedTransactionId,
		messages: DirectDeviceMessages,
	});

	/// Cross-signing key changes.
	#[derive(Debug)]
	pub struct SigningKeyUpdateContent {
		pub user_id: OwnedUserId,
		pub master_key: Option<Raw<Value>>,
		pub self_signing_key: Option<Raw<Value>>,
	}

	crate::impl_codec_struct!(SigningKeyUpdateContent {
		user_id: OwnedUserId,
		master_key: Option<Raw<Value>>,
		self_signing_key: Option<Raw<Value>>,
	});

	/// An ephemeral data unit sent in a transaction.
	#[derive(Debug)]
	pub enum Edu {
		Presence(PresenceContent),
		Receipt(ReceiptContent),
		Typing(TypingContent),
		DeviceListUpdate(DeviceListUpdateContent),
		DirectToDevice(DirectDeviceContent),
		SigningKeyUpdate(SigningKeyUpdateContent),
		/// An EDU type this server does not understand, kept whole.
		_Custom(Value),
	}

	impl Serialize for Edu {
		fn to_json(&self) -> Value {
			let (kind, content) = match self {
				Self::Presence(c) => ("m.presence", c.to_json()),
				Self::Receipt(c) => ("m.receipt", c.to_json()),
				Self::Typing(c) => ("m.typing", c.to_json()),
				Self::DeviceListUpdate(c) => ("m.device_list_update", c.to_json()),
				Self::DirectToDevice(c) => ("m.direct_to_device", c.to_json()),
				Self::SigningKeyUpdate(c) => ("m.signing_key_update", c.to_json()),
				Self::_Custom(value) => return value.clone(),
			};
			let mut object = Object::new();
			object.insert("edu_type".into(), Value::String(kind.into()));
			object.insert("content".into(), content);
			Value::Object(object)
		}
	}

	impl Deserialize for Edu {
		fn from_json(value: &Value) -> Result<Self, DeError> {
			let input = Input::new(&[], &[], Some(value));
			let kind: String = input.body("edu_type")?;
			let content: Value = input.body("content")?;
			Ok(match kind.as_str() {
				"m.presence" => Self::Presence(Deserialize::from_json(&content)?),
				"m.receipt" => Self::Receipt(Deserialize::from_json(&content)?),
				"m.typing" => Self::Typing(Deserialize::from_json(&content)?),
				"m.device_list_update" => {
					Self::DeviceListUpdate(Deserialize::from_json(&content)?)
				}
				"m.direct_to_device" => Self::DirectToDevice(Deserialize::from_json(&content)?),
				"m.signing_key_update" => {
					Self::SigningKeyUpdate(Deserialize::from_json(&content)?)
				}
				_ => Self::_Custom(value.clone()),
			})
		}
	}
}

/// The outcome of processing one PDU: `Ok(())` or an error message.
pub type PduResult = Result<(), String>;

impl Serialize for PduResult {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Err(message) = self {
			object.insert("error".into(), Value::String(message.clone()));
		}
		Value::Object(object)
	}
}

impl Deserialize for PduResult {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let error: Option<String> = input.body("error")?;
		Ok(error.map_or(Ok(()), Err))
	}
}

pub mod send_transaction_message {
	pub mod v1 {
		use alloc::{collections::BTreeMap, vec::Vec};

		use super::super::{PduResult, edu::Edu};
		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedServerName, OwnedTransactionId,
			endpoint, federation_api::RawPdu, sswire::Raw,
		};

		endpoint! {
			method: "PUT", path: "/_matrix/federation/v1/send/{transaction_id}",
			request {
				path { transaction_id: OwnedTransactionId }
				query {}
				body {
					origin: OwnedServerName,
					origin_server_ts: MilliSecondsSinceUnixEpoch,
					pdus: Vec<RawPdu>,
					edus: Vec<Raw<Edu>>
				}
			}
			response { pdus: BTreeMap<OwnedEventId, PduResult> }
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::{collections::BTreeMap, string::ToString};

	use super::*;
	use crate::{
		OwnedEventId, OwnedRoomId, OwnedUserId,
		codec::{from_str, to_string},
	};

	#[test]
	fn edu_round_trips_by_type() {
		let typing = edu::Edu::Typing(edu::TypingContent::new(
			OwnedRoomId::from("!r:b"),
			OwnedUserId::from("@u:b"),
			true,
		));
		let json = to_string(&typing);
		assert!(json.contains("m.typing"));
		assert!(matches!(from_str::<edu::Edu>(&json).unwrap(), edu::Edu::Typing(t) if t.typing));
		assert!(matches!(
			from_str::<edu::Edu>(r#"{"edu_type":"org.example","content":{}}"#).unwrap(),
			edu::Edu::_Custom(_)
		));
	}

	#[test]
	fn receipts_and_pdu_results_decode() {
		let content: edu::ReceiptContent = from_str(
			r#"{"!r:b":{"m.read":{"@u:b":{"data":{"ts":5,"thread_id":"main"},"event_ids":["$e"]}}}}"#,
		)
		.unwrap();
		let map = &content.receipts[&OwnedRoomId::from("!r:b")];
		let data = &map.read[&OwnedUserId::from("@u:b")];
		assert_eq!(data.data.ts, Some(5));
		assert_eq!(data.event_ids.len(), 1);

		let results: BTreeMap<OwnedEventId, PduResult> =
			from_str(r#"{"$a":{},"$b":{"error":"bad"}}"#).unwrap();
		assert_eq!(results[&OwnedEventId::from("$a")], Ok(()));
		assert_eq!(results[&OwnedEventId::from("$b")], Err("bad".to_string()));
	}
}

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

	impl crate::codec::Serialize for PresenceUpdate {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
				(stringify!(presence), crate::endpoint::enc(&self.presence)),
				(stringify!(status_msg), crate::endpoint::enc(&self.status_msg)),
				(stringify!(currently_active), crate::endpoint::enc(&self.currently_active)),
				(stringify!(last_active_ago), crate::endpoint::enc(&self.last_active_ago)),
			])
		}
	}
	impl crate::codec::Deserialize for PresenceUpdate {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(PresenceUpdate)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				user_id: input.body(stringify!(user_id))?,
				presence: input.body(stringify!(presence))?,
				status_msg: input.body(stringify!(status_msg))?,
				currently_active: input.body(stringify!(currently_active))?,
				last_active_ago: input.body(stringify!(last_active_ago))?,
			})
		}
	}

	/// Presence updates from one server.
	#[derive(Debug, Default)]
	pub struct PresenceContent {
		pub push: Vec<PresenceUpdate>,
	}

	impl crate::codec::Serialize for PresenceContent {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [(
				stringify!(push),
				crate::endpoint::enc(&self.push),
			)])
		}
	}
	impl crate::codec::Deserialize for PresenceContent {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(PresenceContent)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				push: input.body(stringify!(push))?,
			})
		}
	}

	/// Read-receipt data for one user.
	#[derive(Debug)]
	pub struct ReceiptData {
		pub data: Receipt,
		pub event_ids: Vec<OwnedEventId>,
	}

	impl crate::codec::Serialize for ReceiptData {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				(stringify!(data), crate::endpoint::enc(&self.data)),
				(stringify!(event_ids), crate::endpoint::enc(&self.event_ids)),
			])
		}
	}
	impl crate::codec::Deserialize for ReceiptData {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(ReceiptData)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				data: input.body(stringify!(data))?,
				event_ids: input.body(stringify!(event_ids))?,
			})
		}
	}

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

	impl crate::codec::Serialize for TypingContent {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				(stringify!(room_id), crate::endpoint::enc(&self.room_id)),
				(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
				(stringify!(typing), crate::endpoint::enc(&self.typing)),
			])
		}
	}
	impl crate::codec::Deserialize for TypingContent {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(TypingContent)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				room_id: input.body(stringify!(room_id))?,
				user_id: input.body(stringify!(user_id))?,
				typing: input.body(stringify!(typing))?,
			})
		}
	}

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

	impl crate::codec::Serialize for DeviceListUpdateContent {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
				(stringify!(device_id), crate::endpoint::enc(&self.device_id)),
				(
					stringify!(device_display_name),
					crate::endpoint::enc(&self.device_display_name),
				),
				(stringify!(stream_id), crate::endpoint::enc(&self.stream_id)),
				(stringify!(prev_id), crate::endpoint::enc(&self.prev_id)),
				(stringify!(deleted), crate::endpoint::enc(&self.deleted)),
				(stringify!(keys), crate::endpoint::enc(&self.keys)),
			])
		}
	}
	impl crate::codec::Deserialize for DeviceListUpdateContent {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(DeviceListUpdateContent)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				user_id: input.body(stringify!(user_id))?,
				device_id: input.body(stringify!(device_id))?,
				device_display_name: input.body(stringify!(device_display_name))?,
				stream_id: input.body(stringify!(stream_id))?,
				prev_id: input.body(stringify!(prev_id))?,
				deleted: input.body(stringify!(deleted))?,
				keys: input.body(stringify!(keys))?,
			})
		}
	}

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
				Some(id) => Ok(Self::DeviceId(
					OwnedDeviceId::parse(id).map_err(|_| DeError::expected("device ID"))?,
				)),
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

	impl crate::codec::Serialize for DirectDeviceContent {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				(stringify!(sender), crate::endpoint::enc(&self.sender)),
				(stringify!(ev_type), crate::endpoint::enc(&self.ev_type)),
				(stringify!(message_id), crate::endpoint::enc(&self.message_id)),
				(stringify!(messages), crate::endpoint::enc(&self.messages)),
			])
		}
	}
	impl crate::codec::Deserialize for DirectDeviceContent {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(DirectDeviceContent)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				sender: input.body(stringify!(sender))?,
				ev_type: input.body(stringify!(ev_type))?,
				message_id: input.body(stringify!(message_id))?,
				messages: input.body(stringify!(messages))?,
			})
		}
	}

	/// Cross-signing key changes.
	#[derive(Debug)]
	pub struct SigningKeyUpdateContent {
		pub user_id: OwnedUserId,
		pub master_key: Option<Raw<Value>>,
		pub self_signing_key: Option<Raw<Value>>,
	}

	impl crate::codec::Serialize for SigningKeyUpdateContent {
		fn to_json(&self) -> crate::json::Value {
			crate::endpoint::body_object(&mut [
				(stringify!(user_id), crate::endpoint::enc(&self.user_id)),
				(stringify!(master_key), crate::endpoint::enc(&self.master_key)),
				(stringify!(self_signing_key), crate::endpoint::enc(&self.self_signing_key)),
			])
		}
	}
	impl crate::codec::Deserialize for SigningKeyUpdateContent {
		fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
			// A struct is a JSON object; anything else is malformed, not "all defaults".
			if value.as_object().is_none() {
				return Err(crate::codec::DeError::expected(stringify!(SigningKeyUpdateContent)));
			}
			let input = crate::endpoint::Input::body_only(value);
			Ok(Self {
				user_id: input.body(stringify!(user_id))?,
				master_key: input.body(stringify!(master_key))?,
				self_signing_key: input.body(stringify!(self_signing_key))?,
			})
		}
	}

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
			federation_api::RawPdu, sswire::Raw,
		};

		pub struct Request {
			pub transaction_id: OwnedTransactionId,
			pub origin: OwnedServerName,
			pub origin_server_ts: MilliSecondsSinceUnixEpoch,
			pub pdus: Vec<RawPdu>,
			pub edus: Vec<Raw<Edu>>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/federation/v1/send/{transaction_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.transaction_id,
				)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("origin", crate::endpoint::enc(&self.origin)),
						("origin_server_ts", crate::endpoint::enc(&self.origin_server_ts)),
						("pdus", crate::endpoint::enc(&self.pdus)),
						("edus", crate::endpoint::enc(&self.edus)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					transaction_id: input.path()?,
					origin: input.body("origin")?,
					origin_server_ts: input.body("origin_server_ts")?,
					pdus: input.body("pdus")?,
					edus: input.body_or("edus", Vec::new())?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub pdus: BTreeMap<OwnedEventId, PduResult>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("pdus", crate::endpoint::enc(&self.pdus))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					pdus: input.body("pdus")?,
				})
			}
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
			OwnedRoomId::parse("!r:b").unwrap(),
			OwnedUserId::parse("@u:b").unwrap(),
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
		let map = &content.receipts[&OwnedRoomId::parse("!r:b").unwrap()];
		let data = &map.read[&OwnedUserId::parse("@u:b").unwrap()];
		assert_eq!(data.data.ts, Some(5));
		assert_eq!(data.event_ids.len(), 1);

		let results: BTreeMap<OwnedEventId, PduResult> =
			from_str(r#"{"$a":{},"$b":{"error":"bad"}}"#).unwrap();
		assert_eq!(results[&OwnedEventId::parse("$a").unwrap()], Ok(()));
		assert_eq!(results[&OwnedEventId::parse("$b").unwrap()], Err("bad".to_string()));
	}
}

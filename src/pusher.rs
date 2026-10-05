//! Pushers and the push gateway notification API.

use alloc::string::String;

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
	push::PushFormat,
};

/// Identifies a pusher: one app instance on one device.
#[derive(Debug, Eq, PartialEq)]
pub struct PusherIds {
	pub pushkey: String,
	pub app_id: String,
}

impl PusherIds {
	#[must_use]
	pub fn new(pushkey: String, app_id: String) -> Self {
		Self {
			pushkey,
			app_id,
		}
	}
}

/// Delivery settings for an HTTP pusher.
#[derive(Debug, Eq, PartialEq)]
pub struct HttpPusherData {
	pub url: String,
	pub format: Option<PushFormat>,
	/// Extra gateway-specific fields.
	pub data: Object,
}

impl HttpPusherData {
	#[must_use]
	pub fn new(url: String) -> Self {
		Self {
			url,
			format: None,
			data: Object::new(),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum PusherKind {
	Http(HttpPusherData),
	Email(Object),
	Custom {
		kind: String,
		data: Object,
	},
}

impl PusherKind {
	fn name(&self) -> &str {
		match self {
			Self::Http(_) => "http",
			Self::Email(_) => "email",
			Self::Custom {
				kind,
				..
			} => kind,
		}
	}

	fn data_json(&self) -> Value {
		match self {
			Self::Http(http) => {
				let mut object = http.data.clone();
				object.insert("url".into(), Value::String(http.url.clone()));
				if let Some(format) = &http.format {
					object.insert("format".into(), format.to_json());
				}
				Value::Object(object)
			}
			Self::Email(data)
			| Self::Custom {
				data,
				..
			} => Value::Object(data.clone()),
		}
	}

	fn from_parts(kind: &str, data: &Object) -> Result<Self, DeError> {
		Ok(match kind {
			"http" => {
				let mut rest = data.clone();
				let Some(Value::String(url)) = rest.remove("url") else {
					return Err(DeError::expected("pusher data.url"));
				};
				let format =
					rest.remove("format").map(|v| PushFormat::from_json(&v)).transpose()?;
				Self::Http(HttpPusherData {
					url,
					format,
					data: rest,
				})
			}
			"email" => Self::Email(data.clone()),
			other => Self::Custom {
				kind: other.into(),
				data: data.clone(),
			},
		})
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pusher {
	pub ids: PusherIds,
	pub kind: PusherKind,
	pub app_display_name: String,
	pub device_display_name: String,
	pub profile_tag: Option<String>,
	pub lang: String,
}

impl Pusher {
	#[must_use]
	pub fn new(
		ids: PusherIds,
		kind: PusherKind,
		app_display_name: String,
		device_display_name: String,
		lang: String,
	) -> Self {
		Self {
			ids,
			kind,
			app_display_name,
			device_display_name,
			profile_tag: None,
			lang,
		}
	}
}

fn text(object: &Object, name: &str) -> Result<String, DeError> {
	object
		.get(name)
		.and_then(Value::as_str)
		.map(String::from)
		.ok_or_else(|| DeError(alloc::format!("expected string field `{name}`")))
}

impl Serialize for Pusher {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("pushkey".into(), Value::String(self.ids.pushkey.clone()));
		object.insert("app_id".into(), Value::String(self.ids.app_id.clone()));
		object.insert("kind".into(), Value::String(self.kind.name().into()));
		object.insert("app_display_name".into(), Value::String(self.app_display_name.clone()));
		object.insert(
			"device_display_name".into(),
			Value::String(self.device_display_name.clone()),
		);
		if let Some(tag) = &self.profile_tag {
			object.insert("profile_tag".into(), Value::String(tag.clone()));
		}
		object.insert("lang".into(), Value::String(self.lang.clone()));
		object.insert("data".into(), self.kind.data_json());
		Value::Object(object)
	}
}

impl Deserialize for Pusher {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("pusher object"))?;
		let empty = Object::new();
		let data = object.get("data").and_then(Value::as_object).unwrap_or(&empty);
		Ok(Self {
			ids: PusherIds {
				pushkey: text(object, "pushkey")?,
				app_id: text(object, "app_id")?,
			},
			kind: PusherKind::from_parts(&text(object, "kind")?, data)?,
			app_display_name: text(object, "app_display_name")?,
			device_display_name: text(object, "device_display_name")?,
			profile_tag: object.get("profile_tag").and_then(Value::as_str).map(String::from),
			lang: text(object, "lang")?,
		})
	}
}

pub mod set_pusher {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use crate::{
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointRequest, EndpointResponse, Metadata},
			json::{Object, Value},
			pusher::{Pusher, PusherIds},
		};

		#[derive(Debug, Eq, PartialEq)]
		pub struct PusherPostData {
			pub pusher: Pusher,
			/// Whether to keep other pushers with the same app ID.
			pub append: bool,
		}

		#[derive(Debug, Eq, PartialEq)]
		pub enum PusherAction {
			Post(PusherPostData),
			Delete(PusherIds),
		}

		#[derive(Debug)]
		pub struct Request {
			pub action: PusherAction,
		}

		impl Request {
			#[must_use]
			pub fn post(pusher: Pusher) -> Self {
				Self {
					action: PusherAction::Post(PusherPostData {
						pusher,
						append: false,
					}),
				}
			}

			#[must_use]
			pub fn delete(ids: PusherIds) -> Self {
				Self {
					action: PusherAction::Delete(ids),
				}
			}
		}

		const _: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/pushers/set");
		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata::new("POST", "/_matrix/client/v3/pushers/set");

			fn path_args(&self) -> Vec<String> {
				Vec::new()
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				Some(match &self.action {
					PusherAction::Post(post) => {
						let mut value = post.pusher.to_json();
						if let (Value::Object(object), true) = (&mut value, post.append) {
							object.insert("append".into(), Value::Bool(true));
						}
						value
					}
					PusherAction::Delete(ids) => {
						let mut object = Object::new();
						object.insert("pushkey".into(), Value::String(ids.pushkey.clone()));
						object.insert("app_id".into(), Value::String(ids.app_id.clone()));
						object.insert("kind".into(), Value::Null);
						Value::Object(object)
					}
				})
			}

			fn from_parts(
				_path: &[String],
				_query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let body = body.ok_or_else(|| DeError::expected("request body"))?;
				let object = body.as_object().ok_or_else(|| DeError::expected("object"))?;
				let pushkey =
					|name| {
						object.get(name).and_then(Value::as_str).map(String::from).ok_or_else(
							|| DeError(alloc::format!("expected string field `{name}`")),
						)
					};
				let action = if object.get("kind").is_none_or(Value::is_null) {
					PusherAction::Delete(PusherIds {
						pushkey: pushkey("pushkey")?,
						app_id: pushkey("app_id")?,
					})
				} else {
					PusherAction::Post(PusherPostData {
						pusher: Pusher::from_json(body)?,
						append: object.get("append").and_then(Value::as_bool).unwrap_or(false),
					})
				};
				Ok(Self {
					action,
				})
			}
		}

		#[derive(Debug, Default)]
		pub struct Response {}

		impl Response {
			#[must_use]
			pub fn new() -> Self {
				Self {}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(Object::new())
			}

			fn from_body(_body: &Value) -> Result<Self, DeError> {
				Ok(Self {})
			}
		}
	}
}

pub mod get_pushers {
	pub mod v3 {
		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/pushers",
			request { path {} query {} body {} }
			response { pushers: alloc::vec::Vec<crate::pusher::Pusher> }
		}
	}
}

pub mod send_event_notification {
	pub mod v1 {
		use alloc::{string::String, vec::Vec};

		use crate::{
			OwnedRoomId, OwnedUserId, UInt,
			codec::{DeError, Deserialize, Serialize},
			events::TimelineEventType,
			impl_codec_enum, impl_codec_struct,
			json::{Object, Value},
			push::{PushFormat, Tweak},
			serde::Raw,
		};

		#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
		pub enum NotificationPriority {
			#[default]
			High,
			Low,
		}
		impl_codec_enum!(NotificationPriority { High => "high", Low => "low" });

		#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
		pub struct NotificationCounts {
			pub unread: UInt,
			pub missed_calls: UInt,
		}

		impl NotificationCounts {
			#[must_use]
			pub fn new(unread: UInt, missed_calls: UInt) -> Self {
				Self {
					unread,
					missed_calls,
				}
			}

			#[must_use]
			pub fn is_default(&self) -> bool {
				*self == Self::default()
			}
		}
		impl_codec_struct!(NotificationCounts {} default { unread: UInt, missed_calls: UInt });

		/// The `data` object a gateway receives for a device.
		#[derive(Debug, Default, Eq, PartialEq)]
		pub struct PusherData {
			pub format: Option<PushFormat>,
			pub data: Object,
		}

		impl Serialize for PusherData {
			fn to_json(&self) -> Value {
				let mut object = self.data.clone();
				if let Some(format) = &self.format {
					object.insert("format".into(), format.to_json());
				}
				Value::Object(object)
			}
		}
		impl Deserialize for PusherData {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				let mut data = value
					.as_object()
					.ok_or_else(|| DeError::expected("pusher data object"))?
					.clone();
				let format =
					data.remove("format").map(|v| PushFormat::from_json(&v)).transpose()?;
				Ok(Self {
					format,
					data,
				})
			}
		}

		#[derive(Debug, Eq, PartialEq)]
		pub struct Device {
			pub app_id: String,
			pub pushkey: String,
			pub data: PusherData,
			pub tweaks: Vec<Tweak>,
		}

		impl Device {
			#[must_use]
			pub fn new(app_id: String, pushkey: String) -> Self {
				Self {
					app_id,
					pushkey,
					data: PusherData::default(),
					tweaks: Vec::new(),
				}
			}
		}
		impl_codec_struct!(Device { app_id: String, pushkey: String } default {
			data: PusherData,
			tweaks: Vec<Tweak>,
		});

		impl Default for Device {
			fn default() -> Self {
				Self::new(String::new(), String::new())
			}
		}

		#[derive(Debug, Default)]
		pub struct Notification {
			pub event_id: Option<crate::OwnedEventId>,
			pub room_id: Option<OwnedRoomId>,
			pub event_type: Option<TimelineEventType>,
			pub sender: Option<OwnedUserId>,
			pub sender_display_name: Option<String>,
			pub room_name: Option<String>,
			pub room_alias: Option<crate::OwnedRoomAliasId>,
			pub user_is_target: bool,
			pub prio: NotificationPriority,
			pub content: Option<Raw<Value>>,
			pub counts: NotificationCounts,
			pub devices: Vec<Device>,
		}

		impl Notification {
			#[must_use]
			pub fn new(devices: Vec<Device>) -> Self {
				Self {
					devices,
					..Self::default()
				}
			}
		}

		impl Serialize for Notification {
			fn to_json(&self) -> Value {
				let mut object = crate::endpoint::object_from(alloc::vec![
					("event_id", self.event_id.to_json()),
					("room_id", self.room_id.to_json()),
					("type", self.event_type.to_json()),
					("sender", self.sender.to_json()),
					("sender_display_name", self.sender_display_name.to_json()),
					("room_name", self.room_name.to_json()),
					("room_alias", self.room_alias.to_json()),
					("prio", self.prio.to_json()),
					("content", self.content.to_json()),
					("devices", self.devices.to_json()),
				]);
				if self.user_is_target {
					object.insert("user_is_target".into(), Value::Bool(true));
				}
				if !self.counts.is_default() {
					object.insert("counts".into(), self.counts.to_json());
				}
				Value::Object(object)
			}
		}

		impl Deserialize for Notification {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				let input = crate::endpoint::Input::new(&[], &[], Some(value));
				Ok(Self {
					event_id: input.body("event_id")?,
					room_id: input.body("room_id")?,
					event_type: input.body("type")?,
					sender: input.body("sender")?,
					sender_display_name: input.body("sender_display_name")?,
					room_name: input.body("room_name")?,
					room_alias: input.body("room_alias")?,
					user_is_target: input.body_or_default("user_is_target")?,
					prio: input.body("prio").unwrap_or_default(),
					content: input.body("content")?,
					counts: input.body_or_default("counts")?,
					devices: input.body("devices")?,
				})
			}
		}

		crate::endpoint! {
			method: "POST", path: "/_matrix/push/v1/notify",
			request { path {} query {} body { notification: Notification } }
			response { rejected: Vec<String> }
		}

		impl Request {
			#[must_use]
			pub fn new(notification: Notification) -> Self {
				Self {
					notification,
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		codec::{from_str, to_string},
		endpoint::EndpointRequest,
		push::{Action, Tweak},
	};

	fn pusher() -> Pusher {
		let mut http = HttpPusherData::new("https://push.example/_matrix/push/v1/notify".into());
		http.format = Some(PushFormat::EventIdOnly);
		http.data.insert("extra".into(), Value::Bool(true));
		Pusher::new(
			PusherIds::new("key".into(), "app".into()),
			PusherKind::Http(http),
			"App".into(),
			"Phone".into(),
			"en".into(),
		)
	}

	#[test]
	fn pusher_round_trips() {
		let pusher = pusher();
		assert_eq!(from_str::<Pusher>(&to_string(&pusher)).unwrap(), pusher);
	}

	#[test]
	fn set_pusher_distinguishes_post_and_delete() {
		let post = set_pusher::v3::Request::post(pusher());
		let parsed = set_pusher::v3::Request::from_parts(&[], &[], post.body().as_ref()).unwrap();
		assert!(matches!(parsed.action, set_pusher::v3::PusherAction::Post(_)));

		let delete = set_pusher::v3::Request::delete(PusherIds::new("k".into(), "a".into()));
		let body = delete.body().unwrap();
		assert!(to_string(&body).contains("\"kind\":null"));
		let parsed = set_pusher::v3::Request::from_parts(&[], &[], Some(&body)).unwrap();
		assert!(matches!(parsed.action, set_pusher::v3::PusherAction::Delete(_)));
	}

	#[test]
	fn actions_round_trip() {
		for action in [
			Action::Notify,
			Action::SetTweak(Tweak::Sound("default".into())),
			Action::SetTweak(Tweak::Highlight(false)),
		] {
			assert_eq!(from_str::<Action>(&to_string(&action)).unwrap(), action);
		}
		assert_eq!(from_str::<Action>("\"notify\"").unwrap().to_json().as_str(), Some("notify"));
	}

	#[test]
	fn notification_omits_default_counts() {
		use send_event_notification::v1::{Device, Notification};
		let notification = Notification::new(alloc::vec![Device::new("a".into(), "k".into())]);
		let text = to_string(&notification);
		assert!(!text.contains("counts") && text.contains("\"prio\":\"high\""), "{text}");
		assert_eq!(from_str::<Notification>(&text).unwrap().devices.len(), 1);
	}
}

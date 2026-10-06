use crate::{MilliSecondsSinceUnixEpoch, OwnedUserId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum PresenceState {
	Online,
	Unavailable,
	#[default]
	Offline,
	Busy,
}
impl PresenceState {
	#[must_use]
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Online => "online",
			Self::Unavailable => "unavailable",
			Self::Offline => "offline",
			Self::Busy => "org.matrix.msc3026.busy",
		}
	}
}
impl core::fmt::Display for PresenceState {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(self.as_str())
	}
}
crate::impl_codec_enum!(PresenceState {
	Online => "online", Unavailable => "unavailable", Offline => "offline", Busy => "org.matrix.msc3026.busy",
});

#[derive(Debug, Default)]
pub struct PresenceEventContent {
	pub avatar_url: Option<crate::OwnedMxcUri>,
	pub displayname: Option<String>,
	pub last_active_ago: Option<u64>,
	pub currently_active: Option<bool>,
	pub presence: PresenceState,
	pub status_msg: Option<String>,
}

#[derive(Debug)]
pub struct PresenceEvent {
	pub sender: OwnedUserId,
	pub content: PresenceEventContent,
	pub origin_server_ts: Option<MilliSecondsSinceUnixEpoch>,
}

impl crate::codec::Serialize for PresenceEventContent {
	fn to_json(&self) -> crate::json::Value {
		let mut object = crate::json::Object::new();
		object.insert("presence".into(), self.presence.to_json());
		if let Some(v) = &self.avatar_url {
			object.insert("avatar_url".into(), v.to_json());
		}
		if let Some(v) = &self.displayname {
			object.insert("displayname".into(), v.to_json());
		}
		if let Some(v) = &self.last_active_ago {
			object.insert("last_active_ago".into(), v.to_json());
		}
		if let Some(v) = &self.currently_active {
			object.insert("currently_active".into(), v.to_json());
		}
		if let Some(v) = &self.status_msg {
			object.insert("status_msg".into(), v.to_json());
		}
		crate::json::Value::Object(object)
	}
}
impl crate::codec::Deserialize for PresenceEventContent {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		use crate::codec::{DeError, from_value};
		let object = value.as_object().ok_or_else(|| DeError::expected("object"))?;
		let get = |name: &str| object.get(name).filter(|v| !v.is_null());
		Ok(Self {
			// `compat-empty-string-null` in the old build: "" meant no avatar.
			avatar_url: get("avatar_url")
				.filter(|v| v.as_str() != Some(""))
				.map(from_value)
				.transpose()?,
			displayname: get("displayname").map(from_value).transpose()?,
			last_active_ago: get("last_active_ago").map(from_value).transpose()?,
			currently_active: get("currently_active").map(from_value).transpose()?,
			presence: get("presence").map(from_value).transpose()?.unwrap_or_default(),
			status_msg: get("status_msg").map(from_value).transpose()?,
		})
	}
}
impl crate::codec::Serialize for PresenceEvent {
	fn to_json(&self) -> crate::json::Value {
		let mut object = crate::json::Object::new();
		object.insert("sender".into(), self.sender.to_json());
		object.insert("content".into(), self.content.to_json());
		crate::json::Value::Object(object)
	}
}

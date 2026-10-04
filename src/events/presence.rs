use crate::{MilliSecondsSinceUnixEpoch, OwnedUserId};

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum PresenceState {
	Online,
	Unavailable,
	#[default]
	Offline,
	Busy,
}
crate::impl_codec_enum!(PresenceState {
	Online => "online", Unavailable => "unavailable", Offline => "offline", Busy => "busy",
});

#[derive(Clone, Debug, Default)]
pub struct PresenceEventContent {
	pub avatar_url: Option<crate::OwnedMxcUri>,
	pub displayname: Option<String>,
	pub last_active_ago: Option<u64>,
	pub currently_active: Option<bool>,
	pub presence: PresenceState,
}

#[derive(Clone, Debug)]
pub struct PresenceEvent {
	pub sender: OwnedUserId,
	pub content: PresenceEventContent,
	pub origin_server_ts: Option<MilliSecondsSinceUnixEpoch>,
}

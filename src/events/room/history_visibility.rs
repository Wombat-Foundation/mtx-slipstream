#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum HistoryVisibility {
	#[default]
	Shared,
	Invited,
	Joined,
	WorldReadable,
}
crate::impl_codec_enum!(HistoryVisibility { Shared => "shared", Invited => "invited", Joined => "joined", WorldReadable => "world_readable" });
#[derive(Clone, Debug, Default)]
pub struct RoomHistoryVisibilityEventContent {
	pub history_visibility: HistoryVisibility,
}

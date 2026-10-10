#[derive(Debug, Default)]
pub struct RoomCanonicalAliasEventContent {
	pub alias: Option<crate::OwnedRoomAliasId>,
	pub alt_aliases: Vec<crate::OwnedRoomAliasId>,
}

#[derive(Clone, Debug, Default)]
pub struct RoomAvatarEventContent {
	pub url: Option<crate::OwnedMxcUri>,
}

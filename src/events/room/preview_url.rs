#[derive(Clone, Debug)]
pub struct RoomPreviewUrlsEventContent {
	pub disabled: bool,
}
impl Default for RoomPreviewUrlsEventContent {
	fn default() -> Self {
		Self {
			disabled: true,
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct RoomPolicyEventContent {
	pub entity: String,
	pub recommendation: String,
	pub reason: String,
}

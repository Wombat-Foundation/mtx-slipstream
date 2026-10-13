#[derive(Debug, Default)]
pub struct RoomEncryptedEventContent {
	pub algorithm: String,
	pub ciphertext: String,
	pub sender_key: String,
	pub device_id: Option<String>,
	pub session_id: Option<String>,
}
pub use crate::relation_types::Relation;

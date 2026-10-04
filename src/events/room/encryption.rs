#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventEncryptionAlgorithm {
	MegolmV1AesSha2,
}
#[derive(Clone, Debug)]
pub struct RoomEncryptionEventContent {
	pub algorithm: EventEncryptionAlgorithm,
}

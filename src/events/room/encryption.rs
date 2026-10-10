#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventEncryptionAlgorithm {
	MegolmV1AesSha2,
}
#[derive(Debug)]
pub struct RoomEncryptionEventContent {
	pub algorithm: EventEncryptionAlgorithm,
}

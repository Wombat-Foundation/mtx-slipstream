use alloc::string::String;

/// `m.room.policy` content (policy server configuration).
#[derive(Debug, Default)]
pub struct RoomPolicyEventContent {
	pub via: Option<String>,
	pub public_key: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GuestAccess {
	#[default]
	Forbidden,
	CanJoin,
}
crate::impl_codec_enum!(GuestAccess { Forbidden => "forbidden", CanJoin => "can_join" });
#[derive(Debug, Default)]
pub struct RoomGuestAccessEventContent {
	pub guest_access: GuestAccess,
}
impl RoomGuestAccessEventContent {
	#[must_use]
	pub fn new(guest_access: GuestAccess) -> Self {
		Self {
			guest_access,
		}
	}
}
impl crate::events::EventContent for RoomGuestAccessEventContent {
	type EventType = crate::events::StateEventType;
	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomGuestAccess
	}
}

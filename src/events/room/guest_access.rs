#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GuestAccess {
	#[default]
	Forbidden,
	CanJoin,
}
impl crate::codec::Serialize for GuestAccess {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Forbidden => "forbidden",
			Self::CanJoin => "can_join",
		}))
	}
}
impl crate::codec::Deserialize for GuestAccess {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("forbidden") => Ok(Self::Forbidden),
			Some("can_join") => Ok(Self::CanJoin),
			_ => Err(crate::codec::DeError::expected(stringify!(GuestAccess))),
		}
	}
}
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

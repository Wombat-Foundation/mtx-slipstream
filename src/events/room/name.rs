#[derive(Debug, Default)]
pub struct RoomNameEventContent {
	pub name: String,
}
impl RoomNameEventContent {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
		}
	}
}
impl crate::events::EventContent for RoomNameEventContent {
	type EventType = crate::events::StateEventType;
	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomName
	}
}

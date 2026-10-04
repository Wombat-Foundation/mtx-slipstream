#[derive(Clone, Debug, Default)]
pub struct RoomNameEventContent {
	pub name: Option<String>,
}
impl RoomNameEventContent {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: Some(name.into()),
		}
	}
}
impl crate::events::EventContent for RoomNameEventContent {
	type EventType = crate::events::StateEventType;
	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomName
	}
}

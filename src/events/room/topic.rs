#[derive(Debug, Default)]
pub struct RoomTopicEventContent {
	pub topic: String,
}
impl crate::events::EventContent for RoomTopicEventContent {
	type EventType = crate::events::StateEventType;
	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomTopic
	}
}

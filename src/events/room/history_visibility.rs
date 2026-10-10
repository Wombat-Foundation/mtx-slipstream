#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum HistoryVisibility {
	#[default]
	Shared,
	Invited,
	Joined,
	WorldReadable,
	Custom(String),
}
impl crate::codec::Serialize for HistoryVisibility {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(match self {
			Self::Shared => "shared".into(),
			Self::Invited => "invited".into(),
			Self::Joined => "joined".into(),
			Self::WorldReadable => "world_readable".into(),
			Self::Custom(v) => v.clone(),
		})
	}
}
impl crate::codec::Deserialize for HistoryVisibility {
	fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		let s =
			v.as_str().ok_or_else(|| crate::codec::DeError::expected("history visibility"))?;
		Ok(match s {
			"shared" => Self::Shared,
			"invited" => Self::Invited,
			"joined" => Self::Joined,
			"world_readable" => Self::WorldReadable,
			other => Self::Custom(other.into()),
		})
	}
}
#[derive(Debug, Default)]
pub struct RoomHistoryVisibilityEventContent {
	pub history_visibility: HistoryVisibility,
}
impl RoomHistoryVisibilityEventContent {
	#[must_use]
	pub fn new(history_visibility: HistoryVisibility) -> Self {
		Self {
			history_visibility,
		}
	}
}
impl crate::events::EventContent for RoomHistoryVisibilityEventContent {
	type EventType = crate::events::StateEventType;
	fn event_type(&self) -> Self::EventType {
		crate::events::StateEventType::RoomHistoryVisibility
	}
}

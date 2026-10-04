#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ruleset;
impl Ruleset {
	#[must_use]
	pub fn new() -> Self {
		Self
	}
	pub fn server_default<T>(_user: &T) -> Self {
		Self
	}
}

impl crate::codec::Serialize for Ruleset {
	fn to_json(&self) -> crate::json::Value { crate::json::Value::Object(crate::json::Object::new()) }
}

impl crate::codec::Deserialize for Ruleset {
	fn from_json(_: &crate::json::Value) -> Result<Self, crate::codec::DeError> { Ok(Self) }
}
#[derive(Clone, Debug, Default)]
pub struct Action;
#[derive(Clone, Debug, Default)]
pub struct Tweak;
#[derive(Clone, Debug, Default)]
pub struct PushConditionPowerLevelsCtx;
#[derive(Clone, Debug, Default)]
pub struct PushConditionRoomCtx;
#[derive(Clone, Debug, Default)]
pub struct PushFormat;

#[derive(Clone, Debug, Default)]
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

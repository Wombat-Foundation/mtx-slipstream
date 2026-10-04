//! `m.direct`.

use alloc::{collections::BTreeMap, vec::Vec};

use super::wrappers::GlobalAccountDataEvent;
use crate::{
	OwnedRoomId, OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

/// The direct-message rooms for each user.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DirectEventContent(pub BTreeMap<OwnedUserId, Vec<OwnedRoomId>>);

impl Serialize for DirectEventContent {
	fn to_json(&self) -> Value {
		self.0.to_json()
	}
}

impl Deserialize for DirectEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self(BTreeMap::from_json(value)?))
	}
}

pub type DirectEvent = GlobalAccountDataEvent<DirectEventContent>;

//! Per-user records the server persists.

use alloc::string::String;

use crate::{
	OwnedUserId,
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

/// A user's suspension state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserSuspension {
	/// Whether the user is currently suspended.
	pub suspended: bool,
	/// When the user was suspended (Unix timestamp in milliseconds).
	pub suspended_at: u64,
	/// User ID of who suspended this user.
	pub suspended_by: String,
}

crate::impl_codec_struct!(UserSuspension {
	suspended: bool,
	suspended_at: u64,
	suspended_by: String,
});

/// A profile change retained for MSC4429 incremental sync.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileUpdate {
	pub user_id: OwnedUserId,
	pub field: String,
	/// The new value, or `None` when the field was removed.
	pub value: Option<Value>,
}

impl Serialize for ProfileUpdate {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("user_id".into(), self.user_id.to_json());
		object.insert("field".into(), Value::String(self.field.clone()));
		object.insert("value".into(), self.value.clone().unwrap_or(Value::Null));
		Value::Object(object)
	}
}

impl Deserialize for ProfileUpdate {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("profile update"))?;
		Ok(Self {
			user_id: object
				.get("user_id")
				.map(OwnedUserId::from_json)
				.transpose()?
				.ok_or_else(|| DeError::expected("user_id"))?,
			field: object
				.get("field")
				.map(String::from_json)
				.transpose()?
				.ok_or_else(|| DeError::expected("field"))?,
			value: object.get("value").filter(|v| !v.is_null()).cloned(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn profile_update_nulls_removed_values() {
		let update = ProfileUpdate {
			user_id: OwnedUserId::from("@a:x"),
			field: "displayname".into(),
			value: None,
		};
		assert_eq!(
			to_string(&update),
			r#"{"field":"displayname","user_id":"@a:x","value":null}"#
		);
		assert_eq!(from_str::<ProfileUpdate>(&to_string(&update)).unwrap(), update);
	}

	#[test]
	fn suspension_round_trips() {
		let suspension = UserSuspension {
			suspended: true,
			suspended_at: 5,
			suspended_by: "@admin:x".into(),
		};
		assert_eq!(from_str::<UserSuspension>(&to_string(&suspension)).unwrap(), suspension);
	}
}

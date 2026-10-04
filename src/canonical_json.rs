//! Canonical JSON compatibility primitives.

use alloc::string::String;
use core::fmt;

pub use crate::json;

pub type Object = json::Object;
pub type Value = json::Value;
pub type Array = alloc::vec::Vec<Value>;
/// Drop-in for `serde_json::Map<String, Value>`, backed by Rezzy.
pub type JsonMap = Object;

/// Error returned when redaction fails.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RedactionError {
	/// The room version has no known redaction rules.
	UnsupportedRoomVersion(String),
}

impl fmt::Display for RedactionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::UnsupportedRoomVersion(v) => write!(f, "unsupported room version: {v}"),
		}
	}
}
impl core::error::Error for RedactionError {}

/// Converts a JSON map into a canonical JSON object.
///
/// # Errors
///
/// Currently infallible; the `Result` mirrors ruma's signature.
pub fn try_from_json_map(map: JsonMap) -> Result<Object, crate::CanonicalJsonError> {
	Ok(map)
}

/// Redacts `content` in place according to the room version's rules.
///
/// # Errors
///
/// Returns an error if the room version is not one Rezzy has rules for.
pub fn redact_content_in_place(
	content: &mut Object,
	version: &crate::RoomVersionId,
	event_type: String,
) -> Result<(), RedactionError> {
	if matches!(version, crate::RoomVersionId::Custom(_)) {
		return Err(RedactionError::UnsupportedRoomVersion(version.as_str().into()));
	}
	let (redacted, _) = rezzy::split_redaction_content(
		&Value::Object(core::mem::take(content)),
		&event_type,
		version.as_str(),
	);
	if let Value::Object(object) = redacted {
		*content = object;
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::RoomVersionId;

	#[test]
	fn redacts_member_content_to_membership() {
		let Value::Object(mut content) =
			Value::parse(r#"{"membership":"join","displayname":"Bob"}"#).unwrap()
		else {
			panic!("not an object");
		};
		redact_content_in_place(&mut content, &RoomVersionId::V11, "m.room.member".into())
			.unwrap();
		assert!(content.contains_key("membership"));
		assert!(!content.contains_key("displayname"));
	}

	#[test]
	fn rejects_custom_room_version() {
		let mut content = Object::new();
		assert!(
			redact_content_in_place(
				&mut content,
				&RoomVersionId::Custom("x".into()),
				"m.x".into()
			)
			.is_err()
		);
	}
}

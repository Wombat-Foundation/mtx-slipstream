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
#[derive(Debug, Eq, PartialEq)]
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

/// Serializes `value` into a canonical JSON value.
///
/// # Errors
///
/// Currently infallible; the `Result` mirrors ruma's signature. Unlike ruma's, this does
/// not reject floats or integers outside the canonical `[-2^53 + 1, 2^53 - 1]` range.
pub fn to_canonical_value<T: crate::codec::Serialize>(
	value: T,
) -> Result<Value, crate::CanonicalJsonError> {
	Ok(crate::codec::to_value(&value))
}

/// Redacts `content` in place according to the room version's rules.
///
/// # Errors
///
/// Returns an error if the room version is not one Rezzy has rules for.
pub fn redact_content_in_place(
	content: &mut Object,
	version: &crate::RoomVersionId,
	event_type: impl AsRef<str>,
) -> Result<(), RedactionError> {
	// MSC3389's unstable room version redacts like version 10 but keeps
	// `rel_type` and `event_id` of `m.relates_to`.
	let msc3389 = version.as_str() == MSC3389_ROOM_VERSION;
	if matches!(version, crate::RoomVersionId::Custom(_)) && !msc3389 {
		return Err(RedactionError::UnsupportedRoomVersion(version.as_str().into()));
	}
	let preserved_relation = if msc3389 {
		content.get("m.relates_to").and_then(Value::as_object).map(|relation| {
			["rel_type", "event_id"]
				.into_iter()
				.filter_map(|key| {
					relation
						.get(key)
						.map(|value| (alloc::string::String::from(key), value.clone()))
				})
				.collect::<Object>()
		})
	} else {
		None
	};
	// TODO: better logic here; it could be v11, v12, etc
	let rules_version = if msc3389 {
		"10"
	} else {
		version.as_str()
	};
	let (redacted, _) = rezzy::split_redaction_content(
		&Value::Object(content.clone()),
		event_type.as_ref(),
		rules_version,
	);
	if let Value::Object(object) = redacted {
		*content = object;
	}
	if let Some(relation) = preserved_relation.filter(|relation| !relation.is_empty()) {
		content.insert("m.relates_to".into(), Value::Object(relation));
	}
	Ok(())
}

/// The unstable room version that keeps `m.relates_to` through redaction (MSC3389).
const MSC3389_ROOM_VERSION: &str = "org.matrix.msc3389.10";

/// Redacts a full event object, using its `type` field when no type is supplied.
///
/// # Errors
///
/// Returns an error if the room version has no known redaction rules.
pub fn redact(
	mut object: Object,
	version: &crate::RoomVersionId,
	event_type: Option<&str>,
) -> Result<Object, RedactionError> {
	let kind = event_type
		.map(str::to_owned)
		.or_else(|| object.get("type").and_then(Value::as_str).map(str::to_owned))
		.unwrap_or_default();
	if let Some(Value::Object(content)) = object.get_mut("content") {
		redact_content_in_place(content, version, kind)?;
	}
	Ok(object)
}

/// Redacts a full event object in place.
///
/// # Errors
///
/// Returns an error if the room version has no known redaction rules.
pub fn redact_in_place(
	object: &mut Object,
	version: &crate::RoomVersionId,
	event_type: Option<&str>,
) -> Result<(), RedactionError> {
	let redacted = redact(core::mem::take(object), version, event_type)?;
	*object = redacted;
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
		redact_content_in_place(&mut content, &RoomVersionId::V11, "m.room.member").unwrap();
		assert!(content.contains_key("membership"));
		assert!(!content.contains_key("displayname"));
	}

	#[test]
	fn rejects_custom_room_version() {
		let mut content = Object::new();
		assert!(
			redact_content_in_place(&mut content, &RoomVersionId::Custom("x".into()), "m.x")
				.is_err()
		);
	}

	fn relation_content() -> Object {
		let Value::Object(object) = Value::parse(
			r#"{"body":"x","m.relates_to":{"rel_type":"m.annotation","event_id":"$e","key":"k"}}"#,
		)
		.unwrap() else {
			panic!("object")
		};
		object
	}

	#[test]
	fn msc3389_room_version_keeps_rel_type_and_event_id() {
		let version = RoomVersionId::Custom("org.matrix.msc3389.10".into());
		let mut content = relation_content();
		redact_content_in_place(&mut content, &version, "m.reaction").unwrap();
		let relation = content.get("m.relates_to").and_then(Value::as_object).unwrap();
		assert_eq!(relation.len(), 2, "only rel_type and event_id survive: {relation:?}");
		assert!(relation.contains_key("rel_type") && relation.contains_key("event_id"));
		assert!(!content.contains_key("body"));
	}

	#[test]
	fn older_room_versions_strip_the_relation_entirely() {
		let mut content = relation_content();
		redact_content_in_place(&mut content, &RoomVersionId::V9, "m.reaction").unwrap();
		assert!(content.is_empty(), "{content:?}");
	}
}

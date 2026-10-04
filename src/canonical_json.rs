//! Canonical JSON compatibility primitives.

pub use crate::json;

pub type Object = json::Object;
pub type Value = json::Value;
pub type Array = alloc::vec::Vec<Value>;

pub type RedactionError = serde_json::Error;

pub fn try_from_json_map(map: serde_json::Map<alloc::string::String, serde_json::Value>) -> Result<Object, crate::CanonicalJsonError> {
	let _ = map;
	unimplemented!("canonical JSON conversion is being implemented over rezzy-json")
}

pub fn redact_content_in_place(content: &mut Object, _: &crate::RoomVersionId, _: alloc::string::String) -> Result<(), RedactionError> {
	for key in ["auth_events", "prev_events", "depth", "hashes", "signatures"] {
		content.remove(key);
	}
	Ok(())
}

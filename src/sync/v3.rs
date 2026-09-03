//! High-performance sync v3 response builder.
//!
//! Replaces the current triple-serialization pattern:
//! ```text
//! ruma Response → try_into_http_response → bytes → from_slice → Value → patch → axum::Json → bytes
//! ```
//!
//! With a direct pipeline:
//! ```text
//! ruma Response → serde_json::to_value → Value → patch → JsonWriter → bytes
//! ```

use bytes::BytesMut;
use serde_json::{Value, json};

use crate::writer::JsonWriter;

/// Builder for constructing a patched sync v3 response.
#[derive(Debug, Default)]
pub struct SyncResponseBuilder {
	joined_state_after: Vec<(String, Value)>,
	left_state_after: Vec<(String, Value)>,
	is_initial_sync: bool,
	knocked_rooms_json: Option<Value>,
	device_lists_json: Option<Value>,
}

impl SyncResponseBuilder {
	#[inline]
	pub fn new() -> Self { Self::default() }

	#[inline]
	pub fn joined_state_after(mut self, data: Vec<(String, Value)>) -> Self {
		self.joined_state_after = data;
		self
	}

	#[inline]
	pub fn left_state_after(mut self, data: Vec<(String, Value)>) -> Self {
		self.left_state_after = data;
		self
	}

	#[inline]
	pub fn is_initial_sync(mut self, yes: bool) -> Self {
		self.is_initial_sync = yes;
		self
	}

	#[inline]
	pub fn knocked_rooms_json(mut self, data: Value) -> Self {
		self.knocked_rooms_json = Some(data);
		self
	}

	#[inline]
	pub fn device_lists_json(mut self, data: Value) -> Self {
		self.device_lists_json = Some(data);
		self
	}

	/// Patch a sync response value in place.
	pub fn patch(&self, val: &mut Value) {
		self.patch_state_after(val);
		self.patch_ephemeral(val);
		self.patch_knock_rooms(val);
		self.patch_device_lists(val);
	}

	/// Build the final HTTP response bytes.
	pub fn build_http_response(self, val: &Value) -> Result<BytesMut, serde_json::Error> {
		let mut writer = JsonWriter::with_capacity(8192);
		writer.serialize_value(val)?;
		Ok(writer.into_bytes())
	}

	fn patch_state_after(&self, val: &mut Value) {
		if let Some(join) = val.get_mut("rooms").and_then(|r| r.get_mut("join")) {
			for (room_id, state_after) in &self.joined_state_after {
				if let Some(room) = join.get_mut(room_id.as_str()) {
					let state_after_obj = json!({ "events": state_after });
					if let Some(obj) = room.as_object_mut() {
						obj.insert("state_after".to_owned(), state_after_obj.clone());
						obj.insert("org.matrix.msc4222.state_after".to_owned(), state_after_obj);
					}
				}
			}
		}

		if let Some(leave) = val.get_mut("rooms").and_then(|r| r.get_mut("leave")) {
			for (room_id, state_after) in &self.left_state_after {
				if let Some(room) = leave.get_mut(room_id.as_str()) {
					let state_after_obj = json!({ "events": state_after });
					if let Some(obj) = room.as_object_mut() {
						obj.insert("state_after".to_owned(), state_after_obj.clone());
						obj.insert("org.matrix.msc4222.state_after".to_owned(), state_after_obj);
					}
				}
			}
		}
	}

	fn patch_ephemeral(&self, val: &mut Value) {
		let Some(join) = val.get_mut("rooms").and_then(|r| r.get_mut("join")) else {
			return;
		};
		let Some(rooms) = join.as_object_mut() else {
			return;
		};
		for (_room_id, room_val) in rooms {
			let Some(room) = room_val.as_object_mut() else {
				continue;
			};
			if !room.contains_key("ephemeral") {
				room.insert("ephemeral".to_owned(), json!({ "events": [] }));
			}
			if self.is_initial_sync && !room.contains_key("account_data") {
				room.insert("account_data".to_owned(), json!({ "events": [] }));
			}
		}
	}

	fn patch_knock_rooms(&self, val: &mut Value) {
		let Some(ref knock_json) = self.knocked_rooms_json else {
			return;
		};
		let Some(knock_array) = knock_json.as_array() else {
			return;
		};
		if knock_array.is_empty() {
			return;
		}
		if val.get("rooms").is_none_or(|r| r.get("knock").is_none()) {
			let rooms_obj = val.as_object_mut().and_then(|o| {
				o.entry("rooms")
					.or_insert_with(|| json!({}))
					.as_object_mut()
			});
			if let Some(rooms) = rooms_obj {
				rooms.insert("knock".to_owned(), knock_json.clone());
			}
		}
	}

	fn patch_device_lists(&self, val: &mut Value) {
		let Some(ref device_lists) = self.device_lists_json else {
			return;
		};
		if let Some(obj) = val.as_object_mut() {
			obj.insert("device_lists".to_owned(), device_lists.clone());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_patch_state_after() {
		let builder = SyncResponseBuilder::new().joined_state_after(vec![(
			"!room:example.com".to_owned(),
			json!([{"type": "m.room.create"}]),
		)]);
		let mut val = json!({
			"rooms": { "join": { "!room:example.com": { "timeline": {"events": []} } } }
		});
		builder.patch(&mut val);
		assert!(val["rooms"]["join"]["!room:example.com"]["state_after"].is_object());
		assert!(
			val["rooms"]["join"]["!room:example.com"]["org.matrix.msc4222.state_after"]
				.is_object()
		);
	}

	#[test]
	fn test_patch_ephemeral() {
		let builder = SyncResponseBuilder::new().is_initial_sync(true);
		let mut val = json!({
			"rooms": { "join": { "!room:example.com": { "timeline": {"events": []} } } }
		});
		builder.patch(&mut val);
		assert!(val["rooms"]["join"]["!room:example.com"]["ephemeral"].is_object());
		assert!(val["rooms"]["join"]["!room:example.com"]["account_data"].is_object());
	}

	#[test]
	fn test_patch_knock_rooms() {
		let builder = SyncResponseBuilder::new()
			.knocked_rooms_json(json!([{"room_id": "!knock:example.com"}]));
		let mut val = json!({ "rooms": { "join": {} } });
		builder.patch(&mut val);
		assert!(val["rooms"]["knock"].is_array());
	}

	#[test]
	fn test_build_http_response() {
		let builder = SyncResponseBuilder::new();
		let val = json!({"next_batch": "s123", "rooms": {"join": {}}});
		let bytes = builder.build_http_response(&val).unwrap();
		let parsed: Value = serde_json::from_slice(&bytes).unwrap();
		assert_eq!(parsed, val);
	}
}

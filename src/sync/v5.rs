//! High-performance sync v5 (sliding sync) response builder.

use bytes::BytesMut;
use simd_json::{OwnedValue, prelude::*};

use crate::writer::BufWriter;

/// Per-room extra data for v5 sliding sync responses.
#[derive(Debug, Clone)]
pub struct RoomExtras {
	/// The membership state of the local user in this room.
	pub membership: Option<String>,
	/// List identifiers the room appears in.
	pub lists: Vec<String>,
	/// Whether to include the full expanded timeline.
	pub expanded_timeline: bool,
}

/// Builder for constructing a patched sync v5 response.
#[derive(Debug, Default)]
pub struct SlidingSyncResponseBuilder {
	thread_subscriptions: Option<OwnedValue>,
	room_extras: Vec<(String, RoomExtras)>,
}

impl SlidingSyncResponseBuilder {
	/// Create a new empty builder.
	#[inline]
	#[must_use]
	pub fn new() -> Self { Self::default() }

	/// Set the `io.element.msc4308.thread_subscriptions` extension value.
	#[inline]
	#[must_use]
	pub fn thread_subscriptions(mut self, data: OwnedValue) -> Self {
		self.thread_subscriptions = Some(data);
		self
	}

	/// Add a single room's extra data.
	#[inline]
	#[must_use]
	pub fn room_extra(mut self, room_id: String, extras: RoomExtras) -> Self {
		self.room_extras.push((room_id, extras));
		self
	}

	/// Replace all room extras at once.
	#[inline]
	#[must_use]
	pub fn room_extras(mut self, extras: Vec<(String, RoomExtras)>) -> Self {
		self.room_extras = extras;
		self
	}

	/// Patch a sync response value in place.
	///
	/// Applies thread subscriptions and per-room extra fields (membership,
	/// lists, timeline, `stripped_state`).
	pub fn patch(&self, val: &mut OwnedValue) {
		self.patch_thread_subscriptions(val);
		self.patch_rooms(val);
	}

	/// Serialize the patched value into a complete HTTP response body.
	///
	/// # Errors
	///
	/// Returns `simd_json::Error` if serialization fails.
	pub fn build_http_response(self, val: &OwnedValue) -> Result<BytesMut, simd_json::Error> {
		let mut buf = BytesMut::with_capacity(8192);
		let mut writer = BufWriter(&mut buf);
		simd_json::to_writer(&mut writer, val)?;
		Ok(buf)
	}

	fn patch_thread_subscriptions(&self, val: &mut OwnedValue) {
		let Some(ref subs) = self.thread_subscriptions else {
			return;
		};
		val.as_object_mut()
			.expect("sync response is a JSON object")
			.entry("extensions".to_owned())
			.or_insert_with(|| OwnedValue::from(simd_json::value::owned::Object::default()))
			.as_object_mut()
			.expect("sync response extensions is a JSON object")
			.insert("io.element.msc4308.thread_subscriptions".to_owned(), subs.clone());
	}

	fn patch_rooms(&self, val: &mut OwnedValue) {
		let Some(rooms) = val.get_mut("rooms").and_then(|v| v.as_object_mut()) else {
			return;
		};
		for (room_id, extra) in &self.room_extras {
			let Some(room) = rooms
				.get_mut(room_id.as_str())
				.and_then(|v| v.as_object_mut())
			else {
				continue;
			};
			if let Some(ref membership) = extra.membership {
				room.insert("membership".to_owned(), OwnedValue::from(membership.clone()));
			}
			if let Some(invite_state) = room.get("invite_state").cloned() {
				room.insert("stripped_state".to_owned(), invite_state);
			}
			if let Some(timeline) = room.get("timeline").cloned() {
				room.insert("timeline_events".to_owned(), timeline);
			}
			if let Ok(mut lists_vec) = simd_json::to_vec(&extra.lists)
				&& let Ok(lists_val) = simd_json::to_owned_value(&mut lists_vec)
			{
				room.insert("lists".to_owned(), lists_val);
			}
			if extra.expanded_timeline {
				room.insert("expanded_timeline".to_owned(), OwnedValue::from(true));
			}
		}
	}
}

#[cfg(test)]
#[coverage(off)]
mod tests {
	use simd_json::json;

	use super::*;

	#[test]
	fn test_patch_thread_subscriptions() {
		let builder = SlidingSyncResponseBuilder::new()
			.thread_subscriptions(json!({"!room:example.com": true}));
		let mut val = json!({"rooms": {}});
		builder.patch(&mut val);
		assert_eq!(
			val["extensions"]["io.element.msc4308.thread_subscriptions"]["!room:example.com"],
			true
		);
	}

	#[test]
	fn test_patch_room_extras() {
		let builder = SlidingSyncResponseBuilder::new().room_extra(
			"!room:example.com".to_owned(),
			RoomExtras {
				membership: Some("join".to_owned()),
				lists: vec!["list1".to_owned()],
				expanded_timeline: true,
			},
		);
		let mut val = json!({
			"rooms": {
				"!room:example.com": {
					"invite_state": {"events": []},
					"timeline": {"events": []}
				}
			}
		});
		builder.patch(&mut val);
		let room = &val["rooms"]["!room:example.com"];
		assert_eq!(room["membership"], "join");
		assert_eq!(room["stripped_state"]["events"], json!([]));
		assert_eq!(room["timeline_events"]["events"], json!([]));
		assert_eq!(room["lists"][0], "list1");
		assert_eq!(room["expanded_timeline"], true);
	}

	#[test]
	fn test_build_http_response() {
		let builder = SlidingSyncResponseBuilder::new();
		let val = json!({"rooms": {}});
		let bytes = builder.build_http_response(&val).unwrap();
		let mut input = bytes.to_vec();
		let parsed: OwnedValue = simd_json::from_slice(&mut input).unwrap();
		assert_eq!(parsed, val);
	}
}

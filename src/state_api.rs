//! Client room-state endpoints: list state, read one entry, send one entry.
//!
//! Wire details follow the pinned ruwuma definitions; only the `v3` paths are
//! declared here. `state_key` is the last path segment and may be empty, so
//! `/state/{type}` (no trailing segment) parses with an empty key.

use alloc::{string::String, vec::Vec};

use crate::{
	MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId,
	codec::{DeError, Serialize},
	endpoint::{EndpointRequest, EndpointResponse, Input, Metadata, query_pairs, to_param},
	events::{AnyStateEventContent, StateEventType},
	json::Value,
	serde::Raw,
};

pub mod get_state_events {
	pub mod v3 {
		use crate::{OwnedRoomId, events::AnyStateEvent, serde::Raw};

		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/rooms/{room_id}/state",
			request {
				path { room_id: OwnedRoomId }
				query {}
				body {}
			}
		}
		crate::endpoint_response_flat!(room_state: alloc::vec::Vec<Raw<AnyStateEvent>>);

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}
	}
}

pub mod get_state_events_for_key {
	pub mod v3 {
		use super::super::{
			DeError, EndpointRequest, EndpointResponse, Input, Metadata, OwnedRoomId, Serialize,
			StateEventType, String, Value, Vec, path_args, query_pairs,
		};

		const PATH: &str = "/_matrix/client/v3/rooms/{room_id}/state/{event_type}/{state_key}";
		const _: Metadata = Metadata::new("GET", PATH);

		/// Reads one state entry; `format=event` asks for the whole event.
		#[derive(Clone, Debug)]
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_type: StateEventType,
			pub state_key: String,
			pub format: Option<String>,
		}

		impl Request {
			#[must_use]
			pub fn new(
				room_id: OwnedRoomId,
				event_type: StateEventType,
				state_key: String,
			) -> Self {
				Self {
					room_id,
					event_type,
					state_key,
					format: None,
				}
			}
		}

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata::new("GET", PATH);

			fn path_args(&self) -> Vec<String> {
				path_args(&self.room_id, &self.event_type, &self.state_key)
			}

			fn query(&self) -> Vec<(String, String)> {
				query_pairs(alloc::vec![("format", self.format.to_json())])
			}

			fn body(&self) -> Option<Value> {
				None
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					event_type: input.path()?,
					state_key: input.path::<Option<String>>()?.unwrap_or_default(),
					format: input.query("format")?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		/// Either the content (the default) or the whole event, both sent
		/// as the entire body.
		#[derive(Clone, Debug, Default)]
		pub struct Response {
			pub content: Option<Value>,
			pub event: Option<Value>,
		}

		impl Response {
			#[must_use]
			pub fn new(content: Value, event: Value) -> Self {
				Self {
					content: Some(content),
					event: Some(event),
				}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				let mut object = crate::json::Object::new();
				for part in [&self.content, &self.event].into_iter().flatten() {
					if let Value::Object(fields) = part {
						object.extend(fields.iter().map(|(k, v)| (k.clone(), v.clone())));
					}
				}
				Value::Object(object)
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				Ok(Self {
					content: Some(body.clone()),
					event: None,
				})
			}
		}
	}
}

pub mod send_state_event {
	pub mod v3 {
		use super::super::{
			AnyStateEventContent, DeError, EndpointRequest, Input, Metadata,
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, Raw, Serialize,
			StateEventType, String, Value, Vec, path_args, query_pairs,
		};

		const PATH: &str = "/_matrix/client/v3/rooms/{room_id}/state/{event_type}/{state_key}";
		const _: Metadata = Metadata::new("PUT", PATH);

		/// Sends one state entry; the body is the event content as given.
		#[derive(Clone, Debug)]
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_type: StateEventType,
			pub state_key: String,
			pub body: Raw<AnyStateEventContent>,
			pub timestamp: Option<MilliSecondsSinceUnixEpoch>,
		}

		impl Request {
			#[must_use]
			pub fn new_raw(
				room_id: OwnedRoomId,
				event_type: StateEventType,
				state_key: String,
				body: Raw<AnyStateEventContent>,
			) -> Self {
				Self {
					room_id,
					event_type,
					state_key,
					body,
					timestamp: None,
				}
			}
		}

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata::new("PUT", PATH);

			fn path_args(&self) -> Vec<String> {
				path_args(&self.room_id, &self.event_type, &self.state_key)
			}

			fn query(&self) -> Vec<(String, String)> {
				query_pairs(alloc::vec![("timestamp", self.timestamp.to_json())])
			}

			fn body(&self) -> Option<Value> {
				Some(self.body.to_json())
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					event_type: input.path()?,
					state_key: input.path::<Option<String>>()?.unwrap_or_default(),
					timestamp: input.query("timestamp")?,
					body: crate::codec::Deserialize::from_json(
						body.ok_or_else(|| DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		crate::endpoint_response! { response { event_id: OwnedEventId } }

		impl Response {
			#[must_use]
			pub fn new(event_id: OwnedEventId) -> Self {
				Self {
					event_id,
				}
			}
		}
	}
}

fn path_args(room_id: &OwnedRoomId, event_type: &StateEventType, state_key: &str) -> Vec<String> {
	alloc::vec![
		to_param(room_id).unwrap_or_default(),
		to_param(event_type).unwrap_or_default(),
		state_key.into(),
	]
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::json;

	fn room() -> OwnedRoomId {
		OwnedRoomId::from("!r:example.org")
	}

	#[test]
	fn send_round_trips_and_keeps_content_text() {
		let request = send_state_event::v3::Request::new_raw(
			room(),
			StateEventType::RoomName,
			String::new(),
			Raw::from_json_text(r#"{"name":"x"}"#).unwrap(),
		);
		assert_eq!(request.path_args(), ["!r:example.org", "m.room.name", ""]);
		let body = request.body().unwrap();
		assert_eq!(body, json!({"name": "x"}));
		let back = send_state_event::v3::Request::from_parts(
			&request.path_args(),
			&request.query(),
			Some(&body),
		)
		.unwrap();
		assert_eq!(back.state_key, "");
		assert_eq!(back.event_type, StateEventType::RoomName);
	}

	#[test]
	fn missing_trailing_state_key_is_empty() {
		let path = ["!r:example.org".to_owned(), "m.room.topic".to_owned()];
		let read = get_state_events_for_key::v3::Request::from_parts(&path, &[], None).unwrap();
		assert_eq!(read.state_key, "");
		assert_eq!(read.format, None);
		let write =
			send_state_event::v3::Request::from_parts(&path, &[], Some(&json!({}))).unwrap();
		assert_eq!(write.state_key, "");
	}

	#[test]
	fn timestamp_and_format_travel_in_the_query() {
		let mut request = get_state_events_for_key::v3::Request::new(
			room(),
			StateEventType::RoomName,
			"k".into(),
		);
		request.format = Some("event".into());
		assert_eq!(request.query(), [("format".to_owned(), "event".to_owned())]);
	}

	#[test]
	fn response_is_content_or_event_as_the_whole_body() {
		let content = get_state_events_for_key::v3::Response {
			content: Some(json!({"name": "x"})),
			event: None,
		};
		assert_eq!(content.to_body(), json!({"name": "x"}));
		let event = get_state_events_for_key::v3::Response {
			content: None,
			event: Some(json!({"type": "m.room.name", "content": {"name": "x"}})),
		};
		assert_eq!(event.to_body(), json!({"type": "m.room.name", "content": {"name": "x"}}));
		assert_eq!(get_state_events_for_key::v3::Response::default().to_body(), json!({}));
	}

	#[test]
	fn state_list_body_is_a_bare_array() {
		let response = get_state_events::v3::Response {
			room_state: alloc::vec![Raw::from_json_text(r#"{"type":"m.room.name"}"#).unwrap()],
		};
		assert!(matches!(response.to_body(), Value::Array(items) if items.len() == 1));
	}
}

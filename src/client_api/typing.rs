pub mod create_typing_event {
	pub mod v3 {
		use crate::{
			OwnedRoomId, OwnedUserId,
			codec::{DeError, Deserialize, Serialize},
			endpoint,
			json::Value,
		};

		/// Whether a user is typing, and for how long.
		///
		/// The wire form is an object, not a bare boolean:
		/// `{"typing": true, "timeout": 30000}`. `Typing::Yes` always carries a
		/// timeout so that "typing with no timeout" is unrepresentable.
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum Typing {
			/// Typing for the given duration.
			Yes(std::time::Duration),
			/// Not typing.
			No,
		}

		impl Serialize for Typing {
			fn to_json(&self) -> Value {
				let (typing, timeout) = match self {
					Self::Yes(timeout) => (true, Some(*timeout)),
					Self::No => (false, None),
				};

				// `object_from` drops null values, so `Typing::No` omits the timeout
				// entirely instead of sending an explicit null.
				Value::Object(endpoint::object_from(alloc::vec![
					("typing", Value::Bool(typing)),
					("timeout", timeout.to_json()),
				]))
			}
		}

		impl Deserialize for Typing {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				let object =
					value.as_object().ok_or_else(|| DeError::expected("typing object"))?;
				let typing = object
					.get("typing")
					.and_then(Value::as_bool)
					.ok_or_else(|| DeError::expected("typing boolean"))?;

				if !typing {
					return Ok(Self::No);
				}

				let timeout =
					object.get("timeout").ok_or_else(|| DeError::expected("typing timeout"))?;

				Ok(Self::Yes(std::time::Duration::from_json(timeout)?))
			}
		}

		#[derive(Debug)]
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub user_id: OwnedUserId,
			pub state: Typing,
		}

		impl endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata = endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/v3/rooms/{roomId}/typing/{userId}",
			);

			fn path_args(&self) -> Vec<String> {
				Vec::new()
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				Some(self.state.to_json())
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					user_id: input.path()?,
					state: Typing::from_json(
						body.ok_or_else(|| DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;

				Ok(value)
			}
		}

		impl Request {
			/// Creates a request for the given room, user and typing state.
			#[must_use]
			pub fn new(room_id: OwnedRoomId, user_id: OwnedUserId, state: Typing) -> Self {
				Self {
					room_id,
					user_id,
					state,
				}
			}
		}

		#[derive(Debug, Default)]
		pub struct Response {}

		impl endpoint::EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(crate::json::Object::new())
			}

			fn from_body(_body: &Value) -> Result<Self, DeError> {
				Ok(Self {})
			}
		}

		impl Response {
			/// Creates an empty response.
			#[must_use]
			pub fn new() -> Self {
				Self {}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::create_typing_event::v3::Typing;
	use crate::codec::{Deserialize, Serialize};
	use crate::json::Value;

	#[test]
	fn yes_serializes_as_object_with_timeout() {
		let value = Typing::Yes(std::time::Duration::from_millis(30_000)).to_json();

		assert_eq!(
			value.get("typing").and_then(Value::as_bool),
			Some(true),
			"typing should be a boolean, not a bare value"
		);
		assert_eq!(value.get("timeout").and_then(Value::as_u64), Some(30_000));
	}

	#[test]
	fn no_serializes_as_object_without_timeout() {
		let value = Typing::No.to_json();

		assert_eq!(value.get("typing").and_then(Value::as_bool), Some(false));
		assert!(value.get("timeout").is_none(), "timeout should be omitted");
	}

	#[test]
	fn round_trips_yes_with_timeout() {
		let original = Typing::Yes(std::time::Duration::from_millis(1_500));
		let decoded = Typing::from_json(&original.to_json()).expect("valid typing");

		assert_eq!(decoded, original);
	}

	#[test]
	fn round_trips_no() {
		let decoded = Typing::from_json(&Typing::No.to_json()).expect("valid typing");

		assert_eq!(decoded, Typing::No);
	}

	#[test]
	fn typing_true_without_timeout_is_rejected() {
		let value = crate::json!({ "typing": true });

		assert!(Typing::from_json(&value).is_err(), "typing without timeout should be rejected");
	}

	#[test]
	fn typing_false_with_timeout_reads_as_no() {
		let value = crate::json!({ "typing": false, "timeout": 30_000u64 });

		assert_eq!(Typing::from_json(&value).expect("valid typing"), Typing::No);
	}

	#[test]
	fn bare_boolean_is_rejected() {
		let value = crate::json!({});

		assert!(
			Typing::from_json(&value).is_err(),
			"a bare object without the typing field should be rejected"
		);
	}
}

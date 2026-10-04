//! Codec impls and helpers for event-content and wire types.

use alloc::{string::String, vec::Vec};

use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use bytes::BufMut;

use crate::{
	MilliSecondsSinceUnixEpoch,
	api::client::{
		discovery::{discover_homeserver::RtcFocusInfo, discover_support::ContactRole},
		error::{Error, ErrorBody, ErrorKind},
		uiaa::UiaaResponse,
	},
	api::error::IntoHttpError,
	codec::{DeError, Deserialize, Serialize, from_value},
	events::room::create::RoomCreateEventContent,
	json::Value,
	serde::{Base64, Raw},
};

impl Serialize for MilliSecondsSinceUnixEpoch {
	fn to_json(&self) -> Value {
		self.0.to_json()
	}
}
impl Deserialize for MilliSecondsSinceUnixEpoch {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self(from_value(value)?))
	}
}

macro_rules! transparent_value {
	($($t:ty),*) => {$(
		impl Serialize for $t {
			fn to_json(&self) -> Value { self.0.clone() }
		}
		impl Deserialize for $t {
			fn from_json(value: &Value) -> Result<Self, DeError> { Ok(Self(value.clone())) }
		}
	)*};
}
transparent_value!(RtcFocusInfo, ContactRole);

impl Serialize for RoomCreateEventContent {
	fn to_json(&self) -> Value {
		let mut object = crate::json::Object::new();
		if let Some(creator) = &self.creator {
			object.insert("creator".into(), creator.to_json());
		}
		object.insert("room_version".into(), self.room_version.to_json());
		if let Some(creators) = &self.additional_creators {
			object.insert("additional_creators".into(), creators.to_json());
		}
		Value::Object(object)
	}
}

/// Error decoding unpadded base64.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Base64DecodeError;

impl core::fmt::Display for Base64DecodeError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str("invalid base64")
	}
}
impl core::error::Error for Base64DecodeError {}

impl Base64 {
	#[must_use]
	pub fn new(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}

	/// Parses standard-alphabet base64, with or without padding.
	///
	/// # Errors
	///
	/// Returns an error if the input is not valid base64.
	pub fn parse(input: impl AsRef<[u8]>) -> Result<Self, Base64DecodeError> {
		let trimmed: Vec<u8> = input.as_ref().iter().copied().filter(|&b| b != b'=').collect();
		STANDARD_NO_PAD.decode(trimmed).map(Self).map_err(|_| Base64DecodeError)
	}

	#[must_use]
	pub fn encode(&self) -> String {
		STANDARD_NO_PAD.encode(&self.0)
	}

	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

impl Serialize for Base64 {
	fn to_json(&self) -> Value {
		Value::String(self.encode())
	}
}
impl Deserialize for Base64 {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let s = value.as_str().ok_or_else(|| DeError::expected("base64 string"))?;
		Self::parse(s).map_err(|e| DeError(alloc::string::ToString::to_string(&e)))
	}
}

impl ErrorKind {
	/// The Matrix `errcode` for this kind.
	#[must_use]
	pub fn errcode(&self) -> &'static str {
		match self {
			Self::LimitExceeded {
				..
			} => "M_LIMIT_EXCEEDED",
			Self::Forbidden {
				..
			} => "M_FORBIDDEN",
			Self::UnknownToken {
				..
			} => "M_UNKNOWN_TOKEN",
			Self::MissingToken => "M_MISSING_TOKEN",
			Self::NotFound => "M_NOT_FOUND",
			Self::BadJson => "M_BAD_JSON",
			Self::InvalidParam => "M_INVALID_PARAM",
			Self::TooLarge => "M_TOO_LARGE",
			Self::Unrecognized => "M_UNRECOGNIZED",
			Self::Unauthorized => "M_UNAUTHORIZED",
			Self::UserDeactivated => "M_USER_DEACTIVATED",
			Self::UserLocked => "M_USER_LOCKED",
			Self::UserSuspended => "M_USER_SUSPENDED",
			Self::GuestAccessForbidden => "M_GUEST_ACCESS_FORBIDDEN",
			_ => "M_UNKNOWN",
		}
	}
}

impl ErrorKind {
	/// The `M_FORBIDDEN` kind.
	#[must_use]
	pub fn forbidden() -> Self {
		Self::Forbidden {
			_value: (),
		}
	}

	/// The kind for a Matrix `errcode`; unknown codes map to `Unknown`.
	#[must_use]
	pub fn from_errcode(errcode: &str) -> Self {
		match errcode {
			"M_LIMIT_EXCEEDED" => Self::LimitExceeded {
				retry_after_ms: None,
			},
			"M_FORBIDDEN" => Self::Forbidden {
				_value: (),
			},
			"M_UNKNOWN_TOKEN" => Self::UnknownToken {
				soft_logout: false,
			},
			"M_MISSING_TOKEN" => Self::MissingToken,
			"M_NOT_FOUND" => Self::NotFound,
			"M_BAD_JSON" => Self::BadJson,
			"M_INVALID_PARAM" => Self::InvalidParam,
			"M_TOO_LARGE" => Self::TooLarge,
			"M_UNRECOGNIZED" => Self::Unrecognized,
			"M_UNAUTHORIZED" => Self::Unauthorized,
			"M_USER_DEACTIVATED" => Self::UserDeactivated,
			"M_USER_LOCKED" => Self::UserLocked,
			"M_USER_SUSPENDED" => Self::UserSuspended,
			"M_GUEST_ACCESS_FORBIDDEN" => Self::GuestAccessForbidden,
			_ => Self::Unknown,
		}
	}
}

impl Error {
	fn body_json(&self) -> Value {
		let mut object = crate::json::Object::new();
		if let ErrorBody::Standard {
			kind,
			message,
		} = &self.body
		{
			object.insert("errcode".into(), Value::String(kind.errcode().into()));
			object.insert("error".into(), Value::String(message.clone()));
			if let ErrorKind::LimitExceeded {
				retry_after_ms: Some(ms),
			} = kind
			{
				object.insert("retry_after_ms".into(), ms.to_json());
			}
		}
		Value::Object(object)
	}
}

impl UiaaResponse {
	/// Builds the HTTP response, writing the JSON body into `B`.
	///
	/// # Errors
	///
	/// Returns an error if the response cannot be built.
	pub fn try_into_http_response<B: Default + BufMut>(
		self,
	) -> Result<http::Response<B>, IntoHttpError> {
		let (status, body) = match self {
			Self::AuthResponse(info) => (http::StatusCode::UNAUTHORIZED, info.to_json()),
			Self::MatrixError(error) => (error.status_code, error.body_json()),
		};
		let json = crate::codec::to_string(&body);
		let mut buf = B::default();
		buf.put_slice(json.as_bytes());
		http::Response::builder()
			.status(status)
			.header(http::header::CONTENT_TYPE, "application/json")
			.body(buf)
			.map_err(|e| IntoHttpError(alloc::string::ToString::to_string(&e)))
	}
}

impl<T> Raw<T> {
	/// Parses the raw JSON value without deserializing it into `T`.
	pub fn json(&self) -> Result<crate::json::Value, DeError> {
		crate::codec::from_str(&self.0)
	}

	/// Serializes `value` into raw JSON.
	///
	/// # Errors
	///
	/// Currently infallible; the `Result` mirrors ruma's signature.
	pub fn new(value: &T) -> Result<Self, DeError>
	where
		T: Serialize,
	{
		Ok(Self(crate::codec::to_string(value), core::marker::PhantomData))
	}

	/// Parses the raw JSON into a `T`.
	///
	/// # Errors
	///
	/// Returns an error if the JSON does not match `T`.
	pub fn deserialize(&self) -> Result<T, DeError>
	where
		T: Deserialize,
	{
		crate::codec::from_str(&self.0)
	}
}

impl MilliSecondsSinceUnixEpoch {
	/// The timestamp for a system time, if it fits.
	#[must_use]
	pub fn from_system_time(time: std::time::SystemTime) -> Option<Self> {
		let millis = time.duration_since(std::time::UNIX_EPOCH).ok()?.as_millis();
		u64::try_from(millis).ok().map(Self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn base64_round_trips_with_and_without_padding() {
		let b = Base64::new(alloc::vec![1, 2, 3, 4]);
		assert_eq!(Base64::parse(b.encode()).unwrap(), b);
		assert_eq!(Base64::parse("AQIDBA==").unwrap(), b);
		assert!(Base64::parse("!!").is_err());
	}

	#[test]
	fn timestamp_and_create_content_round_trip() {
		let ts = MilliSecondsSinceUnixEpoch(1234);
		assert_eq!(from_str::<MilliSecondsSinceUnixEpoch>(&to_string(&ts)).unwrap(), ts);
		let c = from_str::<RoomCreateEventContent>(r#"{"room_version":"11","creator":"@a:b"}"#)
			.unwrap();
		assert_eq!(c.room_version, crate::RoomVersionId::V11);
		assert!(c.additional_creators.is_none());
	}

	#[test]
	fn matrix_error_response_has_errcode_body() {
		let error = Error {
			status_code: http::StatusCode::NOT_FOUND,
			body: ErrorBody::Standard {
				kind: ErrorKind::NotFound,
				message: "nope".into(),
			},
		};
		let response = UiaaResponse::MatrixError(error)
			.try_into_http_response::<alloc::vec::Vec<u8>>()
			.unwrap();
		assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
		let body = String::from_utf8(response.into_body()).unwrap();
		assert!(body.contains("M_NOT_FOUND") && body.contains("nope"));
	}
}

//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

pub mod canonical_json;
pub mod codec;
mod compat;
mod content;
mod event_type;
mod events_codec;
pub mod federation;
pub mod sync;
pub mod writer;

/// Canonical JSON substrate supplied by Rezzy.
pub use rezzy::json;

/// Primitive Matrix scalar compatibility types.
pub type Int = i64;
pub type UInt = u64;

use alloc::borrow::Borrow;
use core::{fmt, hash::Hash, ops::Deref};

/// Error returned when parsing a Matrix identifier fails.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MatrixIdParseError;

impl fmt::Display for MatrixIdParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("invalid Matrix identifier")
	}
}

impl core::error::Error for MatrixIdParseError {}

macro_rules! matrix_id {
	($borrowed:ident, $owned:ident) => {
		#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
		pub struct $owned(alloc::string::String);

		pub type $borrowed = $owned;

		impl $owned {
			/// Parses a Matrix identifier.
			///
			/// # Errors
			///
			/// This compatibility parser currently accepts every input and never
			/// returns an error.
			pub fn parse(value: &str) -> Result<Self, MatrixIdParseError> {
				Ok(Self(value.to_owned()))
			}
			pub fn as_str(&self) -> &str {
				&self.0
			}
			pub(crate) fn server_part(&self) -> Option<OwnedServerName> {
				self.as_str().rsplit_once(':').map(|(_, server)| OwnedServerName::from(server))
			}
		}

		impl From<alloc::string::String> for $owned {
			fn from(value: alloc::string::String) -> Self {
				Self(value)
			}
		}
		impl From<&str> for $owned {
			fn from(value: &str) -> Self {
				Self(value.to_owned())
			}
		}
		impl AsRef<$owned> for $owned {
			fn as_ref(&self) -> &$owned {
				self
			}
		}
		impl Borrow<str> for $owned {
			fn borrow(&self) -> &str {
				self.as_str()
			}
		}
		impl Deref for $owned {
			type Target = str;
			fn deref(&self) -> &str {
				self.as_str()
			}
		}
		impl fmt::Debug for $owned {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_tuple(stringify!($owned)).field(&self.0).finish()
			}
		}
		impl codec::Serialize for $owned {
			fn to_json(&self) -> json::Value {
				json::Value::String(self.0.clone())
			}
		}
		impl codec::Deserialize for $owned {
			fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
				value
					.as_str()
					.map(Self::from)
					.ok_or_else(|| codec::DeError::expected(stringify!($owned)))
			}
		}
		impl fmt::Display for $owned {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.write_str(self.as_str())
			}
		}
	};
}

matrix_id!(EventId, OwnedEventId);
matrix_id!(RoomId, OwnedRoomId);
matrix_id!(RoomAliasId, OwnedRoomAliasId);
matrix_id!(ServerName, OwnedServerName);
matrix_id!(UserId, OwnedUserId);
matrix_id!(RoomOrAliasId, OwnedRoomOrAliasId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MilliSecondsSinceUnixEpoch(pub UInt);

impl MilliSecondsSinceUnixEpoch {
	#[must_use]
	pub fn get(self) -> UInt {
		self.0
	}
	#[must_use]
	pub fn now() -> Self {
		let millis = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.map_or(0, |duration| UInt::try_from(duration.as_millis()).unwrap_or(UInt::MAX));
		Self(millis)
	}
}

#[macro_export]
macro_rules! int {
	($value:expr) => {
		$crate::Int::from($value)
	};
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RoomVersionId {
	V1,
	V2,
	V3,
	V4,
	V5,
	V6,
	V7,
	V8,
	V9,
	V10,
	V11,
	V12,
	Custom(alloc::string::String),
}

impl RoomVersionId {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::V1 => "1",
			Self::V2 => "2",
			Self::V3 => "3",
			Self::V4 => "4",
			Self::V5 => "5",
			Self::V6 => "6",
			Self::V7 => "7",
			Self::V8 => "8",
			Self::V9 => "9",
			Self::V10 => "10",
			Self::V11 => "11",
			Self::V12 => "12",
			Self::Custom(value) => value,
		}
	}
}

impl core::fmt::Display for RoomVersionId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str(self.as_str())
	}
}

impl codec::Serialize for RoomVersionId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.as_str().to_owned())
	}
}
impl codec::Deserialize for RoomVersionId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		let s = value.as_str().ok_or_else(|| codec::DeError::expected("room version"))?;
		Self::try_from(s).map_err(|()| codec::DeError::expected("room version"))
	}
}

impl TryFrom<&str> for RoomVersionId {
	type Error = ();
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(match value {
			"1" => Self::V1,
			"2" => Self::V2,
			"3" => Self::V3,
			"4" => Self::V4,
			"5" => Self::V5,
			"6" => Self::V6,
			"7" => Self::V7,
			"8" => Self::V8,
			"9" => Self::V9,
			"10" => Self::V10,
			"11" => Self::V11,
			"12" => Self::V12,
			other => Self::Custom(other.to_owned()),
		})
	}
}

pub mod int {
	pub use crate::{Int, UInt};
}

pub mod http_headers {
	use alloc::string::String;

	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	pub enum ContentDispositionType {
		Inline,
		Attachment,
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	pub struct ContentDisposition {
		pub disposition: ContentDispositionType,
		pub filename: Option<String>,
	}

	impl ContentDisposition {
		#[must_use]
		pub fn new(disposition: ContentDispositionType) -> Self {
			Self {
				disposition,
				filename: None,
			}
		}
		#[must_use]
		pub fn with_filename(mut self, filename: Option<String>) -> Self {
			self.filename = filename;
			self
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	pub struct ContentDispositionParseError;
}

pub mod api {
	pub mod error {
		use core::fmt;

		/// Error converting a request or response to or from HTTP.
		#[derive(Clone, Debug, Eq, PartialEq)]
		pub struct IntoHttpError(pub alloc::string::String);

		impl fmt::Display for IntoHttpError {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.write_str(&self.0)
			}
		}
		impl core::error::Error for IntoHttpError {}
		impl From<crate::codec::DeError> for IntoHttpError {
			fn from(e: crate::codec::DeError) -> Self {
				Self(e.0)
			}
		}
		impl From<crate::CanonicalJsonError> for IntoHttpError {
			fn from(e: crate::CanonicalJsonError) -> Self {
				Self(alloc::string::ToString::to_string(&e))
			}
		}
		impl From<crate::json::Error> for IntoHttpError {
			fn from(e: crate::json::Error) -> Self {
				Self(alloc::string::ToString::to_string(&e))
			}
		}
	}
	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	pub enum Direction {
		Forward,
		Backward,
	}

	pub mod client {
		pub mod filter {
			#[derive(Clone, Debug, Default)]
			pub struct RoomEventFilter {
				pub not_rooms: alloc::vec::Vec<crate::OwnedRoomId>,
				pub rooms: Option<alloc::vec::Vec<crate::OwnedRoomId>>,
				pub not_senders: alloc::vec::Vec<crate::OwnedUserId>,
				pub senders: Option<alloc::vec::Vec<crate::OwnedUserId>>,
				pub types: Option<alloc::vec::Vec<alloc::string::String>>,
				pub not_types: alloc::vec::Vec<alloc::string::String>,
				pub contains_url: Option<bool>,
				pub not_rooms_or_senders: alloc::vec::Vec<alloc::string::String>,
				pub not_types_or_rel_types: alloc::vec::Vec<alloc::string::String>,
				pub lazy_load_options: Option<()>,
				pub rel_types: Option<alloc::vec::Vec<alloc::string::String>>,
				pub not_rel_types: alloc::vec::Vec<alloc::string::String>,
				pub url_filter: Option<UrlFilter>,
			}
			#[derive(Clone, Copy, Debug, Eq, PartialEq)]
			pub enum UrlFilter {
				EventsWithUrl,
				EventsWithoutUrl,
			}
		}
		pub mod discovery {
			pub mod discover_homeserver {
				#[derive(Clone, Debug, Default)]
				pub struct RtcFocusInfo(pub crate::json::Value);
			}
			pub mod discover_support {
				#[derive(Clone, Debug, Default)]
				pub struct ContactRole(pub crate::json::Value);
			}
			pub mod get_capabilities {
				#[derive(Clone, Copy, Debug, Eq, PartialEq)]
				pub enum RoomVersionStability {
					Stable,
					Unstable,
				}
			}
		}
		pub mod error {
			#[derive(Clone, Debug)]
			pub enum ErrorKind {
				LimitExceeded {
					retry_after_ms: Option<crate::UInt>,
				},
				SenderIgnored {
					room_id: Option<crate::OwnedRoomId>,
				},
				WrongRoomKeysVersion {
					_value: (),
				},
				Forbidden {
					_value: (),
				},
				UnknownToken {
					soft_logout: bool,
				},
				NotImplemented,
				FeatureDisabled,
				NotFound,
				TooLarge,
				Unrecognized,
				CannotOverwriteMedia,
				NotYetUploaded,
				GuestAccessForbidden,
				ThreepidAuthFailed,
				UserDeactivated,
				ThreepidDenied,
				InviteBlocked,
				UserSuspended,
				MissingToken,
				Unauthorized,
				UserLocked,
				Unknown,
				BadJson,
				InvalidParam,
			}
			#[derive(Clone, Debug)]
			pub enum ErrorBody {
				Standard {
					kind: ErrorKind,
					message: alloc::string::String,
				},
				Other,
			}
			#[derive(Clone, Debug)]
			pub struct Error {
				pub status_code: http::StatusCode,
				pub body: ErrorBody,
			}
		}
		pub mod uiaa {
			#[derive(Clone, Debug, Default)]
			pub struct UiaaInfo(pub crate::json::Value);
			#[derive(Clone, Debug)]
			pub enum UiaaResponse {
				AuthResponse(UiaaInfo),
				MatrixError(crate::api::client::error::Error),
			}
		}
	}
	pub trait OutgoingResponse {}
}

pub mod events {
	pub use crate::event_type::{MessageLikeEventType, StateEventType, TimelineEventType};
	pub trait EventContent {
		type EventType;
		fn event_type(&self) -> Self::EventType;
	}
	pub mod relation {
		#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
		pub enum RelationType {
			Reply,
			Replacement,
			Annotation,
			Reference,
			Thread,
		}
		crate::impl_codec_enum!(RelationType {
			Reply => "m.in_reply_to",
			Replacement => "m.replace",
			Annotation => "m.annotation",
			Reference => "m.reference",
			Thread => "m.thread",
		});
	}
	pub mod room {
		pub mod redaction {
			#[derive(Clone, Debug, Default)]
			pub struct RoomRedactionEventContent {
				pub redacts: Option<crate::OwnedEventId>,
			}
		}
		pub mod create {
			#[derive(Clone, Debug, Default)]
			pub struct RoomCreateEventContent {
				pub creator: Option<crate::OwnedUserId>,
				pub room_version: Option<crate::RoomVersionId>,
				pub additional_creators: Option<alloc::vec::Vec<crate::OwnedUserId>>,
			}
			impl crate::codec::Deserialize for RoomCreateEventContent {
				fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
					let object = value
						.as_object()
						.ok_or_else(|| crate::codec::DeError::expected("object"))?;
					let get = |name: &str| object.get(name).filter(|value| !value.is_null());
					Ok(Self {
						creator: get("creator").map(crate::codec::from_value).transpose()?,
						room_version: get("room_version")
							.map(crate::codec::from_value)
							.transpose()?,
						additional_creators: get("additional_creators")
							.map(crate::codec::from_value)
							.transpose()?,
					})
				}
			}
		}
		pub mod member {
			#[derive(Clone, Debug, Default)]
			pub struct RoomMemberEventContent {
				pub membership: MembershipState,
			}
			#[derive(Clone, Debug, Default, PartialEq, Eq)]
			pub enum MembershipState {
				Join,
				Invite,
				#[default]
				Leave,
				Ban,
				Knock,
			}
			crate::impl_codec_enum!(MembershipState {
				Join => "join",
				Invite => "invite",
				Leave => "leave",
				Ban => "ban",
				Knock => "knock",
			});
			#[derive(Clone, Debug)]
			pub struct ThirdPartyInviteSigned {
				pub mxid: crate::OwnedUserId,
				pub token: alloc::string::String,
			}
			#[derive(Clone, Debug)]
			pub struct ThirdPartyInvite {
				pub signed: ThirdPartyInviteSigned,
			}
			impl crate::codec::Deserialize for ThirdPartyInvite {
				fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
					let signed = value
						.get("signed")
						.ok_or_else(|| crate::codec::DeError::expected("signed"))?;
					Ok(Self {
						signed: ThirdPartyInviteSigned {
							mxid: crate::codec::from_value(
								signed
									.get("mxid")
									.ok_or_else(|| crate::codec::DeError::expected("mxid"))?,
							)?,
							token: crate::codec::from_value(
								signed
									.get("token")
									.ok_or_else(|| crate::codec::DeError::expected("token"))?,
							)?,
						},
					})
				}
			}
		}
		pub mod power_levels {
			#[derive(Clone, Debug, Default)]
			pub struct RoomPowerLevelsEventContent {
				pub ban: crate::Int,
				pub events:
					alloc::collections::BTreeMap<crate::events::TimelineEventType, crate::Int>,
				pub events_default: crate::Int,
				pub invite: crate::Int,
				pub kick: crate::Int,
				pub redact: crate::Int,
				pub state_default: crate::Int,
				pub users: alloc::collections::BTreeMap<crate::OwnedUserId, crate::Int>,
				pub users_default: crate::Int,
				pub notifications: crate::power_levels::NotificationPowerLevels,
			}
			impl RoomPowerLevelsEventContent {
				#[must_use]
				pub fn new() -> Self {
					Self {
						ban: 50,
						invite: 0,
						kick: 50,
						redact: 50,
						state_default: 50,
						..Self::default()
					}
				}
			}
		}
		pub mod join_rules {
			#[derive(Clone, Debug, Default)]
			pub enum JoinRule {
				#[default]
				Public,
				Knock,
				Invite,
				Private,
				Restricted(RestrictedRule),
				KnockRestricted(RestrictedRule),
			}
			#[derive(Clone, Debug, Default)]
			pub struct RestrictedRule {
				pub allow: alloc::vec::Vec<crate::json::Value>,
			}
			#[derive(Clone, Debug, Default)]
			pub struct RoomJoinRulesEventContent {
				pub join_rule: JoinRule,
			}
		}
		pub mod third_party_invite {
			#[derive(Clone, Debug, Default)]
			pub struct RoomThirdPartyInviteEventContent {
				pub public_key: Option<alloc::string::String>,
				pub public_keys: Option<alloc::vec::Vec<crate::json::Value>>,
			}
		}
		pub mod space {
			pub mod child {
				#[derive(Clone, Debug, Default)]
				pub struct RoomSpaceChildEventContent;
			}
		}
	}
	pub mod space {
		pub mod child {
			pub use super::super::room::space::child::RoomSpaceChildEventContent;
			pub type HierarchySpaceChildEvent = RoomSpaceChildEventContent;
		}
	}
	#[derive(Clone, Debug, Default)]
	pub struct AnyTimelineEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnySyncTimelineEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyMessageLikeEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyStateEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnySyncStateEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyStrippedStateEvent;
	#[derive(Clone, Debug, Default)]
	pub struct StateEvent<T>(pub core::marker::PhantomData<T>);
}

pub mod power_levels {
	#[derive(Clone, Debug, Default)]
	pub struct NotificationPowerLevels {
		pub room: crate::Int,
	}
	impl NotificationPowerLevels {
		#[must_use]
		pub fn new() -> Self {
			Self {
				room: 50,
			}
		}
	}
	#[must_use]
	pub fn default_power_level() -> crate::Int {
		0
	}
}

pub mod signatures {
	#[derive(Clone, Debug)]
	pub struct Error(pub alloc::string::String);
	/// Computes the event reference hash via Rezzy.
	///
	/// # Errors
	///
	/// Returns an error if the room version has no reference hash or the
	/// canonical JSON writer rejects the object.
	pub fn reference_hash(
		object: &crate::CanonicalJsonObject,
		version: &crate::RoomVersionId,
	) -> Result<alloc::string::String, Error> {
		rezzy::reference_hash(&crate::json::Value::Object(object.clone()), version.as_str())
			.map_err(Error)
	}
}

pub mod serde {
	use core::marker::PhantomData;

	#[must_use]
	pub fn default_true() -> bool {
		true
	}

	#[derive(Clone, Debug, Default)]
	pub struct Raw<T>(pub alloc::string::String, pub PhantomData<T>);

	impl<T> crate::codec::Serialize for Raw<T> {
		fn to_json(&self) -> crate::json::Value {
			crate::json::Value::parse(&self.0).unwrap_or_default()
		}
	}
	impl<T> crate::codec::Deserialize for Raw<T> {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			Ok(Self(crate::codec::to_string(value), PhantomData))
		}
	}

	impl<T> Raw<T> {
		pub fn from_json_text(input: &str) -> Result<Self, crate::codec::DeError> {
			crate::json::Value::parse(input)
				.map(|_| Self(input.to_owned(), PhantomData))
				.map_err(|error| crate::codec::DeError(error.to_string()))
		}

		#[must_use]
		pub fn get(&self) -> &str {
			&self.0
		}
		/// Deserializes the raw JSON into `U`.
		///
		/// # Errors
		///
		/// Returns an error if the JSON is invalid or does not match `U`.
		pub fn deserialize_as<U: crate::codec::Deserialize>(
			&self,
		) -> Result<U, crate::codec::DeError> {
			crate::codec::from_str(&self.0)
		}
	}

	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	pub struct Base64(pub alloc::vec::Vec<u8>);
	use crate::{Int, codec::DeError, json::Value};

	/// Reads a power level that v1 rooms may spell as a string or integer.
	///
	/// # Errors
	///
	/// Returns an error if the value is neither an integer nor a numeric string.
	pub fn deserialize_v1_powerlevel(value: &Value) -> Result<Int, DeError> {
		value
			.as_i64()
			.or_else(|| value.as_str().and_then(|s| s.parse().ok()))
			.ok_or_else(|| DeError::expected("power level"))
	}

	/// Reads a map of power levels in either v1 or integer form.
	///
	/// # Errors
	///
	/// Returns an error if `value` is not an object of power levels.
	pub fn vec_deserialize_int_powerlevel_values(
		value: &Value,
	) -> Result<alloc::collections::BTreeMap<alloc::string::String, Int>, DeError> {
		value
			.as_object()
			.ok_or_else(|| DeError::expected("object"))?
			.iter()
			.map(|(k, v)| Ok((k.clone(), deserialize_v1_powerlevel(v)?)))
			.collect()
	}

	/// Reads an array of power levels in either v1 or integer form.
	///
	/// # Errors
	///
	/// Returns an error if `value` is not an array of power levels.
	pub fn vec_deserialize_v1_powerlevel_values(
		value: &Value,
	) -> Result<alloc::vec::Vec<Int>, DeError> {
		value
			.as_array()
			.ok_or_else(|| DeError::expected("array"))?
			.iter()
			.map(deserialize_v1_powerlevel)
			.collect()
	}
}

#[derive(Clone, Debug)]
pub struct JsParseIntError;
#[derive(Clone, Debug)]
pub struct JsTryFromIntError;
#[derive(Clone, Debug)]
pub struct MxcUriError;
#[derive(Clone, Debug)]
pub struct IdParseError;

/// Matrix-facing names shared by the server and the serialization layer.
///
/// This is intentionally kept at the crate root so the eventual migration
/// from `ruma` can be a namespace change rather than another JSON rewrite.
pub type CanonicalJsonObject = canonical_json::Object;
pub type CanonicalJsonValue = canonical_json::Value;
pub type CanonicalJsonArray = canonical_json::Array;
/// Error converting or validating canonical JSON.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalJsonError {
	SerDe(alloc::string::String),
}

impl fmt::Display for CanonicalJsonError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::SerDe(error) => f.write_str(error),
		}
	}
}
impl core::error::Error for CanonicalJsonError {}

#[cfg(test)]
mod codec_tests {
	use crate::{
		OwnedEventId, RoomVersionId,
		codec::{from_str, to_string},
		events::{TimelineEventType, room::member::MembershipState},
	};

	#[test]
	fn ids_and_enums_round_trip() {
		let id = OwnedEventId::from("$abc:example.org");
		assert_eq!(from_str::<OwnedEventId>(&to_string(&id)).unwrap(), id);
		let ty = TimelineEventType::RoomMember;
		assert_eq!(to_string(&ty), "\"m.room.member\"");
		assert_eq!(from_str::<TimelineEventType>("\"m.room.member\"").unwrap(), ty);
		assert!(from_str::<MembershipState>("\"nope\"").is_err());
		assert_eq!(from_str::<RoomVersionId>("\"11\"").unwrap(), RoomVersionId::V11);
	}
}

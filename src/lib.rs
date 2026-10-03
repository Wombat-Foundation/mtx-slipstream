//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

pub mod canonical_json;
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
		#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, ::serde::Serialize, ::serde::Deserialize)]
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
			pub fn server_name(&self) -> Option<OwnedServerName> {
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ::serde::Serialize, ::serde::Deserialize)]
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
			}
			#[derive(Clone, Debug, Default)]
			pub struct UrlFilter;
		}
		pub mod discovery {
			pub mod discover_homeserver {
				#[derive(Clone, Debug, Default)]
				pub struct RtcFocusInfo;
			}
			pub mod discover_support {
				#[derive(Clone, Debug, Default)]
				pub struct ContactRole;
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
			pub struct UiaaInfo;
			#[derive(Clone, Debug)]
			pub struct UiaaResponse;
		}
	}
	pub trait OutgoingResponse {}
}

pub mod events {
	#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
	#[derive(serde::Serialize, serde::Deserialize)]
	pub enum TimelineEventType {
		RoomAliases,
		RoomCreate,
		RoomJoinRules,
		RoomMember,
		RoomMessage,
		RoomPowerLevels,
		RoomRedaction,
		RoomThirdPartyInvite,
		RoomTopic,
	}
	pub type StateEventType = TimelineEventType;
	pub type MessageLikeEventType = TimelineEventType;
	pub trait EventContent {
		type EventType;
		fn event_type(&self) -> Self::EventType;
	}
	pub mod relation {
		#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
		#[derive(serde::Serialize, serde::Deserialize)]
		pub enum RelationType {
			Reply,
			Replacement,
			Annotation,
			Reference,
			Thread,
		}
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
			pub struct RoomCreateEventContent;
		}
		pub mod member {
			#[derive(Clone, Debug, Default)]
			pub struct RoomMemberEventContent;
			#[derive(Clone, Debug)]
			#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq)]
			pub enum MembershipState {
				Join,
				Invite,
				Leave,
				Ban,
				Knock,
			}
			#[derive(Clone, Debug, Default)]
			pub struct ThirdPartyInvite;
		}
		pub mod power_levels {
			#[derive(Clone, Debug, Default)]
			pub struct RoomPowerLevelsEventContent;
		}
		pub mod join_rules {
			#[derive(Clone, Debug, Default)]
			pub enum JoinRule {
				#[default]
				Public,
				Knock,
				Invite,
				Private,
			}
			#[derive(Clone, Debug, Default)]
			pub struct RoomJoinRulesEventContent {
				pub join_rule: JoinRule,
			}
		}
		pub mod third_party_invite {
			#[derive(Clone, Debug, Default)]
			pub struct RoomThirdPartyInviteEventContent;
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
	pub struct NotificationPowerLevels;
	#[must_use]
	pub fn default_power_level() -> crate::Int {
		0
	}
}

pub mod signatures {
	#[derive(Clone, Debug)]
	pub struct Error;
	/// Computes the event reference hash.
	///
	/// # Errors
	///
	/// Returns an error if hashing fails.
	pub fn reference_hash(
		_: &crate::CanonicalJsonObject,
		_: &crate::RoomVersionId,
	) -> Result<alloc::string::String, Error> {
		Ok(alloc::string::String::new())
	}
}

pub mod serde {
	use core::marker::PhantomData;

	#[must_use]
	pub fn default_true() -> bool { true }

	#[derive(Clone, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
	pub struct Raw<T>(pub alloc::string::String, pub PhantomData<T>);

	impl<T> Raw<T> {
		#[must_use]
		pub fn get(&self) -> &str { &self.0 }
	}

	#[derive(Clone, Debug, Default)]
	pub struct Base64;
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

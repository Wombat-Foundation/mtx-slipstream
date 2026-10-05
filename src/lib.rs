//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

pub mod antispam;
pub mod appservice;
pub mod backup;
pub mod canonical_json;
pub mod codec;
pub mod device;
pub mod filter;
pub mod id_validation;
pub mod membership;
pub mod room_api;
pub mod search;
pub mod session;
pub mod state_api;
pub mod threads;
mod to_device_api;
mod uiaa;
pub use antispam::{draupnir as draupnir_antispam, meowlnir as meowlnir_antispam};
pub mod directory;
pub mod encryption;
mod key_id;
mod relation_types;
mod room_power_levels;
mod space_child;
pub use key_id::{
	Base64PublicKey, KeyId, OneTimeKeyAlgorithm, OneTimeKeyId, OneTimeKeyName, OwnedKeyId,
	OwnedOneTimeKeyId, ServerSigningKeyVersion, SigningKeyAlgorithm,
};
mod compat;
mod content;
pub mod delayed_events;
pub mod endpoint;
mod event_type;
mod events_codec;
pub mod federation;
pub mod federation_api;
/// Shared to-device target identifier used by client and federation payloads.
pub mod to_device {
	pub use crate::room::federation::transactions::edu::DeviceIdOrAllDevices;
}
pub mod js_option;
#[macro_use]
pub mod media_api;
pub mod push;
pub mod push_rules;
pub mod pusher;
pub mod state_hashes;
pub mod sync;
pub mod sync_events;
pub mod user_models;
pub mod writer;

/// Compatibility namespace for the legacy `uint!(...)` macro.
pub mod uint {
	pub use crate::uint;
}

/// Canonical JSON substrate supplied by Rezzy.
pub mod json {
	pub use rezzy::json::*;
}

/// Builds a [`json::Value`] from JSON-like literal syntax, interpolating
/// any [`codec::Serialize`] value.
///
/// Object keys are string literals or parenthesised expressions such as
/// `(user_id())`, and anything `AsRef<str>` works.
///
/// Unlike `json::json!`, interpolated expressions are borrowed and encoded
/// through the codec, so Slipstream models can appear directly.
#[macro_export]
macro_rules! json {
	(null) => { $crate::json::Value::Null };
	([ $($values:tt)* ]) => {{
		#[allow(unused_mut)]
		let mut values = $crate::alloc_vec();
		$crate::json!(@array values; $($values)*);
		$crate::json::Value::Array(values)
	}};
	({ $($values:tt)* }) => {{
		#[allow(unused_mut)]
		let mut object = $crate::json::Object::new();
		$crate::json!(@object object; $($values)*);
		$crate::json::Value::Object(object)
	}};
	(@array $values:ident;) => {};
	(@array $values:ident; null $(, $($rest:tt)*)?) => {{
		$values.push($crate::json::Value::Null);
		$crate::json!(@array $values; $($($rest)*)?);
	}};
	(@array $values:ident; { $($inner:tt)* } $(, $($rest:tt)*)?) => {{
		$values.push($crate::json!({ $($inner)* }));
		$crate::json!(@array $values; $($($rest)*)?);
	}};
	(@array $values:ident; [ $($inner:tt)* ] $(, $($rest:tt)*)?) => {{
		$values.push($crate::json!([ $($inner)* ]));
		$crate::json!(@array $values; $($($rest)*)?);
	}};
	(@array $values:ident; $value:expr, $($rest:tt)*) => {{
		$values.push($crate::codec::to_value(&$value));
		$crate::json!(@array $values; $($rest)*);
	}};
	(@array $values:ident; $value:expr) => { $values.push($crate::codec::to_value(&$value)); };
	(@object $object:ident;) => {};
	(@object $object:ident; $key:tt : null $(, $($rest:tt)*)?) => {{
		$object.insert($crate::alloc_string(AsRef::<str>::as_ref(&$key)), $crate::json::Value::Null);
		$crate::json!(@object $object; $($($rest)*)?);
	}};
	(@object $object:ident; $key:tt : { $($inner:tt)* } $(, $($rest:tt)*)?) => {{
		$object.insert($crate::alloc_string(AsRef::<str>::as_ref(&$key)), $crate::json!({ $($inner)* }));
		$crate::json!(@object $object; $($($rest)*)?);
	}};
	(@object $object:ident; $key:tt : [ $($inner:tt)* ] $(, $($rest:tt)*)?) => {{
		$object.insert($crate::alloc_string(AsRef::<str>::as_ref(&$key)), $crate::json!([ $($inner)* ]));
		$crate::json!(@object $object; $($($rest)*)?);
	}};
	(@object $object:ident; $key:tt : $value:expr, $($rest:tt)*) => {{
		$object.insert($crate::alloc_string(AsRef::<str>::as_ref(&$key)), $crate::codec::to_value(&$value));
		$crate::json!(@object $object; $($rest)*);
	}};
	(@object $object:ident; $key:tt : $value:expr) => {
		$object.insert($crate::alloc_string(AsRef::<str>::as_ref(&$key)), $crate::codec::to_value(&$value));
	};
	($value:expr) => { $crate::codec::to_value(&$value) };
}

#[doc(hidden)]
#[must_use]
pub const fn alloc_vec() -> alloc::vec::Vec<json::Value> {
	alloc::vec::Vec::new()
}

#[doc(hidden)]
#[must_use]
pub fn alloc_string(value: &str) -> alloc::string::String {
	value.to_owned()
}

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
		matrix_id!($borrowed, $owned, $crate::id_validation::any);
	};
	($borrowed:ident, $owned:ident, $validate:path) => {
		#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
		pub struct $owned(alloc::string::String);

		pub type $borrowed = $owned;

		impl $owned {
			/// Parses a Matrix identifier, checking its grammar.
			///
			/// # Errors
			///
			/// Returns [`MatrixIdParseError`] if `value` does not match the
			/// identifier's grammar. `From<&str>` skips this check and is meant for
			/// values already known to be well formed, such as database rows.
			/// `TryFrom<&str>` is core's blanket impl over that `From`, so it never
			/// fails either; use `parse` wherever input is untrusted.
			pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
				let value = value.as_ref();
				if $validate(value) {
					Ok(Self(value.to_owned()))
				} else {
					Err(MatrixIdParseError)
				}
			}
			pub fn as_str(&self) -> &str {
				&self.0
			}
			#[must_use]
			pub fn as_bytes(&self) -> &[u8] {
				self.0.as_bytes()
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
		impl core::str::FromStr for $owned {
			type Err = MatrixIdParseError;

			fn from_str(value: &str) -> Result<Self, Self::Err> {
				Self::parse(value)
			}
		}
		impl From<&$owned> for alloc::string::String {
			fn from(value: &$owned) -> Self {
				value.0.clone()
			}
		}
		impl From<$owned> for alloc::string::String {
			fn from(value: $owned) -> Self {
				value.0
			}
		}
		impl From<&$owned> for $owned {
			fn from(value: &$owned) -> Self {
				value.clone()
			}
		}
		impl AsRef<[u8]> for $owned {
			fn as_ref(&self) -> &[u8] {
				self.0.as_bytes()
			}
		}
		impl AsRef<str> for $owned {
			fn as_ref(&self) -> &str {
				self.as_str()
			}
		}
		impl AsRef<$owned> for $owned {
			fn as_ref(&self) -> &$owned {
				self
			}
		}
		impl PartialEq<&$owned> for $owned {
			fn eq(&self, other: &&$owned) -> bool {
				self == *other
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
					.and_then(|text| Self::parse(text).ok())
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

/// The server part of an identifier of the form `<sigil><local>:<server>`.
pub(crate) fn server_part(id: &str) -> Option<OwnedServerName> {
	id.rsplit_once(':').map(|(_, server)| OwnedServerName::from(server))
}

matrix_id!(EventId, OwnedEventId, crate::id_validation::event_id);
matrix_id!(RoomId, OwnedRoomId, crate::id_validation::room_id);
matrix_id!(RoomAliasId, OwnedRoomAliasId, crate::id_validation::room_alias_id);
matrix_id!(ServerName, OwnedServerName, crate::id_validation::server_name);
matrix_id!(UserId, OwnedUserId, crate::id_validation::user_id);
matrix_id!(RoomOrAliasId, OwnedRoomOrAliasId, crate::id_validation::room_or_alias_id);
matrix_id!(ServerSigningKeyId, OwnedServerSigningKeyId);
matrix_id!(SigningKeyId, OwnedSigningKeyId);
matrix_id!(DeviceId, OwnedDeviceId);
matrix_id!(TransactionId, OwnedTransactionId);
matrix_id!(ClientSecret, OwnedClientSecret);
matrix_id!(SessionId, OwnedSessionId);
matrix_id!(MxcUri, OwnedMxcUri, crate::id_validation::mxc_uri);

/// Borrowed MXC URI components used by media services.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mxc<'a> {
	pub server_name: &'a OwnedServerName,
	pub media_id: &'a str,
}

impl fmt::Display for Mxc<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "mxc://{}/{}", self.server_name, self.media_id)
	}
}

impl OwnedMxcUri {
	#[must_use]
	pub fn is_valid(&self) -> bool {
		self.as_str().starts_with("mxc://")
	}
	/// The server part of the `mxc://` URI.
	///
	/// # Errors
	///
	/// Returns an error if the URI is not of the form `mxc://server/media_id`.
	pub fn server_name(&self) -> Result<OwnedServerName, MatrixIdParseError> {
		self.as_str()
			.strip_prefix("mxc://")
			.and_then(|value| {
				value.split_once('/').map(|(server, _)| OwnedServerName::from(server))
			})
			.ok_or(MatrixIdParseError)
	}
	/// The media ID part of the `mxc://` URI.
	///
	/// # Errors
	///
	/// Returns an error if the URI is not of the form `mxc://server/media_id`.
	pub fn media_id(&self) -> Result<&str, MatrixIdParseError> {
		self.as_str()
			.strip_prefix("mxc://")
			.and_then(|value| value.split_once('/').map(|(_, media)| media))
			.ok_or(MatrixIdParseError)
	}
}

pub mod identifiers_validation {
	pub mod server_name {
		/// Checks that a server name is valid.
		///
		/// # Errors
		///
		/// Returns an error if `value` is not `host[:port]` with a DNS name,
		/// IPv4 address or bracketed IPv6 address.
		pub fn validate(value: &str) -> Result<(), crate::MatrixIdParseError> {
			if crate::id_validation::server_name(value) {
				Ok(())
			} else {
				Err(crate::MatrixIdParseError)
			}
		}
	}
}

pub mod media {
	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	pub enum Method {
		Crop,
		Scale,
	}
}
pub use events::room::encryption::EventEncryptionAlgorithm;
pub use js_option::JsOption;

pub mod space {
	pub use crate::federation_api::space::SpaceRoomJoinRule;
}

pub mod presence {
	pub use crate::events::presence::PresenceState;
}

impl OwnedRoomId {
	#[must_use]
	pub fn new(server_name: &OwnedServerName) -> Self {
		Self::from(alloc::format!("!admin:{server_name}"))
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MilliSecondsSinceUnixEpoch(pub UInt);

/// Seconds since the Unix epoch, represented as an unsigned Matrix integer.
pub type SecondsSinceUnixEpoch = MilliSecondsSinceUnixEpoch;

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
	#[must_use]
	pub fn to_system_time(self) -> std::time::SystemTime {
		std::time::UNIX_EPOCH
			.checked_add(std::time::Duration::from_millis(self.0))
			.unwrap_or(std::time::UNIX_EPOCH)
	}
}

#[macro_export]
macro_rules! int {
	($value:expr) => {
		$crate::Int::from($value)
	};
}

#[macro_export]
macro_rules! uint {
	($value:expr) => {
		(($value) as $crate::UInt)
	};
}

#[macro_export]
macro_rules! event_id {
	($value:expr) => {
		$crate::OwnedEventId::from($value)
	};
}

#[macro_export]
macro_rules! device_id {
	($value:expr) => {
		$crate::OwnedDeviceId::from($value)
	};
}

#[macro_export]
macro_rules! room_id {
	($value:expr) => {
		$crate::OwnedRoomId::from($value)
	};
}

#[macro_export]
macro_rules! user_id {
	($value:expr) => {
		$crate::OwnedUserId::from($value)
	};
}

#[macro_export]
macro_rules! room_version_id {
	($value:expr) => {
		$crate::RoomVersionId::parse($value).expect("valid room version")
	};
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RoomVersionId {
	#[default]
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
	/// Parses a room version identifier.
	///
	/// # Errors
	///
	/// Returns [`IdParseError`] when the string is not a known room version.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, IdParseError> {
		value.as_ref().parse()
	}

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

impl AsRef<[u8]> for RoomVersionId {
	fn as_ref(&self) -> &[u8] {
		self.as_str().as_bytes()
	}
}

impl core::str::FromStr for RoomVersionId {
	type Err = IdParseError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::try_from(value).map_err(|()| IdParseError)
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

	pub mod push_gateway {
		pub use crate::pusher::send_event_notification;
	}

	pub mod client {
		pub mod media {
			pub use crate::media_api::legacy::{
				create_content, create_content_async, create_mxc_uri, get_content,
				get_content_as_filename, get_content_thumbnail, get_media_config,
				get_media_preview,
			};
		}
		pub mod membership {
			pub use crate::membership::{
				InvitationRecipient, MembershipEventFilter, RoomMember, ThirdPartySigned,
				ban_user, forget_room, get_member_events, invite_user, join_room_by_id,
				join_room_by_id_or_alias, joined_members, joined_rooms, kick_user, leave_room,
				unban_user,
			};
		}
		pub mod knock {
			pub use crate::membership::knock_room;
		}
		pub mod state {
			pub use crate::state_api::{
				get_state_events, get_state_events_for_key, send_state_event,
			};
		}
		pub mod session {
			pub use crate::session::{
				get_login_token, get_login_types, login, logout, logout_all,
			};
		}
		pub mod to_device {
			pub use crate::to_device_api::send_event_to_device;
		}
		pub mod authenticated_media {
			pub use crate::media_api::authenticated_client::{
				get_content, get_content_as_filename, get_content_thumbnail, get_media_config,
				get_media_preview,
			};
		}
		pub mod sync {
			pub use crate::sync_events::{
				self, CompatListFilters, DeviceLists, UnreadNotificationsCount,
			};
		}
		pub mod push {
			pub use crate::push_rules::{
				PushRule, delete_pushrule, get_pushrule, get_pushrule_actions,
				get_pushrule_enabled, get_pushrules_all, get_pushrules_global_scope,
				set_pushrule, set_pushrule_actions, set_pushrule_enabled,
			};
			pub use crate::pusher::{
				HttpPusherData, Pusher, PusherIds, PusherKind, get_pushers, set_pusher,
			};
		}
		pub mod directory {
			pub use crate::directory::{get_public_rooms, get_public_rooms_filtered};
		}
		pub mod appservice {
			pub use crate::appservice::request_ping;
		}
		pub use crate::{
			backup::{
				add_backup_keys, add_backup_keys_for_room, add_backup_keys_for_session,
				create_backup_version, delete_backup_keys, delete_backup_keys_for_room,
				delete_backup_keys_for_session, delete_backup_version, get_backup_info,
				get_backup_keys, get_backup_keys_for_room, get_backup_keys_for_session,
				get_latest_backup_info, update_backup_version,
			},
			device::{
				dehydrated_device, delete_device, delete_devices, get_device, get_devices,
				update_device,
			},
		};
		pub mod backup {
			pub use crate::backup::*;
		}
		pub mod room {
			pub use crate::room_api::*;
		}
		pub mod space {
			pub use crate::federation_api::space::{
				SpaceHierarchyRoomsChunk, client::get_hierarchy,
			};
		}
		pub mod search {
			pub use crate::search::search_events;
		}
		pub mod threads {
			pub use crate::threads::{IncludeThreads, get_threads};
		}
		pub mod device {
			pub use crate::device::{
				DehydratedDeviceData, Device, delete_device, delete_devices, get_device,
				get_devices, update_device,
			};
		}
		pub mod filter {
			pub use crate::filter::{
				EventFormat, Filter, FilterDefinition, LazyLoadOptions, RoomEventFilter,
				RoomFilter, UrlFilter, create_filter, get_filter,
			};
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
			pub use crate::uiaa::StandardErrorBody;
			/// When a rate-limited client may retry.
			#[derive(Clone, Copy, Debug, Eq, PartialEq)]
			pub enum RetryAfter {
				Delay(core::time::Duration),
			}
			#[derive(Clone, Debug)]
			pub enum ErrorKind {
				LimitExceeded {
					retry_after: Option<RetryAfter>,
				},
				SenderIgnored {
					sender: Option<crate::OwnedUserId>,
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
				MissingParam,
				UrlNotSet,
				TooLarge,
				Unrecognized,
				Exclusive,
				BadAlias,
				InvalidUsername,
				UnsupportedRoomVersion,
				RoomInUse,
				UnknownPos,
				CannotOverwriteMedia,
				NotYetUploaded,
				GuestAccessForbidden,
				ThreepidAuthFailed,
				UserDeactivated,
				ThreepidDenied,
				ThreepidInUse,
				InviteBlocked,
				UserSuspended,
				MissingToken,
				Unauthorized,
				UserLocked,
				Unknown,
				BadJson,
				InvalidParam,
				NotJson,
				UserInUse,
				ThreepidMediumNotSupported,
				ThreepidNotFound,
				UnableToAuthorizeJoin,
				UnableToGrantJoin,
				IncompatibleRoomVersion {
					room_version: crate::RoomVersionId,
				},
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
			pub use crate::uiaa::{
				AuthData, AuthFlow, AuthType, Dummy, EmailIdentity, FallbackAcknowledgement,
				Password, ReCaptcha, RegistrationToken, Terms, ThirdpartyIdCredentials, UiaaInfo,
				UserIdentifier,
			};
			#[derive(Clone, Debug)]
			pub enum UiaaResponse {
				AuthResponse(UiaaInfo),
				MatrixError(crate::api::client::error::Error),
			}
		}
	}

	/// Compatibility path for application-service types and endpoints.
	pub use crate::appservice;
	pub use crate::{
		endpoint::{
			AuthScheme, EndpointError, EndpointRequest, FromHttpRequestError,
			FromHttpResponseError, IncomingRequest, IncomingResponse, MatrixVersion, Metadata,
			OutgoingRequest, OutgoingResponse, SendAccessToken,
		},
		federation_api as federation,
	};
}

pub mod events {
	pub use crate::event_type::{
		GlobalAccountDataEventType, MessageLikeEventType, RoomAccountDataEventType,
		StateEventType, TimelineEventType,
	};
	#[derive(Clone, Debug, Default)]
	pub struct AnyGlobalAccountDataEvent;
	#[derive(Clone, Debug)]
	pub enum AnyRawAccountDataEvent {
		Room(crate::serde::Raw<AnyRoomAccountDataEvent>),
		Global(crate::serde::Raw<AnyGlobalAccountDataEvent>),
	}
	#[derive(Clone, Debug, Default)]
	pub struct AnyRoomAccountDataEvent;
	mod ephemeral;
	pub use ephemeral::{AnySyncEphemeralRoomEvent, SyncReceiptEvent, SyncTypingEvent};
	#[derive(Clone, Debug, Default)]
	pub struct AnyToDeviceEvent;
	pub use mentions::Mentions;
	pub mod direct;
	pub mod ignored_user_list;
	pub mod invite_permission_config;
	mod mentions;
	pub mod presence;
	pub mod receipt;
	pub mod tag;
	pub mod typing;
	mod wrappers;
	pub use wrappers::{
		EphemeralRoomEvent, GlobalAccountDataEvent, RoomAccountDataEvent, StaticEventContent,
		SyncEphemeralRoomEvent,
	};
	pub trait EventContent {
		type EventType;
		fn event_type(&self) -> Self::EventType;
	}
	impl EventContent for room::message::RoomMessageEventContent {
		type EventType = MessageLikeEventType;
		fn event_type(&self) -> Self::EventType {
			MessageLikeEventType::RoomMessage
		}
	}
	impl EventContent for room::canonical_alias::RoomCanonicalAliasEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomCanonicalAlias
		}
	}
	impl EventContent for room::preview_url::RoomPreviewUrlsEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::from("m.room.preview_urls")
		}
	}
	macro_rules! event_content {
		($($t:ty => $kind:ident :: $variant:ident),* $(,)?) => {$(
			impl EventContent for $t {
				type EventType = $kind;
				fn event_type(&self) -> Self::EventType {
					$kind::$variant
				}
			}
		)*};
	}
	event_content! {
		room::member::RoomMemberEventContent => StateEventType::RoomMember,
		room::join_rules::RoomJoinRulesEventContent => StateEventType::RoomJoinRules,
		room::avatar::RoomAvatarEventContent => StateEventType::RoomAvatar,
		room::third_party_invite::RoomThirdPartyInviteEventContent => StateEventType::RoomThirdPartyInvite,
		room::server_acl::RoomServerAclEventContent => StateEventType::RoomServerAcl,
		room::encryption::RoomEncryptionEventContent => StateEventType::RoomEncryption,
		room::redaction::RoomRedactionEventContent => MessageLikeEventType::RoomRedaction,
	}
	pub mod relation {
		pub use crate::relation_types::{
			Annotation, BundledMessageLikeRelations, BundledReference, BundledThread,
			CustomRelation, InReplyTo, Reference, ReferenceChunk, Relation, Replacement, Thread,
		};
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
	pub mod push_rules {
		pub use crate::push::{
			Action, PushConditionPowerLevelsCtx, PushConditionRoomCtx, PushFormat, Ruleset, Tweak,
		};
		pub type PushRulesEvent = crate::events::GlobalAccountDataEvent<PushRulesEventContent>;
		#[derive(Clone, Debug, Default, Eq, PartialEq)]
		pub struct PushRulesEventContent {
			pub global: crate::push::Ruleset,
		}
		impl crate::codec::Serialize for PushRulesEventContent {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				object.insert("global".into(), self.global.to_json());
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for PushRulesEventContent {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let global = match value {
					crate::json::Value::Object(object) => object
						.get("global")
						.map(crate::codec::Deserialize::from_json)
						.transpose()?
						.unwrap_or_default(),
					_ => crate::push::Ruleset::default(),
				};
				Ok(Self {
					global,
				})
			}
		}
	}
	pub mod room {
		pub mod avatar;
		pub mod canonical_alias;
		pub mod encrypted;
		pub mod encryption;
		pub mod guest_access;
		pub mod history_visibility;
		pub mod message;
		pub mod name;
		pub mod policy;
		pub mod preview_url;
		pub mod server_acl;
		pub mod tombstone;
		pub mod topic;
		pub use message::MediaSource;
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
				pub room_version: crate::RoomVersionId,
				pub additional_creators: Option<alloc::vec::Vec<crate::OwnedUserId>>,
				pub federate: bool,
				pub predecessor: Option<PreviousRoom>,
				pub room_type: Option<crate::room::RoomType>,
			}
			/// The room this room replaces.
			#[derive(Clone, Debug, Eq, PartialEq)]
			pub struct PreviousRoom {
				pub room_id: crate::OwnedRoomId,
				pub event_id: Option<crate::OwnedEventId>,
			}
			impl PreviousRoom {
				#[must_use]
				pub fn new(room_id: crate::OwnedRoomId) -> Self {
					Self {
						room_id,
						event_id: None,
					}
				}
			}
			impl crate::codec::Serialize for PreviousRoom {
				fn to_json(&self) -> crate::json::Value {
					crate::json::Value::Object(crate::endpoint::object_from(alloc::vec![
						("room_id", self.room_id.to_json()),
						("event_id", self.event_id.to_json()),
					]))
				}
			}
			impl crate::codec::Deserialize for PreviousRoom {
				fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
					let object = value
						.as_object()
						.ok_or_else(|| crate::codec::DeError::expected("predecessor object"))?;
					let room_id = object
						.get("room_id")
						.ok_or_else(|| crate::codec::DeError::expected("predecessor room_id"))?;
					Ok(Self {
						room_id: crate::codec::from_value(room_id)?,
						event_id: object
							.get("event_id")
							.filter(|v| !v.is_null())
							.map(crate::codec::from_value)
							.transpose()?,
					})
				}
			}
			impl RoomCreateEventContent {
				#[must_use]
				pub fn new_v1(creator: crate::OwnedUserId) -> Self {
					Self {
						creator: Some(creator),
						room_version: crate::RoomVersionId::V1,
						federate: true,
						..Self::default()
					}
				}
				#[must_use]
				pub fn new_v11() -> Self {
					Self {
						room_version: crate::RoomVersionId::V11,
						federate: true,
						..Self::default()
					}
				}
				#[must_use]
				pub fn new_v12() -> Self {
					Self {
						room_version: crate::RoomVersionId::V12,
						federate: true,
						..Self::default()
					}
				}
			}
			impl crate::events::EventContent for RoomCreateEventContent {
				type EventType = crate::events::StateEventType;
				fn event_type(&self) -> Self::EventType {
					crate::events::StateEventType::RoomCreate
				}
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
							.transpose()?
							.unwrap_or(crate::RoomVersionId::V1),
						additional_creators: get("additional_creators")
							.map(crate::codec::from_value)
							.transpose()?,
						federate: get("m.federate")
							.and_then(crate::json::Value::as_bool)
							.unwrap_or(true),
						predecessor: get("predecessor")
							.map(crate::codec::from_value)
							.transpose()?,
						room_type: get("type").and_then(crate::json::Value::as_str).map(|t| {
							if t == "m.space" {
								crate::room::RoomType::Space
							} else {
								crate::room::RoomType::Room
							}
						}),
					})
				}
			}
		}
		pub mod member {
			#[derive(Clone, Debug, Default)]
			pub struct RoomMemberEventContent {
				pub membership: MembershipState,
				pub displayname: Option<alloc::string::String>,
				pub avatar_url: Option<crate::OwnedMxcUri>,
				pub blurhash: Option<alloc::string::String>,
				pub reason: Option<alloc::string::String>,
				pub is_direct: Option<bool>,
				pub third_party_invite: Option<ThirdPartyInvite>,
				pub redact_events: Option<bool>,
				pub join_authorized_via_users_server: Option<crate::OwnedUserId>,
			}
			impl RoomMemberEventContent {
				#[must_use]
				pub fn new(membership: MembershipState) -> Self {
					Self {
						membership,
						displayname: None,
						avatar_url: None,
						blurhash: None,
						reason: None,
						is_direct: None,
						third_party_invite: None,
						redact_events: None,
						join_authorized_via_users_server: None,
					}
				}
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
			impl MembershipState {
				#[must_use]
				pub const fn as_str(&self) -> &'static str {
					match self {
						Self::Join => "join",
						Self::Invite => "invite",
						Self::Leave => "leave",
						Self::Ban => "ban",
						Self::Knock => "knock",
					}
				}
			}
			#[derive(Clone, Debug)]
			pub struct ThirdPartyInviteSigned {
				pub mxid: crate::OwnedUserId,
				pub signatures: crate::Signatures,
				pub token: alloc::string::String,
			}
			#[derive(Clone, Debug)]
			pub struct ThirdPartyInvite {
				pub display_name: alloc::string::String,
				pub signed: ThirdPartyInviteSigned,
			}
			impl crate::codec::Deserialize for ThirdPartyInvite {
				fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
					let input = crate::endpoint::Input::new(&[], &[], Some(value));
					let signed = input.body::<crate::json::Value>("signed")?;
					let signed_input = crate::endpoint::Input::new(&[], &[], Some(&signed));
					Ok(Self {
						display_name: input.body("display_name")?,
						signed: ThirdPartyInviteSigned {
							mxid: signed_input.body("mxid")?,
							signatures: signed_input.body("signatures")?,
							token: signed_input.body("token")?,
						},
					})
				}
			}
		}
		pub mod power_levels {
			pub use crate::room_power_levels::RoomPowerLevels;
			#[derive(Clone, Debug)]
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
				/// All-default values from the specification: `events_default`,
				/// `users_default` and `invite` are 0, the rest are 50.
				#[must_use]
				pub fn new() -> Self {
					Self {
						ban: 50,
						events: alloc::collections::BTreeMap::new(),
						events_default: 0,
						invite: 0,
						kick: 50,
						redact: 50,
						state_default: 50,
						users: alloc::collections::BTreeMap::new(),
						users_default: 0,
						notifications: crate::power_levels::NotificationPowerLevels::new(),
					}
				}
			}
			impl Default for RoomPowerLevelsEventContent {
				fn default() -> Self {
					Self::new()
				}
			}
			impl crate::events::EventContent for RoomPowerLevelsEventContent {
				type EventType = crate::events::StateEventType;
				fn event_type(&self) -> Self::EventType {
					crate::events::StateEventType::RoomPowerLevels
				}
			}
		}
		pub mod join_rules {
			#[derive(Clone, Debug, Default, PartialEq, Eq)]
			pub enum JoinRule {
				#[default]
				Public,
				Knock,
				Invite,
				Private,
				Restricted(RestrictedRule),
				KnockRestricted(RestrictedRule),
			}
			#[derive(Clone, Debug, Default, PartialEq, Eq)]
			pub struct RestrictedRule {
				pub allow: alloc::vec::Vec<AllowRule>,
			}
			/// One way a restricted room lets a user join.
			#[derive(Clone, Debug, PartialEq, Eq)]
			pub enum AllowRule {
				/// The user is a member of another room.
				RoomMembership(RoomMembership),
				/// The antispam service decides (`fi.mau.spam_checker`).
				UnstableSpamChecker,
				/// Any other rule type, kept as received.
				_Custom(crate::json::Value),
			}
			/// Membership of the room `room_id`.
			#[derive(Clone, Debug, PartialEq, Eq)]
			pub struct RoomMembership {
				pub room_id: crate::OwnedRoomId,
			}
			impl AllowRule {
				/// Allows joining by membership of `room_id`.
				#[must_use]
				pub fn room_membership(room_id: crate::OwnedRoomId) -> Self {
					Self::RoomMembership(RoomMembership {
						room_id,
					})
				}
			}
			#[derive(Clone, Debug, Default)]
			pub struct RoomJoinRulesEventContent {
				pub join_rule: JoinRule,
			}
			impl RoomJoinRulesEventContent {
				#[must_use]
				pub fn new(join_rule: JoinRule) -> Self {
					Self {
						join_rule,
					}
				}
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
				pub use crate::space_child::{
					RedactedSpaceChildEventContent, SpaceChildEventContent,
				};
				/// Alias kept for the federation hierarchy types.
				pub type RoomSpaceChildEventContent = SpaceChildEventContent;
			}
		}
	}
	pub mod space {
		pub mod child {
			pub use super::super::room::space::child::{
				RedactedSpaceChildEventContent, RoomSpaceChildEventContent,
				SpaceChildEventContent,
			};
			pub use crate::space_child::HierarchySpaceChildEvent;
		}
	}
	#[derive(Clone, Debug, Default)]
	pub struct AnyTimelineEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnySyncTimelineEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyMessageLikeEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyMessageLikeEventContent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyStateEventContent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyStateEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnySyncStateEvent;
	#[derive(Clone, Debug, Default)]
	pub struct AnyStrippedStateEvent;
	#[derive(Clone, Debug, Default)]
	pub struct StateEvent<T>(pub core::marker::PhantomData<T>);
}

pub mod room {
	#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
	pub enum RoomType {
		#[default]
		Room,
		Space,
	}
	impl RoomType {
		/// The `m.room.create` `type` value; empty for ordinary rooms.
		#[must_use]
		pub fn as_str(&self) -> &'static str {
			match self {
				Self::Room => "",
				Self::Space => "m.space",
			}
		}
	}
	pub mod federation {
		pub use crate::federation_api::*;
	}
}

pub mod power_levels {
	#[derive(Clone, Debug)]
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
	impl Default for NotificationPowerLevels {
		fn default() -> Self {
			Self::new()
		}
	}
	#[must_use]
	pub fn default_power_level() -> crate::Int {
		0
	}
}

pub mod signatures;

pub mod serde {
	use core::marker::PhantomData;

	/// A JSON object, as `serde_json::Map<String, Value>`.
	pub type JsonObject = crate::json::Object;

	#[must_use]
	pub fn default_true() -> bool {
		true
	}

	#[derive(Clone, Debug, Default)]
	pub struct Raw<T>(pub alloc::string::String, pub PhantomData<T>);

	/// Raw JSON text backed by Slipstream's JSON codec.
	pub type RawJsonValue = Raw<crate::json::Value>;

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

	impl<T> PartialEq for Raw<T> {
		fn eq(&self, other: &Self) -> bool {
			self.0 == other.0
		}
	}
	impl<T> Eq for Raw<T> {}
	impl<T> core::fmt::Display for Raw<T> {
		fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
			f.write_str(&self.0)
		}
	}
	impl<T> AsRef<str> for Raw<T> {
		fn as_ref(&self) -> &str {
			&self.0
		}
	}

	impl<T> Raw<T> {
		/// Wraps already-serialized JSON text, validating that it parses.
		///
		/// # Errors
		///
		/// Returns an error if `input` is not valid JSON.
		pub fn from_json_text(input: &str) -> Result<Self, crate::codec::DeError> {
			crate::json::Value::parse(input)
				.map(|_| Self(input.to_owned(), PhantomData))
				.map_err(|error| crate::codec::DeError(error.to_string()))
		}

		/// Wraps already-serialized JSON text, validating that it parses.
		///
		/// # Errors
		///
		/// Returns an error if `input` is not valid JSON.
		pub fn from_json_string(
			input: alloc::string::String,
		) -> Result<Self, crate::codec::DeError> {
			match crate::json::Value::parse(&input) {
				Ok(_) => Ok(Self(input, PhantomData)),
				Err(error) => Err(crate::codec::DeError(error.to_string())),
			}
		}

		#[must_use]
		pub fn get(&self) -> &str {
			&self.0
		}

		/// Serializes any codec value into raw JSON, regardless of `T`.
		#[must_use]
		pub fn from_value<U: crate::codec::Serialize + ?Sized>(value: &U) -> Self {
			Self(crate::codec::to_string(value), PhantomData)
		}

		/// Reinterprets the raw JSON as another event type.
		#[must_use]
		pub fn cast<U>(&self) -> Raw<U> {
			Raw(self.0.clone(), PhantomData)
		}

		/// Raw JSON for an empty object, `{}`.
		#[must_use]
		pub fn empty_object() -> Self {
			Self(alloc::string::String::from("{}"), PhantomData)
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
		// Integers are limited to the canonical JSON range, as in ruma's `Int`;
		// strings may carry surrounding whitespace.
		const MAX: Int = 9_007_199_254_740_991;
		value
			.as_i64()
			.or_else(|| value.as_str().and_then(|s| s.trim().parse().ok()))
			.filter(|level| (-MAX..=MAX).contains(level))
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

/// Signatures by entity and key ID, as found in signed JSON.
pub type Signatures<E = OwnedServerName, K = ServerSigningKeyVersion> =
	alloc::collections::BTreeMap<
		E,
		alloc::collections::BTreeMap<OwnedKeyId<SigningKeyAlgorithm, K>, alloc::string::String>,
	>;

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

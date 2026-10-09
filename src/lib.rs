//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
mod slipstream_json;
mod space_child;
pub use key_id::{
	Base64PublicKey, KeyId, OneTimeKeyAlgorithm, OneTimeKeyId, OneTimeKeyName, OwnedKeyId,
	OwnedOneTimeKeyId, ServerSigningKeyVersion, SigningKeyAlgorithm,
};
pub mod client_api;
mod compat;
mod content;
pub mod delayed_events;
pub mod endpoint;
mod event_type;
mod events_codec;
pub mod federation;
pub mod federation_api;
pub mod thirdparty;
pub mod continuwuity_admin_api {
	pub mod rooms {
		pub mod ban {
			pub mod v1 {
				use crate::OwnedRoomId;
				pub struct Request {
					pub room_id: OwnedRoomId,
					pub banned: bool,
				}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
						"PUT",
						"/_continuwuity/admin/rooms/{room_id}/ban",
					);
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
							&self.room_id,
						)])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [("banned", crate::endpoint::enc(&self.banned))],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {
							room_id: input.path()?,
							banned: input.body("banned")?,
						};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub evicted: Vec<crate::OwnedUserId>,
					pub failed_evicted: Vec<crate::OwnedUserId>,
					pub aliases: Vec<crate::OwnedRoomAliasId>,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [
							("evicted", crate::endpoint::enc(&self.evicted)),
							("failed_evicted", crate::endpoint::enc(&self.failed_evicted)),
							("aliases", crate::endpoint::enc(&self.aliases)),
						])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							evicted: input.body("evicted")?,
							failed_evicted: input.body("failed_evicted")?,
							aliases: input.body("aliases")?,
						})
					}
				}
			}
		}
		pub mod list {
			pub mod v1 {
				use crate::OwnedRoomId;
				pub struct Request {}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata =
						crate::endpoint::Metadata::new("GET", "/_continuwuity/admin/rooms/list");
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub rooms: Vec<OwnedRoomId>,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [(
							"rooms",
							crate::endpoint::enc(&self.rooms),
						)])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							rooms: input.body("rooms")?,
						})
					}
				}
			}
		}
	}
}
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

/// The server part of an identifier of the form `<sigil><local>:<server>`.
pub(crate) fn server_part(id: &str) -> Option<OwnedServerName> {
	id.split_once(':').map(|(_, server)| OwnedServerName::from_trusted(server))
}

#[cfg(test)]
mod server_part_tests {
	use super::server_part;

	#[test]
	fn preserves_server_ports() {
		assert_eq!(server_part("@user:172.17.0.1:1027").unwrap().as_str(), "172.17.0.1:1027");
	}
}

#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedEventId(alloc::string::String);
pub type EventId = OwnedEventId;
impl OwnedEventId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::event_id(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedEventId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedEventId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedEventId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedEventId> for alloc::string::String {
	fn from(value: &OwnedEventId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedEventId> for alloc::string::String {
	fn from(value: OwnedEventId) -> Self {
		value.0
	}
}
impl From<&OwnedEventId> for OwnedEventId {
	fn from(value: &OwnedEventId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedEventId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedEventId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedEventId> for OwnedEventId {
	fn as_ref(&self) -> &OwnedEventId {
		self
	}
}
impl PartialEq<&OwnedEventId> for OwnedEventId {
	fn eq(&self, other: &&OwnedEventId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedEventId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedEventId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedEventId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedEventId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedEventId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedEventId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedEventId)))
	}
}
impl fmt::Display for OwnedEventId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedRoomId(alloc::string::String);
pub type RoomId = OwnedRoomId;
impl OwnedRoomId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::room_id(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedRoomId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedRoomId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedRoomId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedRoomId> for alloc::string::String {
	fn from(value: &OwnedRoomId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedRoomId> for alloc::string::String {
	fn from(value: OwnedRoomId) -> Self {
		value.0
	}
}
impl From<&OwnedRoomId> for OwnedRoomId {
	fn from(value: &OwnedRoomId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedRoomId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedRoomId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedRoomId> for OwnedRoomId {
	fn as_ref(&self) -> &OwnedRoomId {
		self
	}
}
impl PartialEq<&OwnedRoomId> for OwnedRoomId {
	fn eq(&self, other: &&OwnedRoomId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedRoomId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedRoomId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedRoomId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedRoomId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedRoomId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedRoomId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedRoomId)))
	}
}
impl fmt::Display for OwnedRoomId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedRoomAliasId(alloc::string::String);
pub type RoomAliasId = OwnedRoomAliasId;
impl OwnedRoomAliasId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::room_alias_id(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedRoomAliasId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedRoomAliasId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedRoomAliasId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedRoomAliasId> for alloc::string::String {
	fn from(value: &OwnedRoomAliasId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedRoomAliasId> for alloc::string::String {
	fn from(value: OwnedRoomAliasId) -> Self {
		value.0
	}
}
impl From<&OwnedRoomAliasId> for OwnedRoomAliasId {
	fn from(value: &OwnedRoomAliasId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedRoomAliasId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedRoomAliasId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedRoomAliasId> for OwnedRoomAliasId {
	fn as_ref(&self) -> &OwnedRoomAliasId {
		self
	}
}
impl PartialEq<&OwnedRoomAliasId> for OwnedRoomAliasId {
	fn eq(&self, other: &&OwnedRoomAliasId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedRoomAliasId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedRoomAliasId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedRoomAliasId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedRoomAliasId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedRoomAliasId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedRoomAliasId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedRoomAliasId)))
	}
}
impl fmt::Display for OwnedRoomAliasId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedServerName(alloc::string::String);
pub type ServerName = OwnedServerName;
impl OwnedServerName {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::server_name(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedServerName {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedServerName {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedServerName {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedServerName> for alloc::string::String {
	fn from(value: &OwnedServerName) -> Self {
		value.0.clone()
	}
}
impl From<OwnedServerName> for alloc::string::String {
	fn from(value: OwnedServerName) -> Self {
		value.0
	}
}
impl From<&OwnedServerName> for OwnedServerName {
	fn from(value: &OwnedServerName) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedServerName {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedServerName {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedServerName> for OwnedServerName {
	fn as_ref(&self) -> &OwnedServerName {
		self
	}
}
impl PartialEq<&OwnedServerName> for OwnedServerName {
	fn eq(&self, other: &&OwnedServerName) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedServerName {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedServerName {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedServerName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedServerName)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedServerName {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedServerName {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedServerName)))
	}
}
impl fmt::Display for OwnedServerName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedUserId(alloc::string::String);
pub type UserId = OwnedUserId;
impl OwnedUserId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::user_id(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedUserId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedUserId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedUserId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedUserId> for alloc::string::String {
	fn from(value: &OwnedUserId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedUserId> for alloc::string::String {
	fn from(value: OwnedUserId) -> Self {
		value.0
	}
}
impl From<&OwnedUserId> for OwnedUserId {
	fn from(value: &OwnedUserId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedUserId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedUserId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedUserId> for OwnedUserId {
	fn as_ref(&self) -> &OwnedUserId {
		self
	}
}
impl PartialEq<&OwnedUserId> for OwnedUserId {
	fn eq(&self, other: &&OwnedUserId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedUserId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedUserId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedUserId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedUserId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedUserId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedUserId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedUserId)))
	}
}
impl fmt::Display for OwnedUserId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl OwnedUserId {
	/// Validates an owned user ID that may have been constructed from an
	/// unchecked database or wire string.
	///
	/// # Errors
	///
	/// Returns an error when the identifier does not match the Matrix user ID grammar.
	pub fn validate_strict(&self) -> Result<(), MatrixIdParseError> {
		let conforming = crate::id_validation::user_id(self.as_str())
			&& crate::id_validation::user_localpart_is_fully_conforming(self.localpart());
		conforming.then_some(()).ok_or(MatrixIdParseError)
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedRoomOrAliasId(alloc::string::String);
pub type RoomOrAliasId = OwnedRoomOrAliasId;
impl OwnedRoomOrAliasId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::room_or_alias_id(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedRoomOrAliasId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedRoomOrAliasId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedRoomOrAliasId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedRoomOrAliasId> for alloc::string::String {
	fn from(value: &OwnedRoomOrAliasId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedRoomOrAliasId> for alloc::string::String {
	fn from(value: OwnedRoomOrAliasId) -> Self {
		value.0
	}
}
impl From<&OwnedRoomOrAliasId> for OwnedRoomOrAliasId {
	fn from(value: &OwnedRoomOrAliasId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedRoomOrAliasId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedRoomOrAliasId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedRoomOrAliasId> for OwnedRoomOrAliasId {
	fn as_ref(&self) -> &OwnedRoomOrAliasId {
		self
	}
}
impl PartialEq<&OwnedRoomOrAliasId> for OwnedRoomOrAliasId {
	fn eq(&self, other: &&OwnedRoomOrAliasId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedRoomOrAliasId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedRoomOrAliasId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedRoomOrAliasId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedRoomOrAliasId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedRoomOrAliasId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedRoomOrAliasId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedRoomOrAliasId)))
	}
}
impl fmt::Display for OwnedRoomOrAliasId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedServerSigningKeyId(alloc::string::String);
pub type ServerSigningKeyId = OwnedServerSigningKeyId;
impl OwnedServerSigningKeyId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::any(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedServerSigningKeyId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedServerSigningKeyId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedServerSigningKeyId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedServerSigningKeyId> for alloc::string::String {
	fn from(value: &OwnedServerSigningKeyId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedServerSigningKeyId> for alloc::string::String {
	fn from(value: OwnedServerSigningKeyId) -> Self {
		value.0
	}
}
impl From<&OwnedServerSigningKeyId> for OwnedServerSigningKeyId {
	fn from(value: &OwnedServerSigningKeyId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedServerSigningKeyId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedServerSigningKeyId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedServerSigningKeyId> for OwnedServerSigningKeyId {
	fn as_ref(&self) -> &OwnedServerSigningKeyId {
		self
	}
}
impl PartialEq<&OwnedServerSigningKeyId> for OwnedServerSigningKeyId {
	fn eq(&self, other: &&OwnedServerSigningKeyId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedServerSigningKeyId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedServerSigningKeyId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedServerSigningKeyId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedServerSigningKeyId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedServerSigningKeyId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedServerSigningKeyId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedServerSigningKeyId)))
	}
}
impl fmt::Display for OwnedServerSigningKeyId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedSigningKeyId(alloc::string::String);
pub type SigningKeyId = OwnedSigningKeyId;
impl OwnedSigningKeyId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::any(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedSigningKeyId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedSigningKeyId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedSigningKeyId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedSigningKeyId> for alloc::string::String {
	fn from(value: &OwnedSigningKeyId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedSigningKeyId> for alloc::string::String {
	fn from(value: OwnedSigningKeyId) -> Self {
		value.0
	}
}
impl From<&OwnedSigningKeyId> for OwnedSigningKeyId {
	fn from(value: &OwnedSigningKeyId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedSigningKeyId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedSigningKeyId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedSigningKeyId> for OwnedSigningKeyId {
	fn as_ref(&self) -> &OwnedSigningKeyId {
		self
	}
}
impl PartialEq<&OwnedSigningKeyId> for OwnedSigningKeyId {
	fn eq(&self, other: &&OwnedSigningKeyId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedSigningKeyId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedSigningKeyId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedSigningKeyId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedSigningKeyId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedSigningKeyId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedSigningKeyId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedSigningKeyId)))
	}
}
impl fmt::Display for OwnedSigningKeyId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedDeviceId(alloc::string::String);
pub type DeviceId = OwnedDeviceId;
impl OwnedDeviceId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::any(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedDeviceId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedDeviceId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedDeviceId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedDeviceId> for alloc::string::String {
	fn from(value: &OwnedDeviceId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedDeviceId> for alloc::string::String {
	fn from(value: OwnedDeviceId) -> Self {
		value.0
	}
}
impl From<&OwnedDeviceId> for OwnedDeviceId {
	fn from(value: &OwnedDeviceId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedDeviceId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedDeviceId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedDeviceId> for OwnedDeviceId {
	fn as_ref(&self) -> &OwnedDeviceId {
		self
	}
}
impl PartialEq<&OwnedDeviceId> for OwnedDeviceId {
	fn eq(&self, other: &&OwnedDeviceId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedDeviceId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedDeviceId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedDeviceId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedDeviceId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedDeviceId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedDeviceId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedDeviceId)))
	}
}
impl fmt::Display for OwnedDeviceId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedTransactionId(alloc::string::String);
pub type TransactionId = OwnedTransactionId;
impl OwnedTransactionId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::any(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedTransactionId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedTransactionId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedTransactionId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedTransactionId> for alloc::string::String {
	fn from(value: &OwnedTransactionId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedTransactionId> for alloc::string::String {
	fn from(value: OwnedTransactionId) -> Self {
		value.0
	}
}
impl From<&OwnedTransactionId> for OwnedTransactionId {
	fn from(value: &OwnedTransactionId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedTransactionId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedTransactionId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedTransactionId> for OwnedTransactionId {
	fn as_ref(&self) -> &OwnedTransactionId {
		self
	}
}
impl PartialEq<&OwnedTransactionId> for OwnedTransactionId {
	fn eq(&self, other: &&OwnedTransactionId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedTransactionId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedTransactionId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedTransactionId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedTransactionId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedTransactionId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedTransactionId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedTransactionId)))
	}
}
impl fmt::Display for OwnedTransactionId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedClientSecret(alloc::string::String);
pub type ClientSecret = OwnedClientSecret;
impl OwnedClientSecret {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::any(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedClientSecret {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedClientSecret {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedClientSecret {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedClientSecret> for alloc::string::String {
	fn from(value: &OwnedClientSecret) -> Self {
		value.0.clone()
	}
}
impl From<OwnedClientSecret> for alloc::string::String {
	fn from(value: OwnedClientSecret) -> Self {
		value.0
	}
}
impl From<&OwnedClientSecret> for OwnedClientSecret {
	fn from(value: &OwnedClientSecret) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedClientSecret {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedClientSecret {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedClientSecret> for OwnedClientSecret {
	fn as_ref(&self) -> &OwnedClientSecret {
		self
	}
}
impl PartialEq<&OwnedClientSecret> for OwnedClientSecret {
	fn eq(&self, other: &&OwnedClientSecret) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedClientSecret {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedClientSecret {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedClientSecret {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedClientSecret)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedClientSecret {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedClientSecret {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedClientSecret)))
	}
}
impl fmt::Display for OwnedClientSecret {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedSessionId(alloc::string::String);
pub type SessionId = OwnedSessionId;
impl OwnedSessionId {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::any(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedSessionId {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedSessionId {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedSessionId {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedSessionId> for alloc::string::String {
	fn from(value: &OwnedSessionId) -> Self {
		value.0.clone()
	}
}
impl From<OwnedSessionId> for alloc::string::String {
	fn from(value: OwnedSessionId) -> Self {
		value.0
	}
}
impl From<&OwnedSessionId> for OwnedSessionId {
	fn from(value: &OwnedSessionId) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedSessionId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedSessionId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedSessionId> for OwnedSessionId {
	fn as_ref(&self) -> &OwnedSessionId {
		self
	}
}
impl PartialEq<&OwnedSessionId> for OwnedSessionId {
	fn eq(&self, other: &&OwnedSessionId) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedSessionId {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedSessionId {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedSessionId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedSessionId)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedSessionId {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedSessionId {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if true {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedSessionId)))
	}
}
impl fmt::Display for OwnedSessionId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OwnedMxcUri(alloc::string::String);
pub type MxcUri = OwnedMxcUri;
impl OwnedMxcUri {
	/// Parses a Matrix identifier, checking its grammar.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if `value` does not match the
	/// identifier's grammar.
	pub fn parse(value: impl AsRef<str>) -> Result<Self, MatrixIdParseError> {
		let value = value.as_ref();
		if crate::id_validation::mxc_uri(value) {
			Ok(Self(value.to_owned()))
		} else {
			Err(MatrixIdParseError)
		}
	}
	/// Constructs an identifier from a value already validated by the
	/// caller, such as a trusted database row or a value generated here.
	///
	/// This bypasses grammar validation and must not be used for wire or
	/// request data.
	#[allow(dead_code)]
	pub(crate) fn from_trusted(value: impl Into<alloc::string::String>) -> Self {
		Self(value.into())
	}
	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.0
	}
	#[must_use]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl TryFrom<alloc::string::String> for OwnedMxcUri {
	type Error = MatrixIdParseError;
	fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl TryFrom<&str> for OwnedMxcUri {
	type Error = MatrixIdParseError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::parse(value)
	}
}
impl core::str::FromStr for OwnedMxcUri {
	type Err = MatrixIdParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::parse(value)
	}
}
impl From<&OwnedMxcUri> for alloc::string::String {
	fn from(value: &OwnedMxcUri) -> Self {
		value.0.clone()
	}
}
impl From<OwnedMxcUri> for alloc::string::String {
	fn from(value: OwnedMxcUri) -> Self {
		value.0
	}
}
impl From<&OwnedMxcUri> for OwnedMxcUri {
	fn from(value: &OwnedMxcUri) -> Self {
		value.clone()
	}
}
impl AsRef<[u8]> for OwnedMxcUri {
	fn as_ref(&self) -> &[u8] {
		self.0.as_bytes()
	}
}
impl AsRef<str> for OwnedMxcUri {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl AsRef<OwnedMxcUri> for OwnedMxcUri {
	fn as_ref(&self) -> &OwnedMxcUri {
		self
	}
}
impl PartialEq<&OwnedMxcUri> for OwnedMxcUri {
	fn eq(&self, other: &&OwnedMxcUri) -> bool {
		self == *other
	}
}
impl Borrow<str> for OwnedMxcUri {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}
impl Deref for OwnedMxcUri {
	type Target = str;
	fn deref(&self) -> &str {
		self.as_str()
	}
}
impl fmt::Debug for OwnedMxcUri {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple(stringify!(OwnedMxcUri)).field(&self.0).finish()
	}
}
impl codec::Serialize for OwnedMxcUri {
	fn to_json(&self) -> json::Value {
		json::Value::String(self.0.clone())
	}
}
impl codec::Deserialize for OwnedMxcUri {
	fn from_json(value: &json::Value) -> Result<Self, codec::DeError> {
		value
			.as_str()
			.and_then(|text| {
				if false {
					Self::parse(text).ok()
				} else {
					// ruma-compatible: MXC URIs are accepted verbatim on the wire;

					// validation happens in `parse`/`is_valid` for callers that need it.
					Some(Self::from_trusted(text))
				}
			})
			.ok_or_else(|| codec::DeError::expected(stringify!(OwnedMxcUri)))
	}
}
impl fmt::Display for OwnedMxcUri {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

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
				value.split_once('/').map(|(server, _)| OwnedServerName::from_trusted(server))
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
	/// Generates a fresh random room ID using the v1 room-ID format.
	///
	/// This constructor does not generate v2 or later room IDs. Those IDs are
	/// derived from the room's create event, have no `:server_name` suffix, and
	/// must be computed by the room creation implementation. See MSC4291.
	///
	/// # Panics
	///
	/// Panics if the operating system's secure random source fails.
	#[must_use]
	pub fn new_v1(server_name: &OwnedServerName) -> Self {
		const ALPHANUMERIC: &[u8; 62] =
			b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
		let mut localpart = [0_u8; 18];
		for slot in &mut localpart {
			loop {
				let mut random_byte = [0_u8; 1];
				getrandom::fill(&mut random_byte)
					.expect("the operating system's random source must be available");
				let byte = random_byte[0];
				if byte < 248 {
					*slot = ALPHANUMERIC[(byte % 62) as usize];
					break;
				}
			}
		}
		Self::from_trusted(alloc::format!(
			"!{}:{server_name}",
			core::str::from_utf8(&localpart).unwrap()
		))
	}

	/// Constructs a v2 room ID from the create-event reference hash.
	///
	/// V2 room IDs contain no server-name suffix.
	///
	/// # Errors
	///
	/// Returns [`MatrixIdParseError`] if the reference hash is not a valid
	/// room-ID localpart.
	pub fn new_v2(reference_hash: &str) -> Result<Self, MatrixIdParseError> {
		Self::parse(alloc::format!("!{reference_hash}"))
	}

	#[deprecated(note = "use OwnedRoomId::new_v1 for v1 room IDs")]
	#[must_use]
	pub fn new(server_name: &OwnedServerName) -> Self {
		Self::new_v1(server_name)
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
	($value:literal) => {
		$crate::OwnedEventId::parse($value).expect("invalid event ID literal")
	};
}

#[macro_export]
macro_rules! device_id {
	($value:literal) => {
		$crate::OwnedDeviceId::parse($value).expect("invalid device ID literal")
	};
}

#[macro_export]
macro_rules! room_id {
	($value:literal) => {
		$crate::OwnedRoomId::parse($value).expect("invalid room ID literal")
	};
}

#[macro_export]
macro_rules! user_id {
	($value:literal) => {
		$crate::OwnedUserId::parse($value).expect("invalid user ID literal")
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

	#[derive(Debug, Eq, PartialEq)]
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

	#[derive(Debug, Eq, PartialEq)]
	pub struct ContentDispositionParseError;
}

pub mod api {
	pub mod error {
		use core::fmt;

		/// Error converting a request or response to or from HTTP.
		#[derive(Debug, Eq, PartialEq)]
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
		pub mod authentication {
			#[derive(Clone, Copy, Debug, Eq, PartialEq)]
			pub enum TokenType {
				Bearer,
			}
			impl crate::codec::Serialize for TokenType {
				fn to_json(&self) -> crate::json::Value {
					crate::json::Value::String(::alloc::string::String::from(match self {
						Self::Bearer => "Bearer",
					}))
				}
			}
			impl crate::codec::Deserialize for TokenType {
				fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
					match value.as_str() {
						Some("Bearer") => Ok(Self::Bearer),
						_ => Err(crate::codec::DeError::expected(stringify!(TokenType))),
					}
				}
			}
		}
		pub use crate::client_api::account;
		pub use crate::client_api::admin;
		pub use crate::client_api::compat::{read_marker, receipt, thirdparty};
		pub use crate::client_api::message_events::get_message_events;
		pub use crate::client_api::profile_keys::{
			delete_profile_key, get_profile_key, set_profile_key,
		};
		pub use crate::client_api::report::report_user;
		pub use crate::client_api::{
			alias, compat, config, context, keys, message, presence, profile, redact, relations,
			report, tag, typing, user_directory, voip,
		};
		pub use authentication::TokenType;
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
			pub mod mutual_rooms {
				pub mod unstable {
					use crate::{OwnedRoomId, OwnedUserId};
					pub struct Request {
						pub user_id: OwnedUserId,
					}
					impl ::core::fmt::Debug for Request {
						fn fmt(
							&self,
							f: &mut crate::endpoint::Fmt<'_>,
						) -> crate::endpoint::FmtResult {
							crate::endpoint::opaque_debug(f, "Request")
						}
					}
					const _: crate::endpoint::Metadata =
						<Request as crate::endpoint::EndpointRequest>::METADATA;
					impl crate::endpoint::EndpointRequest for Request {
						type Response = Response;
						const METADATA: crate::endpoint::Metadata =
							crate::endpoint::Metadata::new(
								"GET",
								"/_matrix/client/unstable/uk.half-shot.msc2666/user/mutual_rooms",
							);
						fn path_args(&self) -> crate::endpoint::Strs {
							crate::endpoint::path_args_from(&mut [])
						}
						fn query(&self) -> crate::endpoint::Pairs {
							crate::endpoint::query_pairs_mut(&mut [(
								"user_id",
								crate::endpoint::enc(&self.user_id),
							)])
						}
						fn body(&self) -> Option<crate::json::Value> {
							crate::endpoint::body_value(
								<Self as crate::endpoint::EndpointRequest>::METADATA.method,
								&mut [],
							)
						}
						fn from_parts(
							path: &[crate::endpoint::Str],
							query: &[crate::endpoint::Pair],
							body: Option<&crate::json::Value>,
						) -> crate::endpoint::Parsed<Self> {
							let input = crate::endpoint::Input::new(path, query, body);
							let value = Self {
								user_id: input.query("user_id")?,
							};
							input.finish()?;
							Ok(value)
						}
					}
					pub struct Response {
						pub joined: alloc::vec::Vec<OwnedRoomId>,
						pub next_batch_token: Option<String>,
					}
					impl ::core::fmt::Debug for Response {
						fn fmt(
							&self,
							f: &mut crate::endpoint::Fmt<'_>,
						) -> crate::endpoint::FmtResult {
							crate::endpoint::opaque_debug(f, "Response")
						}
					}
					impl crate::endpoint::EndpointResponse for Response {
						fn to_body(&self) -> crate::json::Value {
							crate::endpoint::body_object(&mut [
								("joined", crate::endpoint::enc(&self.joined)),
								(
									"next_batch_token",
									crate::endpoint::enc(&self.next_batch_token),
								),
							])
						}
						fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
							let input = crate::endpoint::Input::body_only(body);
							Ok(Self {
								joined: input.body("joined")?,
								next_batch_token: input.body("next_batch_token")?,
							})
						}
					}
				}
			}
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
			pub mod get_room_visibility {
				pub mod v3 {
					use crate::{OwnedRoomId, directory::Visibility};
					pub struct Request {
						pub room_id: OwnedRoomId,
					}
					impl ::core::fmt::Debug for Request {
						fn fmt(
							&self,
							f: &mut crate::endpoint::Fmt<'_>,
						) -> crate::endpoint::FmtResult {
							crate::endpoint::opaque_debug(f, "Request")
						}
					}
					const _: crate::endpoint::Metadata =
						<Request as crate::endpoint::EndpointRequest>::METADATA;
					impl crate::endpoint::EndpointRequest for Request {
						type Response = Response;
						const METADATA: crate::endpoint::Metadata =
							crate::endpoint::Metadata::new(
								"GET",
								"/_matrix/client/v3/directory/list/room/{roomId}",
							);
						fn path_args(&self) -> crate::endpoint::Strs {
							crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
								&self.room_id,
							)])
						}
						fn query(&self) -> crate::endpoint::Pairs {
							crate::endpoint::query_pairs_mut(&mut [])
						}
						fn body(&self) -> Option<crate::json::Value> {
							crate::endpoint::body_value(
								<Self as crate::endpoint::EndpointRequest>::METADATA.method,
								&mut [],
							)
						}
						fn from_parts(
							path: &[crate::endpoint::Str],
							query: &[crate::endpoint::Pair],
							body: Option<&crate::json::Value>,
						) -> crate::endpoint::Parsed<Self> {
							let input = crate::endpoint::Input::new(path, query, body);
							let value = Self {
								room_id: input.path()?,
							};
							input.finish()?;
							Ok(value)
						}
					}
					pub struct Response {
						pub visibility: Visibility,
					}
					impl ::core::fmt::Debug for Response {
						fn fmt(
							&self,
							f: &mut crate::endpoint::Fmt<'_>,
						) -> crate::endpoint::FmtResult {
							crate::endpoint::opaque_debug(f, "Response")
						}
					}
					impl crate::endpoint::EndpointResponse for Response {
						fn to_body(&self) -> crate::json::Value {
							crate::endpoint::body_object(&mut [(
								"visibility",
								crate::endpoint::enc(&self.visibility),
							)])
						}
						fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
							let input = crate::endpoint::Input::body_only(body);
							Ok(Self {
								visibility: input.body("visibility")?,
							})
						}
					}
				}
			}
			pub mod set_room_visibility {
				pub mod v3 {
					use crate::{OwnedRoomId, directory::Visibility};
					pub struct Request {
						pub room_id: OwnedRoomId,
						pub visibility: Visibility,
					}
					impl ::core::fmt::Debug for Request {
						fn fmt(
							&self,
							f: &mut crate::endpoint::Fmt<'_>,
						) -> crate::endpoint::FmtResult {
							crate::endpoint::opaque_debug(f, "Request")
						}
					}
					const _: crate::endpoint::Metadata =
						<Request as crate::endpoint::EndpointRequest>::METADATA;
					impl crate::endpoint::EndpointRequest for Request {
						type Response = Response;
						const METADATA: crate::endpoint::Metadata =
							crate::endpoint::Metadata::new(
								"PUT",
								"/_matrix/client/v3/directory/list/room/{roomId}",
							);
						fn path_args(&self) -> crate::endpoint::Strs {
							crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
								&self.room_id,
							)])
						}
						fn query(&self) -> crate::endpoint::Pairs {
							crate::endpoint::query_pairs_mut(&mut [])
						}
						fn body(&self) -> Option<crate::json::Value> {
							crate::endpoint::body_value(
								<Self as crate::endpoint::EndpointRequest>::METADATA.method,
								&mut [("visibility", crate::endpoint::enc(&self.visibility))],
							)
						}
						fn from_parts(
							path: &[crate::endpoint::Str],
							query: &[crate::endpoint::Pair],
							body: Option<&crate::json::Value>,
						) -> crate::endpoint::Parsed<Self> {
							let input = crate::endpoint::Input::new(path, query, body);
							let value = Self {
								room_id: input.path()?,
								visibility: input.body("visibility")?,
							};
							input.finish()?;
							Ok(value)
						}
					}
					pub struct Response {}
					impl ::core::fmt::Debug for Response {
						fn fmt(
							&self,
							f: &mut crate::endpoint::Fmt<'_>,
						) -> crate::endpoint::FmtResult {
							crate::endpoint::opaque_debug(f, "Response")
						}
					}
					impl crate::endpoint::EndpointResponse for Response {
						fn to_body(&self) -> crate::json::Value {
							crate::endpoint::body_object(&mut [])
						}
						fn from_body(
							_body: &crate::json::Value,
						) -> crate::endpoint::Parsed<Self> {
							Ok(Self {})
						}
					}
				}
			}
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
			pub use crate::client_api::report::room::{report_content, report_room};
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
				pub struct HomeserverInfo {
					pub base_url: alloc::string::String,
				}
				pub struct Request {}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata =
						crate::endpoint::Metadata::new("GET", "/.well-known/matrix/client");
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub homeserver: HomeserverInfo,
					pub identity_server: Option<crate::json::Value>,
					pub sliding_sync_proxy: Option<crate::json::Value>,
					pub tile_server: Option<crate::json::Value>,
					pub rtc_foci: alloc::vec::Vec<RtcFocusInfo>,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [
							("homeserver", crate::endpoint::enc(&self.homeserver)),
							("identity_server", crate::endpoint::enc(&self.identity_server)),
							(
								"sliding_sync_proxy",
								crate::endpoint::enc(&self.sliding_sync_proxy),
							),
							("tile_server", crate::endpoint::enc(&self.tile_server)),
							("rtc_foci", crate::endpoint::enc(&self.rtc_foci)),
						])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							homeserver: input.body("homeserver")?,
							identity_server: input.body("identity_server")?,
							sliding_sync_proxy: input.body("sliding_sync_proxy")?,
							tile_server: input.body("tile_server")?,
							rtc_foci: input.body("rtc_foci")?,
						})
					}
				}
				#[derive(Clone, Debug, Default)]
				pub struct RtcFocusInfo(pub crate::json::Value);
				impl crate::codec::Serialize for HomeserverInfo {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![("base_url", self.base_url.to_json())])
							.into()
					}
				}
				impl crate::codec::Deserialize for HomeserverInfo {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						Ok(Self {
							base_url: crate::codec::Deserialize::from_json(
								v.as_object()
									.and_then(|o| o.get("base_url"))
									.ok_or_else(|| crate::codec::DeError::expected("base_url"))?,
							)?,
						})
					}
				}
			}
			pub mod discover_support {
				#[derive(Clone, Debug, Default)]
				pub struct Contact {
					pub role: ContactRole,
					pub email_address: Option<alloc::string::String>,
					pub matrix_id: Option<alloc::string::String>,
					pub pgp_key: Option<alloc::string::String>,
				}
				pub struct Request {}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata =
						crate::endpoint::Metadata::new("GET", "/.well-known/matrix/support");
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub contacts: alloc::vec::Vec<Contact>,
					pub support_page: Option<alloc::string::String>,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [
							("contacts", crate::endpoint::enc(&self.contacts)),
							("support_page", crate::endpoint::enc(&self.support_page)),
						])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							contacts: input.body("contacts")?,
							support_page: input.body("support_page")?,
						})
					}
				}
				#[derive(Clone, Debug, Default)]
				pub struct ContactRole(pub crate::json::Value);
				impl crate::codec::Serialize for Contact {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![
							("role", self.role.to_json()),
							("email_address", self.email_address.to_json()),
							("matrix_id", self.matrix_id.to_json()),
							("pgp_key", self.pgp_key.to_json()),
						])
						.into()
					}
				}
				impl crate::codec::Deserialize for Contact {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						let o = v
							.as_object()
							.ok_or_else(|| crate::codec::DeError::expected("object"))?;
						Ok(Self {
							role: crate::codec::Deserialize::from_json(
								o.get("role")
									.ok_or_else(|| crate::codec::DeError::expected("role"))?,
							)?,
							email_address: crate::codec::Deserialize::from_json(
								o.get("email_address").unwrap_or(&crate::json::Value::Null),
							)?,
							matrix_id: crate::codec::Deserialize::from_json(
								o.get("matrix_id").unwrap_or(&crate::json::Value::Null),
							)?,
							pgp_key: crate::codec::Deserialize::from_json(
								o.get("pgp_key").unwrap_or(&crate::json::Value::Null),
							)?,
						})
					}
				}
			}
			pub mod get_capabilities {
				#[derive(Clone, Copy, Debug, Eq, PartialEq)]
				pub enum RoomVersionStability {
					Stable,
					Unstable,
				}
				impl crate::codec::Serialize for RoomVersionStability {
					fn to_json(&self) -> crate::json::Value {
						crate::json::Value::String(::alloc::string::String::from(match self {
							Self::Stable => "stable",
							Self::Unstable => "unstable",
						}))
					}
				}
				impl crate::codec::Deserialize for RoomVersionStability {
					fn from_json(
						value: &crate::json::Value,
					) -> Result<Self, crate::codec::DeError> {
						match value.as_str() {
							Some("stable") => Ok(Self::Stable),
							Some("unstable") => Ok(Self::Unstable),
							_ => Err(crate::codec::DeError::expected(stringify!(
								RoomVersionStability
							))),
						}
					}
				}
				#[derive(Clone, Debug, Default)]
				pub struct Capabilities {
					pub room_versions: RoomVersionsCapability,
					pub change_password: ChangePasswordCapability,
					pub set_displayname: SetDisplayNameCapability,
					pub set_avatar_url: SetAvatarUrlCapability,
					pub thirdparty_id_changes: ThirdPartyIdChangesCapability,
					pub get_login_token: GetLoginTokenCapability,
					pub extra: alloc::collections::BTreeMap<String, crate::json::Value>,
				}
				#[derive(Clone, Debug, Default)]
				pub struct RoomVersionsCapability {
					pub available:
						alloc::collections::BTreeMap<crate::RoomVersionId, RoomVersionStability>,
					pub default: crate::RoomVersionId,
				}
				// Like Ruma, these capabilities are advertised as enabled unless changed.
				#[derive(Clone, Debug)]
				pub struct ChangePasswordCapability {
					pub enabled: bool,
				}
				impl Default for ChangePasswordCapability {
					fn default() -> Self {
						Self {
							enabled: true,
						}
					}
				}
				impl crate::codec::Serialize for ChangePasswordCapability {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![("enabled", self.enabled.to_json())])
							.into()
					}
				}
				impl crate::codec::Deserialize for ChangePasswordCapability {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						Ok(Self {
							enabled: crate::codec::Deserialize::from_json(
								v.as_object()
									.and_then(|o| o.get("enabled"))
									.ok_or_else(|| crate::codec::DeError::expected("enabled"))?,
							)?,
						})
					}
				}
				#[derive(Clone, Debug)]
				pub struct SetDisplayNameCapability {
					pub enabled: bool,
				}
				impl Default for SetDisplayNameCapability {
					fn default() -> Self {
						Self {
							enabled: true,
						}
					}
				}
				impl crate::codec::Serialize for SetDisplayNameCapability {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![("enabled", self.enabled.to_json())])
							.into()
					}
				}
				impl crate::codec::Deserialize for SetDisplayNameCapability {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						Ok(Self {
							enabled: crate::codec::Deserialize::from_json(
								v.as_object()
									.and_then(|o| o.get("enabled"))
									.ok_or_else(|| crate::codec::DeError::expected("enabled"))?,
							)?,
						})
					}
				}
				#[derive(Clone, Debug)]
				pub struct SetAvatarUrlCapability {
					pub enabled: bool,
				}
				impl Default for SetAvatarUrlCapability {
					fn default() -> Self {
						Self {
							enabled: true,
						}
					}
				}
				impl crate::codec::Serialize for SetAvatarUrlCapability {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![("enabled", self.enabled.to_json())])
							.into()
					}
				}
				impl crate::codec::Deserialize for SetAvatarUrlCapability {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						Ok(Self {
							enabled: crate::codec::Deserialize::from_json(
								v.as_object()
									.and_then(|o| o.get("enabled"))
									.ok_or_else(|| crate::codec::DeError::expected("enabled"))?,
							)?,
						})
					}
				}
				#[derive(Clone, Debug, Default)]
				pub struct ThirdPartyIdChangesCapability {
					pub enabled: bool,
				}
				#[derive(Clone, Debug, Default)]
				pub struct GetLoginTokenCapability {
					pub enabled: bool,
				}
				impl crate::codec::Serialize for RoomVersionsCapability {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![
							("available", self.available.to_json()),
							("default", self.default.to_json()),
						])
						.into()
					}
				}
				impl crate::codec::Deserialize for RoomVersionsCapability {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						let o = v
							.as_object()
							.ok_or_else(|| crate::codec::DeError::expected("object"))?;
						Ok(Self {
							available: crate::codec::Deserialize::from_json(
								o.get("available").ok_or_else(|| {
									crate::codec::DeError::expected("available")
								})?,
							)?,
							default: crate::codec::Deserialize::from_json(
								o.get("default")
									.ok_or_else(|| crate::codec::DeError::expected("default"))?,
							)?,
						})
					}
				}
				impl crate::codec::Serialize for ThirdPartyIdChangesCapability {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![("enabled", self.enabled.to_json())])
							.into()
					}
				}
				impl crate::codec::Deserialize for ThirdPartyIdChangesCapability {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						Ok(Self {
							enabled: crate::codec::Deserialize::from_json(
								v.as_object()
									.and_then(|o| o.get("enabled"))
									.ok_or_else(|| crate::codec::DeError::expected("enabled"))?,
							)?,
						})
					}
				}
				impl crate::codec::Serialize for GetLoginTokenCapability {
					fn to_json(&self) -> crate::json::Value {
						crate::endpoint::object_from(vec![("enabled", self.enabled.to_json())])
							.into()
					}
				}
				impl crate::codec::Deserialize for GetLoginTokenCapability {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						Ok(Self {
							enabled: crate::codec::Deserialize::from_json(
								v.as_object()
									.and_then(|o| o.get("enabled"))
									.ok_or_else(|| crate::codec::DeError::expected("enabled"))?,
							)?,
						})
					}
				}
				impl crate::codec::Serialize for Capabilities {
					fn to_json(&self) -> crate::json::Value {
						let mut object = crate::endpoint::object_from(vec![
							("m.room_versions", self.room_versions.to_json()),
							("m.change_password", self.change_password.to_json()),
							("m.set_displayname", self.set_displayname.to_json()),
							("m.set_avatar_url", self.set_avatar_url.to_json()),
							("m.3pid_changes", self.thirdparty_id_changes.to_json()),
							("m.get_login_token", self.get_login_token.to_json()),
						]);
						object.extend(self.extra.clone());
						crate::json::Value::Object(object)
					}
				}
				impl crate::codec::Deserialize for Capabilities {
					fn from_json(v: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
						let o = v
							.as_object()
							.ok_or_else(|| crate::codec::DeError::expected("object"))?;
						let extra = o
							.iter()
							.filter(|(key, _)| {
								!matches!(
									key.as_str(),
									"m.room_versions"
										| "m.change_password"
										| "m.set_displayname"
										| "m.set_avatar_url"
										| "m.3pid_changes"
										| "m.get_login_token"
								)
							})
							.map(|(key, value)| (key.clone(), value.clone()))
							.collect();
						Ok(Self {
							room_versions: crate::codec::Deserialize::from_json(
								o.get("m.room_versions").ok_or_else(|| {
									crate::codec::DeError::expected("m.room_versions")
								})?,
							)?,
							change_password: match o.get("m.change_password") {
								Some(v) => crate::codec::Deserialize::from_json(v)?,
								None => ChangePasswordCapability::default(),
							},
							set_displayname: match o.get("m.set_displayname") {
								Some(v) => crate::codec::Deserialize::from_json(v)?,
								None => SetDisplayNameCapability::default(),
							},
							set_avatar_url: match o.get("m.set_avatar_url") {
								Some(v) => crate::codec::Deserialize::from_json(v)?,
								None => SetAvatarUrlCapability::default(),
							},
							thirdparty_id_changes: match o.get("m.3pid_changes") {
								Some(v) => crate::codec::Deserialize::from_json(v)?,
								None => ThirdPartyIdChangesCapability::default(),
							},
							get_login_token: match o.get("m.get_login_token") {
								Some(v) => crate::codec::Deserialize::from_json(v)?,
								None => GetLoginTokenCapability::default(),
							},
							extra,
						})
					}
				}
				impl Capabilities {
					/// Adds or replaces an extra capability.
					///
					/// # Errors
					///
					/// This currently never fails; the result is retained for API compatibility.
					pub fn set(
						&mut self,
						name: impl Into<String>,
						value: crate::json::Value,
					) -> Result<(), crate::codec::DeError> {
						self.extra.insert(name.into(), value);
						Ok(())
					}
				}
				pub struct Request {}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata =
						crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/capabilities");
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub capabilities: Capabilities,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [(
							"capabilities",
							crate::endpoint::enc(&self.capabilities),
						)])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							capabilities: input.body("capabilities")?,
						})
					}
				}
			}
			pub mod get_supported_versions {
				pub struct Request {}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata =
						crate::endpoint::Metadata::new("GET", "/_matrix/client/versions");
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub versions: alloc::vec::Vec<String>,
					pub unstable_features: Option<alloc::collections::BTreeMap<String, bool>>,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [
							("versions", crate::endpoint::enc(&self.versions)),
							("unstable_features", crate::endpoint::enc(&self.unstable_features)),
						])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							versions: input.body("versions")?,
							unstable_features: input.body("unstable_features")?,
						})
					}
				}
			}
			pub mod get_rtc_transports {
				pub struct Request {}
				impl ::core::fmt::Debug for Request {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Request")
					}
				}
				const _: crate::endpoint::Metadata =
					<Request as crate::endpoint::EndpointRequest>::METADATA;
				impl crate::endpoint::EndpointRequest for Request {
					type Response = Response;
					const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
						"GET",
						"/_matrix/client/unstable/org.matrix.msc4143/rtc/transports",
					);
					fn path_args(&self) -> crate::endpoint::Strs {
						crate::endpoint::path_args_from(&mut [])
					}
					fn query(&self) -> crate::endpoint::Pairs {
						crate::endpoint::query_pairs_mut(&mut [])
					}
					fn body(&self) -> Option<crate::json::Value> {
						crate::endpoint::body_value(
							<Self as crate::endpoint::EndpointRequest>::METADATA.method,
							&mut [],
						)
					}
					fn from_parts(
						path: &[crate::endpoint::Str],
						query: &[crate::endpoint::Pair],
						body: Option<&crate::json::Value>,
					) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::new(path, query, body);
						let value = Self {};
						input.finish()?;
						Ok(value)
					}
				}
				pub struct Response {
					pub transports: alloc::vec::Vec<crate::json::Value>,
				}
				impl ::core::fmt::Debug for Response {
					fn fmt(
						&self,
						f: &mut crate::endpoint::Fmt<'_>,
					) -> crate::endpoint::FmtResult {
						crate::endpoint::opaque_debug(f, "Response")
					}
				}
				impl crate::endpoint::EndpointResponse for Response {
					fn to_body(&self) -> crate::json::Value {
						crate::endpoint::body_object(&mut [(
							"transports",
							crate::endpoint::enc(&self.transports),
						)])
					}
					fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
						let input = crate::endpoint::Input::body_only(body);
						Ok(Self {
							transports: input.body("transports")?,
						})
					}
				}
				impl Response {
					#[must_use]
					pub fn new(transports: alloc::vec::Vec<crate::json::Value>) -> Self {
						Self {
							transports,
						}
					}
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
			#[derive(Debug)]
			pub enum ErrorBody {
				Standard {
					kind: ErrorKind,
					message: alloc::string::String,
				},
				Other,
			}
			#[derive(Debug)]
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
			#[derive(Debug)]
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
	#[derive(Debug, Default)]
	pub struct AnyGlobalAccountDataEvent;
	pub type AnyGlobalAccountDataEventContent = AnyGlobalAccountDataEvent;
	#[derive(Debug)]
	pub enum AnyRawAccountDataEvent {
		Room(crate::sswire::Raw<AnyRoomAccountDataEvent>),
		Global(crate::sswire::Raw<AnyGlobalAccountDataEvent>),
	}
	#[derive(Debug, Default)]
	pub struct AnyRoomAccountDataEvent;
	pub type AnyRoomAccountDataEventContent = AnyRoomAccountDataEvent;
	mod ephemeral;
	pub mod fully_read;
	pub use ephemeral::{AnySyncEphemeralRoomEvent, SyncReceiptEvent, SyncTypingEvent};
	#[derive(Debug, Default)]
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
	impl EventContent for room::member::RoomMemberEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomMember
		}
	}
	impl EventContent for room::join_rules::RoomJoinRulesEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomJoinRules
		}
	}
	impl EventContent for room::avatar::RoomAvatarEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomAvatar
		}
	}
	impl EventContent for room::third_party_invite::RoomThirdPartyInviteEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomThirdPartyInvite
		}
	}
	impl EventContent for room::server_acl::RoomServerAclEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomServerAcl
		}
	}
	impl EventContent for room::encryption::RoomEncryptionEventContent {
		type EventType = StateEventType;
		fn event_type(&self) -> Self::EventType {
			StateEventType::RoomEncryption
		}
	}
	impl EventContent for room::redaction::RoomRedactionEventContent {
		type EventType = MessageLikeEventType;
		fn event_type(&self) -> Self::EventType {
			MessageLikeEventType::RoomRedaction
		}
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
		impl crate::codec::Serialize for RelationType {
			fn to_json(&self) -> crate::json::Value {
				crate::json::Value::String(::alloc::string::String::from(match self {
					Self::Reply => "m.in_reply_to",
					Self::Replacement => "m.replace",
					Self::Annotation => "m.annotation",
					Self::Reference => "m.reference",
					Self::Thread => "m.thread",
				}))
			}
		}
		impl crate::codec::Deserialize for RelationType {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				match value.as_str() {
					Some("m.in_reply_to") => Ok(Self::Reply),
					Some("m.replace") => Ok(Self::Replacement),
					Some("m.annotation") => Ok(Self::Annotation),
					Some("m.reference") => Ok(Self::Reference),
					Some("m.thread") => Ok(Self::Thread),
					_ => Err(crate::codec::DeError::expected(stringify!(RelationType))),
				}
			}
		}
	}
	pub mod push_rules {
		pub use crate::push::{
			Action, PushConditionPowerLevelsCtx, PushConditionRoomCtx, PushFormat, Ruleset, Tweak,
		};
		pub type PushRulesEvent = crate::events::GlobalAccountDataEvent<PushRulesEventContent>;
		#[derive(Debug, Default, Eq, PartialEq)]
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
			#[derive(Debug, Default)]
			pub struct RoomRedactionEventContent {
				pub redacts: Option<crate::OwnedEventId>,
				pub reason: Option<String>,
			}
		}
		pub mod create {
			#[derive(Debug, Default)]
			pub struct RoomCreateEventContent {
				pub creator: Option<crate::OwnedUserId>,
				pub room_version: crate::RoomVersionId,
				pub additional_creators: Option<alloc::vec::Vec<crate::OwnedUserId>>,
				pub federate: bool,
				pub predecessor: Option<PreviousRoom>,
				pub room_type: Option<crate::room::RoomType>,
			}
			/// The room this room replaces.
			#[derive(Debug, Eq, PartialEq)]
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
			#[derive(Debug, Default)]
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
			impl crate::codec::Serialize for MembershipState {
				fn to_json(&self) -> crate::json::Value {
					crate::json::Value::String(::alloc::string::String::from(match self {
						Self::Join => "join",
						Self::Invite => "invite",
						Self::Leave => "leave",
						Self::Ban => "ban",
						Self::Knock => "knock",
					}))
				}
			}
			impl crate::codec::Deserialize for MembershipState {
				fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
					match value.as_str() {
						Some("join") => Ok(Self::Join),
						Some("invite") => Ok(Self::Invite),
						Some("leave") => Ok(Self::Leave),
						Some("ban") => Ok(Self::Ban),
						Some("knock") => Ok(Self::Knock),
						_ => Err(crate::codec::DeError::expected(stringify!(MembershipState))),
					}
				}
			}
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
			#[derive(Debug)]
			pub struct ThirdPartyInviteSigned {
				pub mxid: crate::OwnedUserId,
				pub signatures: crate::Signatures,
				pub token: alloc::string::String,
			}
			#[derive(Debug)]
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
			#[derive(Debug, Default)]
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
			#[derive(Debug, Default)]
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
	#[derive(Debug, Default)]
	pub struct AnyTimelineEvent;
	#[derive(Debug, Default)]
	pub struct AnySyncTimelineEvent;
	#[derive(Debug, Default)]
	pub struct AnyMessageLikeEvent;
	#[derive(Debug, Default)]
	pub struct AnyMessageLikeEventContent;
	#[derive(Debug, Default)]
	pub struct AnyStateEventContent;
	#[derive(Debug, Default)]
	pub struct AnyStateEvent;
	#[derive(Debug, Default)]
	pub struct AnySyncStateEvent;
	#[derive(Debug, Default)]
	pub struct AnyStrippedStateEvent;
	#[derive(Debug, Default)]
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

pub mod sswire {
	use core::marker::PhantomData;

	/// A JSON object backed by Slipstream's JSON implementation.
	pub type JsonObject = crate::json::Object;

	#[must_use]
	pub fn default_true() -> bool {
		true
	}

	#[derive(Debug, Default)]
	pub struct Raw<T>(pub alloc::string::String, pub PhantomData<T>);
	impl<T> Clone for Raw<T> {
		fn clone(&self) -> Self {
			Self(self.0.clone(), PhantomData)
		}
	}

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

#[derive(Debug)]
pub struct JsParseIntError;
#[derive(Debug)]
pub struct JsTryFromIntError;
#[derive(Debug)]
pub struct MxcUriError;
#[derive(Debug)]
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
#[derive(Debug, Eq, PartialEq)]
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
		OwnedEventId, OwnedServerName, RoomVersionId,
		codec::{from_str, to_string},
		events::{TimelineEventType, room::member::MembershipState},
	};

	#[test]
	fn ids_and_enums_round_trip() {
		let id = OwnedEventId::parse("$abc:example.org").unwrap();
		assert_eq!(from_str::<OwnedEventId>(&to_string(&id)).unwrap(), id);
		let ty = TimelineEventType::RoomMember;
		assert_eq!(to_string(&ty), "\"m.room.member\"");
		assert_eq!(from_str::<TimelineEventType>("\"m.room.member\"").unwrap(), ty);
		assert!(from_str::<MembershipState>("\"nope\"").is_err());
		assert_eq!(from_str::<RoomVersionId>("\"11\"").unwrap(), RoomVersionId::V11);
	}

	#[test]
	fn new_v1_generates_random_valid_ids() {
		let server = OwnedServerName::parse("example.org").unwrap();
		let first = crate::OwnedRoomId::new_v1(&server);
		let second = crate::OwnedRoomId::new_v1(&server);

		assert_ne!(first, second);
		for room_id in [&first, &second] {
			let (localpart, room_server) = room_id.as_str()[1..].split_once(':').unwrap();
			assert_eq!(localpart.len(), 18);
			assert!(localpart.bytes().all(|byte| byte.is_ascii_alphanumeric()));
			assert_eq!(room_server, server.as_str());
			assert!(crate::OwnedRoomId::parse(room_id.as_str()).is_ok());
		}
	}
}

#[cfg(test)]
mod capabilities_tests {
	use crate::{
		api::client::discovery::get_capabilities::Capabilities,
		codec::{Deserialize, Serialize, from_str},
	};

	#[test]
	fn standard_capabilities_default_to_enabled_and_extras_are_kept() {
		let mut capabilities = Capabilities::default();
		capabilities.set("org.example.extra", crate::json::Value::Bool(true)).unwrap();
		let json = capabilities.to_json();
		let object = json.as_object().unwrap();
		for key in ["m.change_password", "m.set_displayname", "m.set_avatar_url"] {
			let enabled = object.get(key).and_then(|v| v.get("enabled"));
			assert_eq!(enabled.and_then(crate::json::Value::as_bool), Some(true), "{key}");
		}
		assert!(
			object.contains_key("m.room_versions") && object.contains_key("org.example.extra")
		);
	}

	#[test]
	fn room_versions_capability_is_required_and_preserves_advertised_versions() {
		let missing = crate::json::Value::Object(crate::endpoint::object_from(vec![]));
		assert!(Capabilities::from_json(&missing).is_err());

		let capabilities: Capabilities =
			from_str(r#"{"m.room_versions":{"available":{"3":"stable"},"default":"3"}}"#)
				.unwrap();
		assert_eq!(capabilities.room_versions.default, crate::RoomVersionId::V3);
		assert!(capabilities.room_versions.available.contains_key(&crate::RoomVersionId::V3));
	}
}

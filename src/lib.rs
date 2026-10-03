//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

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
		impl AsRef<str> for $owned {
			fn as_ref(&self) -> &str {
				self.as_str()
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

/// Matrix-facing names shared by the server and the serialization layer.
///
/// This is intentionally kept at the crate root so the eventual migration
/// from `ruma` can be a namespace change rather than another JSON rewrite.
pub type CanonicalJsonObject = json::Object;
pub type CanonicalJsonValue = json::Value;
pub type CanonicalJsonArray = alloc::vec::Vec<json::Value>;

pub mod canonical_json {
	pub use crate::{CanonicalJsonArray, CanonicalJsonObject, CanonicalJsonValue};
}

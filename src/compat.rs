//! `Display`, `Error` and comparison impls that ruma's types provide.

use alloc::string::String;
use core::fmt;

use crate::{
	IdParseError, JsParseIntError, JsTryFromIntError, MxcUriError, OwnedEventId,
	OwnedRoomAliasId, OwnedRoomId, OwnedRoomOrAliasId, OwnedServerName, OwnedUserId,
	api::client::error::{Error, ErrorKind},
	events::room::member::MembershipState,
	http_headers::ContentDispositionParseError,
};

macro_rules! simple_error {
	($($t:ty => $msg:literal),* $(,)?) => {$(
		impl fmt::Display for $t {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str($msg) }
		}
		impl core::error::Error for $t {}
	)*};
}

simple_error! {
	JsParseIntError => "failed to parse integer",
	JsTryFromIntError => "integer out of range",
	MxcUriError => "invalid MXC URI",
	IdParseError => "invalid identifier",
	ContentDispositionParseError => "invalid Content-Disposition header",
}

impl fmt::Display for ErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{self:?}")
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.body {
			crate::api::client::error::ErrorBody::Standard {
				message,
				..
			} => f.write_str(message),
			crate::api::client::error::ErrorBody::Other => write!(f, "HTTP {}", self.status_code),
		}
	}
}
impl core::error::Error for Error {}

impl Error {
	#[must_use]
	pub fn error_kind(&self) -> Option<&ErrorKind> {
		match &self.body {
			crate::api::client::error::ErrorBody::Standard {
				kind,
				..
			} => Some(kind),
			crate::api::client::error::ErrorBody::Other => None,
		}
	}
}

impl fmt::Display for MembershipState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Join => "join",
			Self::Invite => "invite",
			Self::Leave => "leave",
			Self::Ban => "ban",
			Self::Knock => "knock",
		})
	}
}

macro_rules! str_eq {
	($($t:ty),*) => {$(
		impl PartialEq<str> for $t {
			fn eq(&self, other: &str) -> bool { self.as_str() == other }
		}
		impl PartialEq<&str> for $t {
			fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
		}
		impl PartialEq<$t> for &str {
			fn eq(&self, other: &$t) -> bool { *self == other.as_str() }
		}
		impl PartialEq<$t> for str {
			fn eq(&self, other: &$t) -> bool { self == other.as_str() }
		}
		impl PartialEq<String> for $t {
			fn eq(&self, other: &String) -> bool { self.as_str() == other }
		}
	)*};
}
str_eq!(
	OwnedEventId,
	OwnedRoomId,
	OwnedRoomAliasId,
	OwnedServerName,
	OwnedUserId,
	OwnedRoomOrAliasId
);

impl OwnedEventId {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		self.server_part()
	}
}
impl OwnedRoomId {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		self.server_part()
	}
}
impl OwnedRoomOrAliasId {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		self.server_part()
	}
}
impl OwnedServerName {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		self.server_part()
	}
}
impl OwnedUserId {
	#[must_use]
	pub fn server_name(&self) -> OwnedServerName {
		self.server_part().unwrap_or_else(|| OwnedServerName::from(""))
	}
}
impl OwnedRoomAliasId {
	#[must_use]
	pub fn server_name(&self) -> OwnedServerName {
		self.server_part().unwrap_or_else(|| OwnedServerName::from(""))
	}
}

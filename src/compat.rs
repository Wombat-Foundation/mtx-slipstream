//! `Display`, `Error` and comparison impls that ruma's types provide.

use alloc::string::String;
use core::fmt;

use crate::{
	IdParseError, JsParseIntError, JsTryFromIntError, MxcUriError, OwnedEventId,
	OwnedRoomAliasId, OwnedRoomId, OwnedRoomOrAliasId, OwnedServerName, OwnedUserId,
	api::client::error::{Error, ErrorKind},
	events::room::member::MembershipState,
	http_headers::{ContentDisposition, ContentDispositionParseError, ContentDispositionType},
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
		crate::server_part(self.as_str())
	}
}
impl OwnedRoomId {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		crate::server_part(self.as_str())
	}
}
impl OwnedRoomOrAliasId {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		crate::server_part(self.as_str())
	}
}
impl OwnedServerName {
	#[must_use]
	pub fn server_name(&self) -> Option<OwnedServerName> {
		crate::server_part(self.as_str())
	}
}
impl OwnedUserId {
	/// Builds a user ID from a localpart and an explicit server name.
	///
	/// # Errors
	///
	/// Returns an error if the resulting ID does not parse.
	pub fn parse_with_server_name(
		localpart: impl AsRef<str>,
		server_name: &OwnedServerName,
	) -> Result<Self, crate::MatrixIdParseError> {
		let localpart = localpart.as_ref();
		if localpart.starts_with('@') {
			return Self::parse(localpart);
		}
		Self::parse(&alloc::format!("@{localpart}:{}", server_name.as_str()))
	}

	#[must_use]
	pub fn server_name(&self) -> OwnedServerName {
		crate::server_part(self.as_str()).unwrap_or_else(|| OwnedServerName::from(""))
	}
}
impl OwnedRoomAliasId {
	#[must_use]
	pub fn server_name(&self) -> OwnedServerName {
		crate::server_part(self.as_str()).unwrap_or_else(|| OwnedServerName::from(""))
	}
}

impl fmt::Display for ContentDispositionType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Inline => "inline",
			Self::Attachment => "attachment",
		})
	}
}

impl fmt::Display for ContentDisposition {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.disposition)?;
		if let Some(filename) = &self.filename {
			write!(f, "; filename=\"{}\"", filename.replace('\\', "\\\\").replace('"', "\\\""))?;
		}
		Ok(())
	}
}

impl core::str::FromStr for ContentDisposition {
	type Err = ContentDispositionParseError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let mut parts = value.split(';');
		let disposition = match parts.next().map(str::trim) {
			Some(kind) if kind.eq_ignore_ascii_case("inline") => ContentDispositionType::Inline,
			Some(kind) if kind.eq_ignore_ascii_case("attachment") => {
				ContentDispositionType::Attachment
			}
			_ => return Err(ContentDispositionParseError),
		};
		let filename = parts.find_map(|part| {
			let (key, value) = part.split_once('=')?;
			key.trim().eq_ignore_ascii_case("filename").then(|| {
				let value = value.trim();
				value
					.strip_prefix('"')
					.and_then(|rest| rest.strip_suffix('"'))
					.unwrap_or(value)
					.replace("\\\"", "\"")
					.replace("\\\\", "\\")
			})
		});
		Ok(Self {
			disposition,
			filename,
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::http_headers::{ContentDisposition, ContentDispositionType};

	#[test]
	fn content_disposition_round_trips() {
		let value = ContentDisposition::new(ContentDispositionType::Attachment)
			.with_filename(Some("a \"b\".png".into()));
		let text = value.to_string();
		assert_eq!(text, "attachment; filename=\"a \\\"b\\\".png\"");
		assert_eq!(text.parse::<ContentDisposition>().unwrap(), value);
		assert_eq!(
			"INLINE".parse::<ContentDisposition>().unwrap(),
			ContentDisposition::new(ContentDispositionType::Inline)
		);
		assert!("download".parse::<ContentDisposition>().is_err());
	}
}

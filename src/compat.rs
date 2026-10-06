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

/// Gives each type a fixed `Display` message and an empty `std::error::Error` impl.
macro_rules! simple_error {
	($($t:ty => $msg:literal),* $(,)?) => {$(
		impl core::fmt::Display for $t {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				f.write_str($msg)
			}
		}
		impl core::error::Error for $t {}
	)*};
}
pub(crate) use simple_error;

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
	/// Whether this is a room ID (`!`-prefixed) rather than an alias.
	#[must_use]
	pub fn is_room_id(&self) -> bool {
		self.as_str().starts_with('!')
	}

	/// Whether this is a room alias (`#`-prefixed).
	#[must_use]
	pub fn is_room_alias_id(&self) -> bool {
		self.as_str().starts_with('#')
	}

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
	#[must_use]
	pub fn localpart(&self) -> &str {
		self.as_str()
			.strip_prefix('@')
			.unwrap_or(self.as_str())
			.split_once(':')
			.map_or(self.as_str(), |(local, _)| local)
	}
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
		Self::parse(alloc::format!("@{localpart}:{}", server_name.as_str()))
	}

	#[must_use]
	///
	/// # Panics
	///
	/// Panics if this value is not a valid user ID with a server name.
	pub fn server_name(&self) -> OwnedServerName {
		crate::server_part(self.as_str()).expect("validated user ID must have a server name")
	}
}
/// The part of an identifier between its sigil and the first `:`.
fn id_localpart(value: &str) -> &str {
	let body = value.get(1..).unwrap_or_default();
	body.split_once(':').map_or(body, |(local, _)| local)
}
impl OwnedRoomAliasId {
	/// The full alias as a string (`#localpart:server`), like Ruma's
	/// `RoomAliasId::alias`. Use [`Self::localpart`] for the part between the
	/// sigil and the server name.
	#[must_use]
	pub fn alias(&self) -> &str {
		self.as_str()
	}

	/// The part between the `#` sigil and the server name.
	#[must_use]
	pub fn localpart(&self) -> &str {
		id_localpart(self.as_str())
	}
}
impl OwnedRoomId {
	#[must_use]
	pub fn localpart(&self) -> &str {
		id_localpart(self.as_str())
	}
}
impl OwnedEventId {
	#[must_use]
	pub fn localpart(&self) -> &str {
		id_localpart(self.as_str())
	}
}
impl OwnedServerName {
	/// The host, without any port, and with IPv6 brackets kept.
	#[must_use]
	pub fn host(&self) -> &str {
		let name = self.as_str();
		if name.starts_with('[') {
			return name.find(']').and_then(|end| name.get(..=end)).unwrap_or(name);
		}
		name.rsplit_once(':').map_or(name, |(host, _)| host)
	}

	/// The port, if the server name carries one.
	#[must_use]
	pub fn port(&self) -> Option<u16> {
		let name = self.as_str();
		let rest = if name.starts_with('[') {
			name.split_once(']')?.1
		} else {
			name
		};
		rest.rsplit_once(':')?.1.parse().ok()
	}

	/// Whether the host is an IP literal rather than a DNS name.
	#[must_use]
	pub fn is_ip_literal(&self) -> bool {
		let host = self.host();
		host.starts_with('[') || host.parse::<core::net::Ipv4Addr>().is_ok()
	}
}
impl OwnedRoomAliasId {
	#[must_use]
	///
	/// # Panics
	///
	/// Panics if this value is not a valid room alias ID with a server name.
	pub fn server_name(&self) -> OwnedServerName {
		crate::server_part(self.as_str())
			.expect("validated room alias ID must have a server name")
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

/// Decodes the escapes of an HTTP quoted-string body: a backslash makes the next
/// character literal. Done in one pass so adjacent escapes cannot interfere.
fn unescape_quoted_string(quoted: &str) -> String {
	let mut out = String::with_capacity(quoted.len());
	let mut chars = quoted.chars();
	while let Some(c) = chars.next() {
		match c {
			'\\' => out.push(chars.next().unwrap_or('\\')),
			other => out.push(other),
		}
	}
	out
}

/// Splits header parameters on `;`, leaving `;` inside quoted strings alone.
fn split_header_params(value: &str) -> alloc::vec::Vec<&str> {
	let mut parts = alloc::vec::Vec::new();
	let (mut start, mut in_quotes, mut escaped) = (0, false, false);
	for (index, c) in value.char_indices() {
		match c {
			_ if escaped => escaped = false,
			'\\' if in_quotes => escaped = true,
			'"' => in_quotes = !in_quotes,
			';' if !in_quotes => {
				parts.push(&value[start..index]);
				start = index.saturating_add(1);
			}
			_ => {}
		}
	}
	parts.push(&value[start..]);
	parts
}

impl core::str::FromStr for ContentDisposition {
	type Err = ContentDispositionParseError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let mut parts = split_header_params(value).into_iter();
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
				match value.strip_prefix('"').and_then(|rest| rest.strip_suffix('"')) {
					Some(quoted) => unescape_quoted_string(quoted),
					None => value.to_owned(),
				}
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
	fn escaped_backslashes_and_quotes_unescape_in_one_pass() {
		let parse = |text: &str| -> Option<String> {
			text.parse::<ContentDisposition>().unwrap().filename
		};
		// filename="a\\b"  ->  a\b
		assert_eq!(parse("attachment; filename=\"a\\\\b\"").as_deref(), Some("a\\b"));
		// filename="x\\\"y"  ->  x\"y  (an escaped backslash, then an escaped quote)
		assert_eq!(parse("attachment; filename=\"x\\\\\\\"y\"").as_deref(), Some("x\\\"y"));
		// A trailing escaped backslash does not swallow the closing quote.
		assert_eq!(parse("attachment; filename=\"end\\\\\"; other=1").as_deref(), Some("end\\"));
	}

	#[test]
	fn semicolons_inside_a_quoted_filename_are_kept() {
		let parsed: ContentDisposition =
			"attachment; filename=\"name;with;semicolons\"".parse().unwrap();
		assert_eq!(parsed.filename.as_deref(), Some("name;with;semicolons"));

		let again: ContentDisposition = parsed.to_string().parse().unwrap();
		assert_eq!(again.filename.as_deref(), Some("name;with;semicolons"));

		let escaped: ContentDisposition =
			"inline; filename=\"a\\\";b\"; other=1".parse().unwrap();
		assert_eq!(escaped.filename.as_deref(), Some("a\";b"));
	}

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

#[cfg(test)]
mod room_alias_tests {
	use crate::OwnedRoomAliasId;

	#[test]
	fn alias_is_the_full_alias_like_ruma() {
		let alias = OwnedRoomAliasId::parse("#room:example.org").unwrap();
		assert_eq!(alias.alias(), "#room:example.org");
		assert_eq!(alias.localpart(), "room");
		assert_eq!(alias.server_name().as_str(), "example.org");
	}
}

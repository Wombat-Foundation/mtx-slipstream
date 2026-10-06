//! Grammar checks for Matrix identifiers.
//!
//! These follow the Matrix identifier grammar, including the restricted MXC
//! media-ID alphabet.

use core::net::{Ipv4Addr, Ipv6Addr};

/// Identifiers, including their sigil, are at most this many bytes.
const MAX_BYTES: usize = 255;

/// Accepts any string; used for identifiers that are opaque to the server.
#[must_use]
pub const fn any(_: &str) -> bool {
	true
}

/// `host[:port]` with a DNS name, IPv4 address or bracketed IPv6 address.
#[must_use]
pub fn server_name(value: &str) -> bool {
	if value.is_empty() || value.len() > MAX_BYTES {
		return false;
	}
	if let Some(rest) = value.strip_prefix('[') {
		let Some((address, tail)) = rest.split_once(']') else {
			return false;
		};
		return address.parse::<Ipv6Addr>().is_ok()
			&& (tail.is_empty() || tail.strip_prefix(':').is_some_and(port));
	}
	let (host, port_part) = match value.split_once(':') {
		Some((host, port_part)) => (host, Some(port_part)),
		None => (value, None),
	};
	port_part.is_none_or(port) && (host.parse::<Ipv4Addr>().is_ok() || dns_name(host))
}

fn port(value: &str) -> bool {
	!value.is_empty()
		&& value.len() <= 5
		&& value.bytes().all(|byte| byte.is_ascii_digit())
		&& value.parse::<u16>().is_ok_and(|port| port != 0)
}

fn dns_name(value: &str) -> bool {
	!value.is_empty()
		&& !value.starts_with('.')
		&& value.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'.')
}

/// `@localpart:server`.
#[must_use]
pub fn user_id(value: &str) -> bool {
	value.len() <= MAX_BYTES
		&& value.strip_prefix('@').and_then(|rest| rest.split_once(':')).is_some_and(
			|(local, server)| {
				// Historical grammar: printable ASCII only, no `:` (already split off).
				!local.is_empty()
					&& local.bytes().all(|b| (0x21..=0x7E).contains(&b))
					&& server_name(server)
			},
		)
}

/// Whether a user ID localpart is "fully conforming" to the current grammar
/// (`a-z`, `0-9`, `-`, `.`, `=`, `_`, `/`, `+`), as opposed to merely historical.
#[must_use]
pub fn user_localpart_is_fully_conforming(local: &str) -> bool {
	!local.is_empty()
		&& local.bytes().all(
			|b| matches!(b, b'0'..=b'9' | b'a'..=b'z' | b'-' | b'.' | b'=' | b'_' | b'/' | b'+'),
		)
}

/// `!opaque:server`, or `!hash` for room versions that omit the server.
#[must_use]
pub fn room_id(value: &str) -> bool {
	value.len() <= MAX_BYTES
		&& value.strip_prefix('!').is_some_and(|rest| match rest.split_once(':') {
			Some((opaque, server)) => !opaque.is_empty() && server_name(server),
			None => !rest.is_empty(),
		})
}

/// `$opaque:server` (early room versions) or `$hash`.
#[must_use]
pub fn event_id(value: &str) -> bool {
	value.len() <= MAX_BYTES
		&& value.strip_prefix('$').is_some_and(|rest| match rest.split_once(':') {
			Some((opaque, server)) => !opaque.is_empty() && server_name(server),
			None => !rest.is_empty(),
		})
}

/// `#alias:server`.
#[must_use]
pub fn room_alias_id(value: &str) -> bool {
	value.len() <= MAX_BYTES
		&& value
			.strip_prefix('#')
			.and_then(|rest| rest.split_once(':'))
			.is_some_and(|(alias, server)| !alias.is_empty() && server_name(server))
}

/// A room ID or a room alias.
#[must_use]
pub fn room_or_alias_id(value: &str) -> bool {
	room_id(value) || room_alias_id(value)
}

/// `mxc://server/media_id`.
#[must_use]
pub fn mxc_uri(value: &str) -> bool {
	value.strip_prefix("mxc://").is_some_and(|rest| {
		rest.split_once('/').is_some_and(|(server, media_id)| {
			server_name(server)
				&& !media_id.is_empty()
				&& media_id
					.bytes()
					.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
		})
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn server_names() {
		for good in [
			"example.org",
			"example.org:8448",
			"localhost",
			"1.2.3.4",
			"1.2.3.4:8448",
			"[::1]",
			"[2001:db8::1]:8448",
			"sub-domain.example.org",
		] {
			assert!(server_name(good), "{good}");
		}
		for bad in [
			"",
			"example.org:",
			"example.org:0",
			"example.org:65536",
			"example.org:abc",
			"exa mple.org",
			"a:b:c",
			"[::1",
			"[::1]x",
			"[nonsense]",
			".example.org",
			"exa/mple.org",
		] {
			assert!(!server_name(bad), "{bad}");
		}
		assert!(!server_name(&"a".repeat(256)));
	}

	#[test]
	fn user_ids() {
		assert!(user_id("@alice:example.org"));
		assert!(user_id("@Alice.Weird=chars/+:example.org:8448"));
		for bad in ["alice:example.org", "@alice", "@:example.org", "@alice:", "@a:b c", "!a:b"] {
			assert!(!user_id(bad), "{bad}");
		}
		let long = alloc::format!("@{}:example.org", "a".repeat(250));
		assert!(!user_id(&long));
	}

	#[test]
	fn room_event_and_alias_ids() {
		assert!(room_id("!abc:example.org"));
		assert!(room_id("!hashwithoutserver"));
		assert!(!room_id("abc:example.org"));
		assert!(!room_id("!"));
		assert!(!room_id("!:example.org"));
		assert!(!room_id("!abc:bad name"));

		assert!(event_id("$abc:example.org"));
		assert!(event_id("$Rqnc-F-dvnEYJTyHq_iKxU2bZ1CI92-kuZq3a5lr5Zg"));
		assert!(!event_id("abc"));
		assert!(!event_id("legacy-delay-id"));
		assert!(!event_id("$"));
		assert!(!event_id("$abc:"));

		assert!(room_alias_id("#room:example.org"));
		assert!(!room_alias_id("#room"));
		assert!(!room_alias_id("#:example.org"));
		assert!(room_or_alias_id("#room:example.org"));
		assert!(room_or_alias_id("!abc:example.org"));
		assert!(!room_or_alias_id("@a:example.org"));
	}

	#[test]
	fn mxc_uris() {
		assert!(mxc_uri("mxc://example.org/abcDEF123"));
		assert!(mxc_uri("mxc://example.org/Something"));
		for bad in [
			"mxc://example.org",
			"mxc://example.org/",
			"mxc:///id",
			"http://a/b",
			"mxc://a/b/c",
			"mxc://example.org/with.dot",
		] {
			assert!(!mxc_uri(bad), "{bad}");
		}
	}

	#[test]
	fn trusted_construction_bypasses_parse_but_try_from_validates() {
		use crate::OwnedUserId;
		assert!(OwnedUserId::parse("@a:example.org").is_ok());
		assert!(OwnedUserId::parse("nonsense").is_err());
		assert!("nonsense".parse::<OwnedUserId>().is_err());
		assert_eq!(OwnedUserId::from_trusted("nonsense").as_str(), "nonsense");
	}

	#[test]
	fn codec_decode_enforces_grammar() {
		use crate::{OwnedUserId, codec::from_str};
		assert!(from_str::<OwnedUserId>("\"nonsense\"").is_err());
		assert!(from_str::<OwnedUserId>("\"@a:example.org\"").is_ok());
	}
}

#[cfg(test)]
mod user_id_grammar_tests {
	use crate::OwnedUserId;

	#[test]
	fn parse_rejects_invalid_historical_localparts() {
		for bad in
			["@user name:example.org", "@üser:example.org", "@:example.org", "@a\tb:example.org"]
		{
			assert!(OwnedUserId::parse(bad).is_err(), "{bad:?} must not parse");
		}
		assert!(OwnedUserId::parse("@Alice!:example.org").is_ok(), "historical IDs still parse");
	}

	#[test]
	fn validate_strict_requires_the_current_grammar() {
		let ok = OwnedUserId::parse("@alice_1.x=y/z+w-v:example.org").unwrap();
		assert!(ok.validate_strict().is_ok());
		for historical in ["@Alice:example.org", "@alice!:example.org", "@al@ice:example.org"] {
			let id = OwnedUserId::parse(historical).unwrap();
			assert!(id.validate_strict().is_err(), "{historical} is historical, not strict");
		}
	}
}

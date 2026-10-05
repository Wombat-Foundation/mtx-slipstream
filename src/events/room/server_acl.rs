use alloc::{string::String, vec::Vec};

/// `m.room.server_acl` content.
#[derive(Clone, Debug)]
pub struct RoomServerAclEventContent {
	pub allow_ip_literals: bool,
	pub allow: Vec<String>,
	pub deny: Vec<String>,
}

impl Default for RoomServerAclEventContent {
	fn default() -> Self {
		Self {
			allow_ip_literals: true,
			allow: Vec::new(),
			deny: Vec::new(),
		}
	}
}

impl RoomServerAclEventContent {
	#[must_use]
	pub fn new(allow_ip_literals: bool, allow: Vec<String>, deny: Vec<String>) -> Self {
		Self {
			allow_ip_literals,
			allow,
			deny,
		}
	}

	/// Whether `server_name` is permitted: IP literals are rejected unless
	/// allowed, then `deny` globs reject and `allow` globs accept.
	#[must_use]
	pub fn is_allowed(&self, server_name: &crate::OwnedServerName) -> bool {
		if !self.allow_ip_literals && server_name.is_ip_literal() {
			return false;
		}
		let host = server_name.host();
		!self.deny_matches(host) && self.allow_matches(host)
	}

	/// Whether `host` matches a glob in the allow list; hosts compare
	/// case-insensitively.
	#[must_use]
	pub fn allow_matches(&self, host: &str) -> bool {
		matches_any(&self.allow, host)
	}

	/// Whether `host` matches a glob in the deny list; hosts compare
	/// case-insensitively.
	#[must_use]
	pub fn deny_matches(&self, host: &str) -> bool {
		matches_any(&self.deny, host)
	}

	/// Whether `host` equals (ignoring case) an entry of the allow list.
	#[must_use]
	pub fn allow_contains(&self, host: &str) -> bool {
		contains_any(&self.allow, host)
	}

	/// Whether `host` equals (ignoring case) an entry of the deny list.
	#[must_use]
	pub fn deny_contains(&self, host: &str) -> bool {
		contains_any(&self.deny, host)
	}

	#[must_use]
	pub fn allow_is_empty(&self) -> bool {
		self.allow.is_empty()
	}

	#[must_use]
	pub fn deny_is_empty(&self) -> bool {
		self.deny.is_empty()
	}
}

fn matches_any(patterns: &[String], host: &str) -> bool {
	let host = host.to_lowercase();
	patterns.iter().any(|pattern| glob_match(&pattern.to_lowercase(), &host))
}

fn contains_any(entries: &[String], host: &str) -> bool {
	let host = host.to_lowercase();
	entries.iter().any(|entry| entry.to_lowercase() == host)
}

/// Matches `*` (any run) and `?` (any one character).
///
/// Indices only ever advance, and each backtrack moves the text mark forward,
/// so the loop terminates in at most `pattern.len() * text.len()` steps.
fn glob_match(pattern: &str, text: &str) -> bool {
	let pattern: Vec<char> = pattern.chars().collect();
	let text: Vec<char> = text.chars().collect();
	let (mut p, mut t) = (0_usize, 0_usize);
	// Pattern index just after the last `*`, and the text index it matches up to.
	let mut backtrack: Option<(usize, usize)> = None;
	while t < text.len() {
		let step = match (pattern.get(p).copied(), text.get(t).copied()) {
			(Some('?'), Some(_)) => true,
			(Some(pc), Some(tc)) if pc == tc => true,
			(Some('*'), _) => {
				p = p.saturating_add(1);
				backtrack = Some((p, t));
				continue;
			}
			_ => false,
		};
		if step {
			p = p.saturating_add(1);
			t = t.saturating_add(1);
		} else if let Some((resume, mark)) = backtrack {
			let mark = mark.saturating_add(1);
			backtrack = Some((resume, mark));
			p = resume;
			t = mark;
		} else {
			return false;
		}
	}
	pattern.get(p..).is_some_and(|rest| rest.iter().all(|&c| c == '*'))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn acl_globs_and_ip_literals() {
		let acl = RoomServerAclEventContent::new(
			false,
			alloc::vec!["*".into()],
			alloc::vec!["*.evil.org".into(), "bad?.net".into()],
		);
		assert!(acl.is_allowed(&crate::OwnedServerName::from("good.org:8448")));
		assert!(!acl.is_allowed(&crate::OwnedServerName::from("a.evil.org")));
		assert!(!acl.is_allowed(&crate::OwnedServerName::from("bad1.net")));
		assert!(!acl.is_allowed(&crate::OwnedServerName::from("1.2.3.4:8448")));
		assert!(!acl.is_allowed(&crate::OwnedServerName::from("[::1]:8448")));
		assert!(!acl.is_allowed(&crate::OwnedServerName::from("bad1.net:8448")));
	}

	#[test]
	fn hosts_and_patterns_compare_case_insensitively() {
		let acl = RoomServerAclEventContent::new(
			true,
			alloc::vec!["*".into()],
			alloc::vec!["*.Evil.ORG".into()],
		);
		assert!(!acl.is_allowed(&crate::OwnedServerName::from("A.EVIL.org")));
		assert!(acl.deny_matches("a.evil.org"));
		assert!(acl.deny_contains("*.evil.org"));
		assert!(acl.allow_contains("*"));
		assert!(!acl.allow_is_empty());
		assert!(RoomServerAclEventContent::default().allow_is_empty());
	}

	#[test]
	fn glob_edges() {
		assert!(glob_match("", ""));
		assert!(glob_match("*", ""));
		assert!(glob_match("a*b*c", "aXXbYYc"));
		assert!(!glob_match("a*b*c", "aXXbYY"));
		assert!(glob_match("*.x.org", "a.b.x.org"));
		assert!(!glob_match("?", ""));
		assert!(glob_match("a**", "a"));
		assert!(glob_match("*a*a*a", "aaaa"));
		assert!(!glob_match("abc", "abcd"));
		assert!(glob_match("[::1]", "[::1]"));
	}

	#[test]
	fn allow_ip_literals_permits_them_when_globs_match() {
		let acl = RoomServerAclEventContent::new(true, alloc::vec!["*".into()], Vec::new());
		assert!(acl.is_allowed(&crate::OwnedServerName::from("1.2.3.4:8448")));
		assert!(acl.is_allowed(&crate::OwnedServerName::from("[::1]:8448")));
	}
}

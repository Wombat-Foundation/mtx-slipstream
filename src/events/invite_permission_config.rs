//! `m.invite_permission_config` (MSC4155).

use alloc::{string::String, vec::Vec};

use crate::UserId;

/// How an invite from a user is treated.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FilterLevel {
	/// The invite is delivered.
	#[default]
	Allow,
	/// The invite is delivered but hidden from the recipient.
	Ignore,
	/// The invite is rejected.
	Block,
}

/// A user's invite filtering preferences.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct InvitePermissionConfigEventContent {
	pub enabled: bool,
	pub blocked_users: Vec<String>,
	pub blocked_servers: Vec<String>,
	pub allowed_users: Vec<String>,
	pub allowed_servers: Vec<String>,
	pub ignored_users: Vec<String>,
	pub ignored_servers: Vec<String>,
}

crate::impl_codec_struct!(InvitePermissionConfigEventContent {}
	default {
		enabled: bool,
		blocked_users: Vec<String>,
		blocked_servers: Vec<String>,
		allowed_users: Vec<String>,
		allowed_servers: Vec<String>,
		ignored_users: Vec<String>,
		ignored_servers: Vec<String>,
	}
);

/// Matches `target` against a glob in which `*` matches any run and `?` one character.
fn glob_match(glob: &str, target: &str) -> bool {
	fn go(glob: &[char], target: &[char]) -> bool {
		match glob.split_first() {
			None => target.is_empty(),
			Some(('*', rest)) => (0..=target.len()).any(|skip| go(rest, &target[skip..])),
			Some(('?', rest)) => target.split_first().is_some_and(|(_, tail)| go(rest, tail)),
			Some((c, rest)) => {
				target.split_first().is_some_and(|(t, tail)| t == c && go(rest, tail))
			}
		}
	}
	let glob: Vec<char> = glob.chars().collect();
	let target: Vec<char> = target.chars().collect();
	go(&glob, &target)
}

impl InvitePermissionConfigEventContent {
	/// The level for invites from `sender`, from the ignore lists alone.
	#[must_use]
	pub fn user_filter_level(&self, sender: &UserId) -> FilterLevel {
		let user = sender.as_str();
		let server = sender.server_name();
		let ignored = self.ignored_users.iter().any(|glob| glob_match(glob, user))
			|| server.as_ref().is_some_and(|server| {
				self.ignored_servers.iter().any(|glob| glob_match(glob, server.as_str()))
			});
		if ignored {
			FilterLevel::Ignore
		} else {
			FilterLevel::Allow
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{OwnedUserId, codec::from_str};

	#[test]
	fn ignore_lists_use_globs() {
		let config: InvitePermissionConfigEventContent = from_str(
			r#"{"enabled":true,"ignored_users":["@spam*:*"],"ignored_servers":["evil.org"]}"#,
		)
		.unwrap();
		assert_eq!(
			config.user_filter_level(&OwnedUserId::parse("@spam1:ok.org").unwrap()),
			FilterLevel::Ignore
		);
		assert_eq!(
			config.user_filter_level(&OwnedUserId::parse("@bob:evil.org").unwrap()),
			FilterLevel::Ignore
		);
		assert_eq!(
			config.user_filter_level(&OwnedUserId::parse("@bob:ok.org").unwrap()),
			FilterLevel::Allow
		);
	}
}

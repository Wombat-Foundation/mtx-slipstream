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

impl crate::codec::Serialize for InvitePermissionConfigEventContent {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			("enabled", crate::endpoint::enc(&self.enabled)),
			("blocked_users", crate::endpoint::enc(&self.blocked_users)),
			("blocked_servers", crate::endpoint::enc(&self.blocked_servers)),
			("allowed_users", crate::endpoint::enc(&self.allowed_users)),
			("allowed_servers", crate::endpoint::enc(&self.allowed_servers)),
			("ignored_users", crate::endpoint::enc(&self.ignored_users)),
			("ignored_servers", crate::endpoint::enc(&self.ignored_servers)),
		])
	}
}

impl crate::codec::Deserialize for InvitePermissionConfigEventContent {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		let object = value.as_object().ok_or_else(|| {
			crate::codec::DeError::expected("InvitePermissionConfigEventContent")
		})?;
		let decode =
			|name: &str| object.get(name).map(crate::codec::Deserialize::from_json).transpose();
		Ok(Self {
			// MSC4155 treats an omitted enabled flag as enabled.
			enabled: object
				.get("enabled")
				.map(crate::codec::Deserialize::from_json)
				.transpose()?
				.unwrap_or(true),
			blocked_users: decode("blocked_users")?.unwrap_or_default(),
			blocked_servers: decode("blocked_servers")?.unwrap_or_default(),
			allowed_users: decode("allowed_users")?.unwrap_or_default(),
			allowed_servers: decode("allowed_servers")?.unwrap_or_default(),
			ignored_users: decode("ignored_users")?.unwrap_or_default(),
			ignored_servers: decode("ignored_servers")?.unwrap_or_default(),
		})
	}
}

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
			|| self.ignored_servers.iter().any(|glob| glob_match(glob, server.as_str()));
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

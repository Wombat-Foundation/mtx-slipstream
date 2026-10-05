//! Codec impls for room event content types.

use crate::{
	codec::{DeError, Deserialize, Serialize, from_value},
	events::room::{
		avatar::RoomAvatarEventContent,
		canonical_alias::RoomCanonicalAliasEventContent,
		encryption::RoomEncryptionEventContent,
		guest_access::RoomGuestAccessEventContent,
		history_visibility::RoomHistoryVisibilityEventContent,
		join_rules::{
			AllowRule, JoinRule, RestrictedRule, RoomJoinRulesEventContent, RoomMembership,
		},
		member::{RoomMemberEventContent, ThirdPartyInvite},
		name::RoomNameEventContent,
		policy::RoomPolicyEventContent,
		power_levels::RoomPowerLevelsEventContent,
		preview_url::RoomPreviewUrlsEventContent,
		redaction::RoomRedactionEventContent,
		server_acl::RoomServerAclEventContent,
		third_party_invite::RoomThirdPartyInviteEventContent,
		topic::RoomTopicEventContent,
	},
	json::{Object, Value},
	power_levels::NotificationPowerLevels,
	serde::deserialize_v1_powerlevel,
};

impl Serialize for RoomGuestAccessEventContent {
	fn to_json(&self) -> Value {
		Value::Object(
			[(String::from("guest_access"), self.guest_access.to_json())].into_iter().collect(),
		)
	}
}
impl Serialize for RoomHistoryVisibilityEventContent {
	fn to_json(&self) -> Value {
		Value::Object(
			[(String::from("history_visibility"), self.history_visibility.to_json())]
				.into_iter()
				.collect(),
		)
	}
}
impl Deserialize for RoomHistoryVisibilityEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = object(value)?;
		Ok(Self {
			history_visibility: field(object, "history_visibility")?.unwrap_or_default(),
		})
	}
}
impl Serialize for RoomNameEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Some(name) = &self.name {
			object.insert("name".into(), Value::String(name.clone()));
		}
		Value::Object(object)
	}
}
impl Deserialize for RoomNameEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			name: field(object(value)?, "name")?,
		})
	}
}
impl Deserialize for RoomTopicEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			topic: field(object(value)?, "topic")?.unwrap_or_default(),
		})
	}
}
impl Serialize for RoomAvatarEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Some(url) = &self.url {
			insert(&mut object, "url", url);
		}
		Value::Object(object)
	}
}
impl Deserialize for RoomAvatarEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			url: field(object(value)?, "url")?,
		})
	}
}
impl Serialize for RoomEncryptionEventContent {
	fn to_json(&self) -> Value {
		Value::Object(
			[(String::from("algorithm"), self.algorithm.to_json())].into_iter().collect(),
		)
	}
}
impl Deserialize for RoomEncryptionEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			algorithm: field(object(value)?, "algorithm")?
				.ok_or_else(|| DeError::expected("algorithm"))?,
		})
	}
}
impl Deserialize for RoomGuestAccessEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			guest_access: field(object(value)?, "guest_access")?.unwrap_or_default(),
		})
	}
}
impl Serialize for RoomServerAclEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		insert(&mut object, "allow_ip_literals", &self.allow_ip_literals);
		insert(&mut object, "allow", &self.allow);
		insert(&mut object, "deny", &self.deny);
		Value::Object(object)
	}
}
impl Deserialize for RoomServerAclEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = object(value)?;
		Ok(Self {
			allow_ip_literals: field(object, "allow_ip_literals")?.unwrap_or(true),
			allow: field(object, "allow")?.unwrap_or_default(),
			deny: field(object, "deny")?.unwrap_or_default(),
		})
	}
}
impl Serialize for RoomPolicyEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Some(via) = &self.via {
			insert(&mut object, "via", via);
		}
		if let Some(public_key) = &self.public_key {
			insert(&mut object, "public_key", public_key);
		}
		Value::Object(object)
	}
}
impl Deserialize for RoomPolicyEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = object(value)?;
		Ok(Self {
			via: field(object, "via")?,
			public_key: field(object, "public_key")?,
		})
	}
}
impl Serialize for RoomTopicEventContent {
	fn to_json(&self) -> Value {
		Value::Object(
			[(String::from("topic"), Value::String(self.topic.clone()))].into_iter().collect(),
		)
	}
}

fn object(value: &Value) -> Result<&Object, DeError> {
	value.as_object().ok_or_else(|| DeError::expected("object"))
}

fn field<T: Deserialize>(object: &Object, name: &str) -> Result<Option<T>, DeError> {
	object.get(name).filter(|v| !v.is_null()).map(from_value).transpose()
}

fn level(object: &Object, name: &str, default: crate::Int) -> Result<crate::Int, DeError> {
	// A present `null` is an error, as with the serde field it replaces.
	object.get(name).map_or(Ok(default), deserialize_v1_powerlevel)
}

fn insert<T: Serialize>(object: &mut Object, name: &str, value: &T) {
	object.insert(name.into(), value.to_json());
}

impl Serialize for RoomCanonicalAliasEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Some(alias) = &self.alias {
			insert(&mut object, "alias", alias);
		}
		if !self.alt_aliases.is_empty() {
			insert(&mut object, "alt_aliases", &self.alt_aliases);
		}
		Value::Object(object)
	}
}
impl Deserialize for RoomCanonicalAliasEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = object(value)?;
		Ok(Self {
			alias: field(object, "alias")?,
			alt_aliases: field(object, "alt_aliases")?.unwrap_or_default(),
		})
	}
}
impl Serialize for RoomPreviewUrlsEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		insert(&mut object, "disabled", &self.disabled);
		Value::Object(object)
	}
}
impl Deserialize for RoomPreviewUrlsEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = object(value)?;
		Ok(Self {
			disabled: field(object, "disabled")?.unwrap_or(true),
		})
	}
}

impl Serialize for NotificationPowerLevels {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		insert(&mut o, "room", &self.room);
		Value::Object(o)
	}
}
impl Deserialize for NotificationPowerLevels {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			room: level(object(value)?, "room", 50)?,
		})
	}
}

fn level_map<K: Deserialize + Ord>(
	object: &Object,
	name: &str,
) -> Result<alloc::collections::BTreeMap<K, crate::Int>, DeError> {
	object.get(name).map_or_else(
		|| Ok(alloc::collections::BTreeMap::new()),
		|v| {
			object_of(v)?
				.iter()
				.map(|(k, v)| {
					Ok((K::from_json(&Value::String(k.clone()))?, deserialize_v1_powerlevel(v)?))
				})
				.collect()
		},
	)
}

fn object_of(value: &Value) -> Result<&Object, DeError> {
	object(value)
}

impl Serialize for RoomPowerLevelsEventContent {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		insert(&mut o, "ban", &self.ban);
		insert(&mut o, "events", &self.events);
		insert(&mut o, "events_default", &self.events_default);
		insert(&mut o, "invite", &self.invite);
		insert(&mut o, "kick", &self.kick);
		insert(&mut o, "redact", &self.redact);
		insert(&mut o, "state_default", &self.state_default);
		insert(&mut o, "users", &self.users);
		insert(&mut o, "users_default", &self.users_default);
		insert(&mut o, "notifications", &self.notifications);
		Value::Object(o)
	}
}
impl Deserialize for RoomPowerLevelsEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let o = object(value)?;
		Ok(Self {
			ban: level(o, "ban", 50)?,
			events: level_map(o, "events")?,
			events_default: level(o, "events_default", 0)?,
			invite: level(o, "invite", 0)?,
			kick: level(o, "kick", 50)?,
			redact: level(o, "redact", 50)?,
			state_default: level(o, "state_default", 50)?,
			users: level_map(o, "users")?,
			users_default: level(o, "users_default", 0)?,
			notifications: o.get("notifications").map_or_else(|| Ok(Default::default()), from_value)?,
		})
	}
}

impl Serialize for ThirdPartyInvite {
	fn to_json(&self) -> Value {
		let mut signed = Object::new();
		insert(&mut signed, "mxid", &self.signed.mxid);
		insert(&mut signed, "signatures", &self.signed.signatures);
		insert(&mut signed, "token", &self.signed.token);
		let mut o = Object::new();
		insert(&mut o, "display_name", &self.display_name);
		o.insert("signed".into(), Value::Object(signed));
		Value::Object(o)
	}
}

impl Serialize for RoomMemberEventContent {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		insert(&mut o, "membership", &self.membership);
		let optional: [(&str, Value); 8] = [
			("displayname", self.displayname.to_json()),
			("avatar_url", self.avatar_url.to_json()),
			("xyz.amorgan.blurhash", self.blurhash.to_json()),
			("reason", self.reason.to_json()),
			("is_direct", self.is_direct.to_json()),
			("third_party_invite", self.third_party_invite.to_json()),
			("org.matrix.msc4293.redact_events", self.redact_events.to_json()),
			("join_authorised_via_users_server", self.join_authorized_via_users_server.to_json()),
		];
		for (name, value) in optional {
			if !value.is_null() {
				o.insert(name.into(), value);
			}
		}
		Value::Object(o)
	}
}
impl Deserialize for RoomMemberEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let o = object(value)?;
		Ok(Self {
			membership: field(o, "membership")?.ok_or_else(|| DeError::expected("membership"))?,
			displayname: field(o, "displayname")?,
			avatar_url: match o.get("avatar_url") {
				// `compat-empty-string-null` in the old build: "" meant no avatar.
				Some(Value::String(url)) if url.is_empty() => None,
				_ => field(o, "avatar_url")?,
			},
			blurhash: field(o, "xyz.amorgan.blurhash")?,
			reason: field(o, "reason")?,
			is_direct: field(o, "is_direct")?,
			third_party_invite: field(o, "third_party_invite")?,
			redact_events: field(o, "org.matrix.msc4293.redact_events")?,
			join_authorized_via_users_server: field(o, "join_authorised_via_users_server")?,
		})
	}
}

impl Serialize for RoomRedactionEventContent {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		if let Some(redacts) = &self.redacts {
			insert(&mut o, "redacts", redacts);
		}
		Value::Object(o)
	}
}
impl Deserialize for RoomRedactionEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self {
			redacts: field(object(value)?, "redacts")?,
		})
	}
}

impl Serialize for RoomThirdPartyInviteEventContent {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		if let Some(key) = &self.public_key {
			insert(&mut o, "public_key", key);
		}
		if let Some(keys) = &self.public_keys {
			insert(&mut o, "public_keys", keys);
		}
		Value::Object(o)
	}
}
impl Deserialize for RoomThirdPartyInviteEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let o = object(value)?;
		Ok(Self {
			public_key: field(o, "public_key")?,
			public_keys: field(o, "public_keys")?,
		})
	}
}

impl Serialize for AllowRule {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		match self {
			Self::RoomMembership(m) => {
				o.insert("type".into(), Value::String("m.room_membership".into()));
				o.insert("room_id".into(), m.room_id.to_json());
			}
			Self::UnstableSpamChecker => {
				o.insert("type".into(), Value::String("fi.mau.spam_checker".into()));
			}
			Self::_Custom(value) => return value.clone(),
		}
		Value::Object(o)
	}
}
impl Deserialize for AllowRule {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let o = object(value)?;
		let kind: alloc::string::String =
			field(o, "type")?.ok_or_else(|| DeError::expected("type"))?;
		Ok(match kind.as_str() {
			"m.room_membership" => Self::RoomMembership(RoomMembership {
				room_id: field(o, "room_id")?.ok_or_else(|| DeError::expected("room_id"))?,
			}),
			"fi.mau.spam_checker" => Self::UnstableSpamChecker,
			_ => Self::_Custom(value.clone()),
		})
	}
}

impl Serialize for RoomJoinRulesEventContent {
	fn to_json(&self) -> Value {
		let (rule, allow) = match &self.join_rule {
			JoinRule::Public => ("public", None),
			JoinRule::Knock => ("knock", None),
			JoinRule::Invite => ("invite", None),
			JoinRule::Private => ("private", None),
			JoinRule::Restricted(r) => ("restricted", Some(r)),
			JoinRule::KnockRestricted(r) => ("knock_restricted", Some(r)),
		};
		let mut o = Object::new();
		o.insert("join_rule".into(), Value::String(rule.into()));
		if let Some(r) = allow {
			o.insert("allow".into(), r.allow.to_json());
		}
		Value::Object(o)
	}
}
impl Deserialize for RoomJoinRulesEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let o = object(value)?;
		let rule: alloc::string::String =
			field(o, "join_rule")?.ok_or_else(|| DeError::expected("join_rule"))?;
		let allow = || -> Result<RestrictedRule, DeError> {
			Ok(RestrictedRule {
				allow: o
					.get("allow")
					.map_or(Ok(alloc::vec::Vec::new()), Deserialize::from_json)?,
			})
		};
		let join_rule = match rule.as_str() {
			"public" => JoinRule::Public,
			"knock" => JoinRule::Knock,
			"invite" => JoinRule::Invite,
			"private" => JoinRule::Private,
			"restricted" => JoinRule::Restricted(allow()?),
			"knock_restricted" => JoinRule::KnockRestricted(allow()?),
			_ => return Err(DeError::expected("join rule")),
		};
		Ok(Self {
			join_rule,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn power_levels_accept_string_levels_and_default() {
		let c = from_str::<RoomPowerLevelsEventContent>(
			r#"{"users":{"@a:b":"100"},"events":{"m.room.name":50},"ban":"70"}"#,
		)
		.unwrap();
		assert_eq!(c.ban, 70);
		assert_eq!(c.kick, 50);
		assert_eq!(c.users[&crate::OwnedUserId::from("@a:b")], 100);
		let back = from_str::<RoomPowerLevelsEventContent>(&to_string(&c)).unwrap();
		assert_eq!(back.users, c.users);
		assert_eq!(back.events, c.events);
	}

	#[test]
	fn join_rules_round_trip_restricted() {
		let c = from_str::<RoomJoinRulesEventContent>(
			r#"{"join_rule":"restricted","allow":[{"type":"m.room_membership","room_id":"!r:b"}]}"#,
		)
		.unwrap();
		let JoinRule::Restricted(r) = &c.join_rule else {
			panic!("not restricted")
		};
		assert_eq!(r.allow.len(), 1);
		assert!(to_string(&c).contains("restricted"));
	}

	#[test]
	fn member_requires_membership() {
		assert!(from_str::<RoomMemberEventContent>(r#"{"membership":"join"}"#).is_ok());
		assert!(from_str::<RoomMemberEventContent>("{}").is_err());
	}
}

#[cfg(test)]
mod member_tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn member_content_round_trips_all_fields() {
		let text = r#"{"membership":"invite","displayname":"A","avatar_url":"mxc://x/y","reason":"hi","is_direct":true,"third_party_invite":{"display_name":"A","signed":{"mxid":"@a:x","signatures":{"x":{"ed25519:1":"s"}},"token":"t"}},"org.matrix.msc4293.redact_events":false}"#;
		let content = from_str::<RoomMemberEventContent>(text).unwrap();
		assert_eq!(content.reason.as_deref(), Some("hi"));
		assert_eq!(content.redact_events, Some(false));
		let again = from_str::<RoomMemberEventContent>(&to_string(&content)).unwrap();
		assert_eq!(again.displayname.as_deref(), Some("A"));
		assert!(again.third_party_invite.is_some());
		assert!(!to_string(&RoomMemberEventContent::default()).contains("reason"));
	}

	#[test]
	fn power_levels_golden_default_output() {
		assert_eq!(
			to_string(&RoomPowerLevelsEventContent::new()),
			r#"{"ban":50,"events":{},"events_default":0,"invite":0,"kick":50,"notifications":{"room":50},"redact":50,"state_default":50,"users":{},"users_default":0}"#
		);
	}

	#[test]
	fn power_levels_golden_round_trip_keeps_typical_payload() {
		let text = r#"{"ban":50,"events":{"m.room.name":50,"m.room.power_levels":100},"events_default":0,"invite":0,"kick":50,"notifications":{"room":50},"redact":50,"state_default":50,"users":{"@a:b":100},"users_default":0}"#;
		let content: RoomPowerLevelsEventContent = from_str(text).unwrap();
		assert_eq!(to_string(&content), text);
	}

	#[test]
	fn power_levels_reject_null_and_out_of_range_like_serde() {
		for body in [
			r#"{"ban":null}"#,
			r#"{"users":null}"#,
			r#"{"events":null}"#,
			r#"{"notifications":null}"#,
			r#"{"ban":9007199254740992}"#,
			r#"{"ban":"fifty"}"#,
		] {
			assert!(from_str::<RoomPowerLevelsEventContent>(body).is_err(), "{body}");
		}
		let padded: RoomPowerLevelsEventContent = from_str(r#"{"ban":" 70 ","kick":"+5"}"#).unwrap();
		assert_eq!((padded.ban, padded.kick), (70, 5));
	}

	#[test]
	fn member_golden_minimal_and_full_output() {
		let minimal: RoomMemberEventContent = from_str(r#"{"membership":"join"}"#).unwrap();
		assert_eq!(to_string(&minimal), r#"{"membership":"join"}"#);

		let text = r#"{"avatar_url":"mxc://a/b","displayname":"D","is_direct":true,"join_authorised_via_users_server":"@u:s","membership":"invite","org.matrix.msc4293.redact_events":false,"reason":"r","third_party_invite":{"display_name":"Bob","signed":{"mxid":"@b:s","signatures":{"s":{"ed25519:1":"sig"}},"token":"t"}},"xyz.amorgan.blurhash":"h"}"#;
		let full: RoomMemberEventContent = from_str(text).unwrap();
		assert_eq!(to_string(&full), text);
	}

	#[test]
	fn member_null_empty_and_unknown_values() {
		let nulls: RoomMemberEventContent = from_str(
			r#"{"membership":"leave","displayname":null,"avatar_url":null,"reason":null,"is_direct":null}"#,
		)
		.unwrap();
		assert_eq!(to_string(&nulls), r#"{"membership":"leave"}"#);
		let empty_avatar: RoomMemberEventContent =
			from_str(r#"{"membership":"join","avatar_url":""}"#).unwrap();
		assert!(empty_avatar.avatar_url.is_none());
		for body in [
			r#"{"membership":null}"#,
			r#"{}"#,
			// The closed set of membership values: unknown strings are rejected.
			r#"{"membership":"custom"}"#,
			r#"{"membership":"join","third_party_invite":{"signed":{"mxid":"@b:s","token":"t"}}}"#,
		] {
			assert!(from_str::<RoomMemberEventContent>(body).is_err(), "{body}");
		}
	}
}

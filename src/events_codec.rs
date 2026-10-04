//! Codec impls for room event content types.

use crate::{
	codec::{DeError, Deserialize, Serialize, from_value},
	events::room::{
		canonical_alias::RoomCanonicalAliasEventContent,
		guest_access::RoomGuestAccessEventContent,
		history_visibility::RoomHistoryVisibilityEventContent,
		join_rules::{JoinRule, RestrictedRule, RoomJoinRulesEventContent},
		member::{RoomMemberEventContent, ThirdPartyInvite},
		message::RoomMessageEventContent,
		name::RoomNameEventContent,
		power_levels::RoomPowerLevelsEventContent,
		preview_url::RoomPreviewUrlsEventContent,
		redaction::RoomRedactionEventContent,
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
impl Serialize for RoomNameEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		if let Some(name) = &self.name {
			object.insert("name".into(), Value::String(name.clone()));
		}
		Value::Object(object)
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
	object.get(name).filter(|v| !v.is_null()).map_or(Ok(default), deserialize_v1_powerlevel)
}

fn insert<T: Serialize>(object: &mut Object, name: &str, value: &T) {
	object.insert(name.into(), value.to_json());
}

impl Serialize for RoomMessageEventContent {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("body".into(), Value::String(self.body.clone()));
		Value::Object(object)
	}
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

impl Deserialize for RoomMessageEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let body = object(value)?
			.get("body")
			.and_then(Value::as_str)
			.map(str::to_owned)
			.ok_or_else(|| DeError::expected("body"))?;
		Ok(Self::text_plain(body))
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
	object.get(name).filter(|v| !v.is_null()).map_or_else(
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
			notifications: field(o, "notifications")?.unwrap_or_default(),
		})
	}
}

impl Serialize for ThirdPartyInvite {
	fn to_json(&self) -> Value {
		let mut signed = Object::new();
		insert(&mut signed, "mxid", &self.signed.mxid);
		insert(&mut signed, "token", &self.signed.token);
		let mut o = Object::new();
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
			("blurhash", self.blurhash.to_json()),
			("reason", self.reason.to_json()),
			("is_direct", self.is_direct.to_json()),
			("third_party_invite", self.third_party_invite.to_json()),
			("redact_events", self.redact_events.to_json()),
			("join_authorized_via_users_server", self.join_authorized_via_users_server.to_json()),
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
			avatar_url: field(o, "avatar_url")?,
			blurhash: field(o, "blurhash")?,
			reason: field(o, "reason")?,
			is_direct: field(o, "is_direct")?,
			third_party_invite: field(o, "third_party_invite")?,
			redact_events: field(o, "redact_events")?,
			join_authorized_via_users_server: field(o, "join_authorized_via_users_server")?,
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
			o.insert("allow".into(), Value::Array(r.allow.clone()));
		}
		Value::Object(o)
	}
}
impl Deserialize for RoomJoinRulesEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let o = object(value)?;
		let rule: alloc::string::String =
			field(o, "join_rule")?.ok_or_else(|| DeError::expected("join_rule"))?;
		let allow = || RestrictedRule {
			allow: o.get("allow").and_then(Value::as_array).cloned().unwrap_or_default(),
		};
		let join_rule = match rule.as_str() {
			"public" => JoinRule::Public,
			"knock" => JoinRule::Knock,
			"invite" => JoinRule::Invite,
			"private" => JoinRule::Private,
			"restricted" => JoinRule::Restricted(allow()),
			"knock_restricted" => JoinRule::KnockRestricted(allow()),
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
		let text = r#"{"membership":"invite","displayname":"A","avatar_url":"mxc://x/y","reason":"hi","is_direct":true,"third_party_invite":{"signed":{"mxid":"@a:x","token":"t"}},"redact_events":false}"#;
		let content = from_str::<RoomMemberEventContent>(text).unwrap();
		assert_eq!(content.reason.as_deref(), Some("hi"));
		assert_eq!(content.redact_events, Some(false));
		let again = from_str::<RoomMemberEventContent>(&to_string(&content)).unwrap();
		assert_eq!(again.displayname.as_deref(), Some("A"));
		assert!(again.third_party_invite.is_some());
		assert!(!to_string(&RoomMemberEventContent::default()).contains("reason"));
	}
}

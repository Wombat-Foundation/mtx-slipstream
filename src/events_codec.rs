//! Codec impls for room event content types.

use crate::{
	codec::{DeError, Deserialize, Serialize, from_value},
	events::room::{
		join_rules::{JoinRule, RestrictedRule, RoomJoinRulesEventContent},
		member::RoomMemberEventContent,
		power_levels::RoomPowerLevelsEventContent,
		redaction::RoomRedactionEventContent,
		third_party_invite::RoomThirdPartyInviteEventContent,
	},
	json::{Object, Value},
	power_levels::NotificationPowerLevels,
	serde::deserialize_v1_powerlevel,
};

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

impl Serialize for RoomMemberEventContent {
	fn to_json(&self) -> Value {
		let mut o = Object::new();
		insert(&mut o, "membership", &self.membership);
		Value::Object(o)
	}
}
impl Deserialize for RoomMemberEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let membership = field(object(value)?, "membership")?
			.ok_or_else(|| DeError::expected("membership"))?;
		Ok(Self {
			membership,
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

//! Event type enums with ruma-style `Custom` fallbacks.

use alloc::{borrow::Cow, string::String};
use core::fmt;

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TimelineEventType {
	RoomAliases,
	RoomCreate,
	RoomJoinRules,
	RoomMember,
	RoomMessage,
	RoomPowerLevels,
	RoomRedaction,
	RoomThirdPartyInvite,
	RoomTopic,
	RoomEncrypted,
	RoomEncryption,
	RoomName,
	RoomServerAcl,
	RoomTombstone,
	SpaceChild,
	Beacon,
	CallInvite,
	Audio,
	Emote,
	Image,
	KeyVerificationStart,
	Location,
	Reaction,
	Video,
	Voice,
	CallNotify,
	PollStart,
	Sticker,
	UnstablePollStart,
	Custom(String),
}
impl TimelineEventType {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::RoomAliases => "m.room.aliases",
			Self::RoomCreate => "m.room.create",
			Self::RoomJoinRules => "m.room.join_rules",
			Self::RoomMember => "m.room.member",
			Self::RoomMessage => "m.room.message",
			Self::RoomPowerLevels => "m.room.power_levels",
			Self::RoomRedaction => "m.room.redaction",
			Self::RoomThirdPartyInvite => "m.room.third_party_invite",
			Self::RoomTopic => "m.room.topic",
			Self::RoomEncrypted => "m.room.encrypted",
			Self::RoomEncryption => "m.room.encryption",
			Self::RoomName => "m.room.name",
			Self::RoomServerAcl => "m.room.server_acl",
			Self::RoomTombstone => "m.room.tombstone",
			Self::SpaceChild => "m.space.child",
			Self::Beacon => "org.matrix.msc3488.beacon",
			Self::CallInvite => "m.call.invite",
			Self::Audio => "m.audio",
			Self::Emote => "m.emote",
			Self::Image => "m.image",
			Self::KeyVerificationStart => "m.key.verification.start",
			Self::Location => "m.location",
			Self::Reaction => "m.reaction",
			Self::Video => "m.video",
			Self::Voice => "m.voice",
			Self::CallNotify => "m.call.notify",
			Self::PollStart => "m.poll.start",
			Self::Sticker => "m.sticker",
			Self::UnstablePollStart => "org.matrix.msc3381.poll.start",
			Self::Custom(other) => other,
		}
	}
	#[must_use]
	pub fn to_cow_str(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}
impl AsRef<str> for TimelineEventType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl From<&str> for TimelineEventType {
	fn from(value: &str) -> Self {
		if value == "m.room.aliases" {
			return Self::RoomAliases;
		}
		if value == "m.room.create" {
			return Self::RoomCreate;
		}
		if value == "m.room.join_rules" {
			return Self::RoomJoinRules;
		}
		if value == "m.room.member" {
			return Self::RoomMember;
		}
		if value == "m.room.message" {
			return Self::RoomMessage;
		}
		if value == "m.room.power_levels" {
			return Self::RoomPowerLevels;
		}
		if value == "m.room.redaction" {
			return Self::RoomRedaction;
		}
		if value == "m.room.third_party_invite" {
			return Self::RoomThirdPartyInvite;
		}
		if value == "m.room.topic" {
			return Self::RoomTopic;
		}
		if value == "m.room.encrypted" {
			return Self::RoomEncrypted;
		}
		if value == "m.room.encryption" {
			return Self::RoomEncryption;
		}
		if value == "m.room.name" {
			return Self::RoomName;
		}
		if value == "m.room.server_acl" {
			return Self::RoomServerAcl;
		}
		if value == "m.room.tombstone" {
			return Self::RoomTombstone;
		}
		if value == "m.space.child" {
			return Self::SpaceChild;
		}
		if value == "org.matrix.msc3488.beacon" {
			return Self::Beacon;
		}
		if value == "m.call.invite" {
			return Self::CallInvite;
		}
		if value == "m.audio" {
			return Self::Audio;
		}
		if value == "m.emote" {
			return Self::Emote;
		}
		if value == "m.image" {
			return Self::Image;
		}
		if value == "m.key.verification.start" {
			return Self::KeyVerificationStart;
		}
		if value == "m.location" {
			return Self::Location;
		}
		if value == "m.reaction" {
			return Self::Reaction;
		}
		if value == "m.video" {
			return Self::Video;
		}
		if value == "m.voice" {
			return Self::Voice;
		}
		if value == "m.call.notify" {
			return Self::CallNotify;
		}
		if value == "m.poll.start" {
			return Self::PollStart;
		}
		if value == "m.sticker" {
			return Self::Sticker;
		}
		if value == "org.matrix.msc3381.poll.start" {
			return Self::UnstablePollStart;
		}
		Self::Custom(value.into())
	}
}
impl From<String> for TimelineEventType {
	fn from(value: String) -> Self {
		if value.as_str() == "m.room.aliases" {
			return Self::RoomAliases;
		}
		if value.as_str() == "m.room.create" {
			return Self::RoomCreate;
		}
		if value.as_str() == "m.room.join_rules" {
			return Self::RoomJoinRules;
		}
		if value.as_str() == "m.room.member" {
			return Self::RoomMember;
		}
		if value.as_str() == "m.room.message" {
			return Self::RoomMessage;
		}
		if value.as_str() == "m.room.power_levels" {
			return Self::RoomPowerLevels;
		}
		if value.as_str() == "m.room.redaction" {
			return Self::RoomRedaction;
		}
		if value.as_str() == "m.room.third_party_invite" {
			return Self::RoomThirdPartyInvite;
		}
		if value.as_str() == "m.room.topic" {
			return Self::RoomTopic;
		}
		if value.as_str() == "m.room.encrypted" {
			return Self::RoomEncrypted;
		}
		if value.as_str() == "m.room.encryption" {
			return Self::RoomEncryption;
		}
		if value.as_str() == "m.room.name" {
			return Self::RoomName;
		}
		if value.as_str() == "m.room.server_acl" {
			return Self::RoomServerAcl;
		}
		if value.as_str() == "m.room.tombstone" {
			return Self::RoomTombstone;
		}
		if value.as_str() == "m.space.child" {
			return Self::SpaceChild;
		}
		if value.as_str() == "org.matrix.msc3488.beacon" {
			return Self::Beacon;
		}
		if value.as_str() == "m.call.invite" {
			return Self::CallInvite;
		}
		if value.as_str() == "m.audio" {
			return Self::Audio;
		}
		if value.as_str() == "m.emote" {
			return Self::Emote;
		}
		if value.as_str() == "m.image" {
			return Self::Image;
		}
		if value.as_str() == "m.key.verification.start" {
			return Self::KeyVerificationStart;
		}
		if value.as_str() == "m.location" {
			return Self::Location;
		}
		if value.as_str() == "m.reaction" {
			return Self::Reaction;
		}
		if value.as_str() == "m.video" {
			return Self::Video;
		}
		if value.as_str() == "m.voice" {
			return Self::Voice;
		}
		if value.as_str() == "m.call.notify" {
			return Self::CallNotify;
		}
		if value.as_str() == "m.poll.start" {
			return Self::PollStart;
		}
		if value.as_str() == "m.sticker" {
			return Self::Sticker;
		}
		if value.as_str() == "org.matrix.msc3381.poll.start" {
			return Self::UnstablePollStart;
		}
		Self::Custom(value)
	}
}
impl fmt::Display for TimelineEventType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl PartialEq<str> for TimelineEventType {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl PartialEq<&str> for TimelineEventType {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}
impl Serialize for TimelineEventType {
	fn to_json(&self) -> Value {
		Value::String(self.as_str().into())
	}
}
impl Deserialize for TimelineEventType {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_str()
			.map(Self::from)
			.ok_or_else(|| DeError::expected(stringify!(TimelineEventType)))
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StateEventType {
	RoomAliases,
	RoomCreate,
	RoomJoinRules,
	RoomMember,
	RoomPowerLevels,
	RoomThirdPartyInvite,
	RoomTopic,
	RoomEncryption,
	RoomName,
	RoomAvatar,
	RoomCanonicalAlias,
	RoomGuestAccess,
	RoomHistoryVisibility,
	RoomServerAcl,
	RoomTombstone,
	RoomPolicy,
	SpaceChild,
	SpaceParent,
	Custom(String),
}
impl StateEventType {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::RoomAliases => "m.room.aliases",
			Self::RoomCreate => "m.room.create",
			Self::RoomJoinRules => "m.room.join_rules",
			Self::RoomMember => "m.room.member",
			Self::RoomPowerLevels => "m.room.power_levels",
			Self::RoomThirdPartyInvite => "m.room.third_party_invite",
			Self::RoomTopic => "m.room.topic",
			Self::RoomEncryption => "m.room.encryption",
			Self::RoomName => "m.room.name",
			Self::RoomAvatar => "m.room.avatar",
			Self::RoomCanonicalAlias => "m.room.canonical_alias",
			Self::RoomGuestAccess => "m.room.guest_access",
			Self::RoomHistoryVisibility => "m.room.history_visibility",
			Self::RoomServerAcl => "m.room.server_acl",
			Self::RoomTombstone => "m.room.tombstone",
			Self::RoomPolicy => "m.policy.rule.room",
			Self::SpaceChild => "m.space.child",
			Self::SpaceParent => "m.space.parent",
			Self::Custom(other) => other,
		}
	}
	#[must_use]
	pub fn to_cow_str(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}
impl AsRef<str> for StateEventType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl From<&str> for StateEventType {
	fn from(value: &str) -> Self {
		if value == "m.room.aliases" {
			return Self::RoomAliases;
		}
		if value == "m.room.create" {
			return Self::RoomCreate;
		}
		if value == "m.room.join_rules" {
			return Self::RoomJoinRules;
		}
		if value == "m.room.member" {
			return Self::RoomMember;
		}
		if value == "m.room.power_levels" {
			return Self::RoomPowerLevels;
		}
		if value == "m.room.third_party_invite" {
			return Self::RoomThirdPartyInvite;
		}
		if value == "m.room.topic" {
			return Self::RoomTopic;
		}
		if value == "m.room.encryption" {
			return Self::RoomEncryption;
		}
		if value == "m.room.name" {
			return Self::RoomName;
		}
		if value == "m.room.avatar" {
			return Self::RoomAvatar;
		}
		if value == "m.room.canonical_alias" {
			return Self::RoomCanonicalAlias;
		}
		if value == "m.room.guest_access" {
			return Self::RoomGuestAccess;
		}
		if value == "m.room.history_visibility" {
			return Self::RoomHistoryVisibility;
		}
		if value == "m.room.server_acl" {
			return Self::RoomServerAcl;
		}
		if value == "m.room.tombstone" {
			return Self::RoomTombstone;
		}
		if value == "m.policy.rule.room" {
			return Self::RoomPolicy;
		}
		if value == "m.space.child" {
			return Self::SpaceChild;
		}
		if value == "m.space.parent" {
			return Self::SpaceParent;
		}
		Self::Custom(value.into())
	}
}
impl From<String> for StateEventType {
	fn from(value: String) -> Self {
		if value.as_str() == "m.room.aliases" {
			return Self::RoomAliases;
		}
		if value.as_str() == "m.room.create" {
			return Self::RoomCreate;
		}
		if value.as_str() == "m.room.join_rules" {
			return Self::RoomJoinRules;
		}
		if value.as_str() == "m.room.member" {
			return Self::RoomMember;
		}
		if value.as_str() == "m.room.power_levels" {
			return Self::RoomPowerLevels;
		}
		if value.as_str() == "m.room.third_party_invite" {
			return Self::RoomThirdPartyInvite;
		}
		if value.as_str() == "m.room.topic" {
			return Self::RoomTopic;
		}
		if value.as_str() == "m.room.encryption" {
			return Self::RoomEncryption;
		}
		if value.as_str() == "m.room.name" {
			return Self::RoomName;
		}
		if value.as_str() == "m.room.avatar" {
			return Self::RoomAvatar;
		}
		if value.as_str() == "m.room.canonical_alias" {
			return Self::RoomCanonicalAlias;
		}
		if value.as_str() == "m.room.guest_access" {
			return Self::RoomGuestAccess;
		}
		if value.as_str() == "m.room.history_visibility" {
			return Self::RoomHistoryVisibility;
		}
		if value.as_str() == "m.room.server_acl" {
			return Self::RoomServerAcl;
		}
		if value.as_str() == "m.room.tombstone" {
			return Self::RoomTombstone;
		}
		if value.as_str() == "m.policy.rule.room" {
			return Self::RoomPolicy;
		}
		if value.as_str() == "m.space.child" {
			return Self::SpaceChild;
		}
		if value.as_str() == "m.space.parent" {
			return Self::SpaceParent;
		}
		Self::Custom(value)
	}
}
impl fmt::Display for StateEventType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl PartialEq<str> for StateEventType {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl PartialEq<&str> for StateEventType {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}
impl Serialize for StateEventType {
	fn to_json(&self) -> Value {
		Value::String(self.as_str().into())
	}
}
impl Deserialize for StateEventType {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_str()
			.map(Self::from)
			.ok_or_else(|| DeError::expected(stringify!(StateEventType)))
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MessageLikeEventType {
	RoomMessage,
	RoomRedaction,
	RoomEncrypted,
	CallInvite,
	Custom(String),
}
impl MessageLikeEventType {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::RoomMessage => "m.room.message",
			Self::RoomRedaction => "m.room.redaction",
			Self::RoomEncrypted => "m.room.encrypted",
			Self::CallInvite => "m.call.invite",
			Self::Custom(other) => other,
		}
	}
	#[must_use]
	pub fn to_cow_str(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}
impl AsRef<str> for MessageLikeEventType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl From<&str> for MessageLikeEventType {
	fn from(value: &str) -> Self {
		if value == "m.room.message" {
			return Self::RoomMessage;
		}
		if value == "m.room.redaction" {
			return Self::RoomRedaction;
		}
		if value == "m.room.encrypted" {
			return Self::RoomEncrypted;
		}
		if value == "m.call.invite" {
			return Self::CallInvite;
		}
		Self::Custom(value.into())
	}
}
impl From<String> for MessageLikeEventType {
	fn from(value: String) -> Self {
		if value.as_str() == "m.room.message" {
			return Self::RoomMessage;
		}
		if value.as_str() == "m.room.redaction" {
			return Self::RoomRedaction;
		}
		if value.as_str() == "m.room.encrypted" {
			return Self::RoomEncrypted;
		}
		if value.as_str() == "m.call.invite" {
			return Self::CallInvite;
		}
		Self::Custom(value)
	}
}
impl fmt::Display for MessageLikeEventType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl PartialEq<str> for MessageLikeEventType {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl PartialEq<&str> for MessageLikeEventType {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}
impl Serialize for MessageLikeEventType {
	fn to_json(&self) -> Value {
		Value::String(self.as_str().into())
	}
}
impl Deserialize for MessageLikeEventType {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_str()
			.map(Self::from)
			.ok_or_else(|| DeError::expected(stringify!(MessageLikeEventType)))
	}
}

impl From<TimelineEventType> for StateEventType {
	fn from(value: TimelineEventType) -> Self {
		Self::from(value.as_str())
	}
}
impl From<TimelineEventType> for MessageLikeEventType {
	fn from(value: TimelineEventType) -> Self {
		Self::from(value.as_str())
	}
}
impl From<StateEventType> for TimelineEventType {
	fn from(value: StateEventType) -> Self {
		Self::from(value.as_str())
	}
}
impl From<MessageLikeEventType> for TimelineEventType {
	fn from(value: MessageLikeEventType) -> Self {
		Self::from(value.as_str())
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GlobalAccountDataEventType {
	PushRules,
	Direct,
	IgnoredUserList,
	Custom(String),
}
impl GlobalAccountDataEventType {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::PushRules => "m.push_rules",
			Self::Direct => "m.direct",
			Self::IgnoredUserList => "m.ignored_user_list",
			Self::Custom(other) => other,
		}
	}
	#[must_use]
	pub fn to_cow_str(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}
impl AsRef<str> for GlobalAccountDataEventType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl From<&str> for GlobalAccountDataEventType {
	fn from(value: &str) -> Self {
		if value == "m.push_rules" {
			return Self::PushRules;
		}
		if value == "m.direct" {
			return Self::Direct;
		}
		if value == "m.ignored_user_list" {
			return Self::IgnoredUserList;
		}
		Self::Custom(value.into())
	}
}
impl From<String> for GlobalAccountDataEventType {
	fn from(value: String) -> Self {
		if value.as_str() == "m.push_rules" {
			return Self::PushRules;
		}
		if value.as_str() == "m.direct" {
			return Self::Direct;
		}
		if value.as_str() == "m.ignored_user_list" {
			return Self::IgnoredUserList;
		}
		Self::Custom(value)
	}
}
impl fmt::Display for GlobalAccountDataEventType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl PartialEq<str> for GlobalAccountDataEventType {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl PartialEq<&str> for GlobalAccountDataEventType {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}
impl Serialize for GlobalAccountDataEventType {
	fn to_json(&self) -> Value {
		Value::String(self.as_str().into())
	}
}
impl Deserialize for GlobalAccountDataEventType {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_str()
			.map(Self::from)
			.ok_or_else(|| DeError::expected(stringify!(GlobalAccountDataEventType)))
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RoomAccountDataEventType {
	Tag,
	FullyRead,
	Custom(String),
}
impl RoomAccountDataEventType {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Tag => "m.tag",
			Self::FullyRead => "m.fully_read",
			Self::Custom(other) => other,
		}
	}
	#[must_use]
	pub fn to_cow_str(&self) -> Cow<'_, str> {
		Cow::Borrowed(self.as_str())
	}
}
impl AsRef<str> for RoomAccountDataEventType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
impl From<&str> for RoomAccountDataEventType {
	fn from(value: &str) -> Self {
		if value == "m.tag" {
			return Self::Tag;
		}
		if value == "m.fully_read" {
			return Self::FullyRead;
		}
		Self::Custom(value.into())
	}
}
impl From<String> for RoomAccountDataEventType {
	fn from(value: String) -> Self {
		if value.as_str() == "m.tag" {
			return Self::Tag;
		}
		if value.as_str() == "m.fully_read" {
			return Self::FullyRead;
		}
		Self::Custom(value)
	}
}
impl fmt::Display for RoomAccountDataEventType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
impl PartialEq<str> for RoomAccountDataEventType {
	fn eq(&self, other: &str) -> bool {
		self.as_str() == other
	}
}
impl PartialEq<&str> for RoomAccountDataEventType {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}
impl Serialize for RoomAccountDataEventType {
	fn to_json(&self) -> Value {
		Value::String(self.as_str().into())
	}
}
impl Deserialize for RoomAccountDataEventType {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value
			.as_str()
			.map(Self::from)
			.ok_or_else(|| DeError::expected(stringify!(RoomAccountDataEventType)))
	}
}

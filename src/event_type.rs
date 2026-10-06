//! Event type enums with ruma-style `Custom` fallbacks.

use alloc::{borrow::Cow, string::String};
use core::fmt;

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

macro_rules! event_type {
	($name:ident { $($variant:ident => $wire:literal),* $(,)? }) => {
		#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
		pub enum $name {
			$($variant,)*
			Custom(String),
		}

		impl $name {
			#[must_use]
			pub fn as_str(&self) -> &str {
				match self {
					$(Self::$variant => $wire,)*
					Self::Custom(other) => other,
				}
			}

			#[must_use]
			pub fn to_cow_str(&self) -> Cow<'_, str> { Cow::Borrowed(self.as_str()) }
		}

		impl AsRef<str> for $name {
			fn as_ref(&self) -> &str { self.as_str() }
		}
		impl From<&str> for $name {
			fn from(value: &str) -> Self {
				$(if value == $wire { return Self::$variant; })*
				Self::Custom(value.into())
			}
		}
		impl From<String> for $name {
			fn from(value: String) -> Self {
				$( if value.as_str() == $wire { return Self::$variant; } )*
				Self::Custom(value)
			}
		}
		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
		}
		impl PartialEq<str> for $name {
			fn eq(&self, other: &str) -> bool { self.as_str() == other }
		}
		impl PartialEq<&str> for $name {
			fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
		}
		impl Serialize for $name {
			fn to_json(&self) -> Value { Value::String(self.as_str().into()) }
		}
		impl Deserialize for $name {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				value
					.as_str()
					.map(Self::from)
					.ok_or_else(|| DeError::expected(stringify!($name)))
			}
		}
	};
}

event_type!(TimelineEventType {
	RoomAliases => "m.room.aliases",
	RoomCreate => "m.room.create",
	RoomJoinRules => "m.room.join_rules",
	RoomMember => "m.room.member",
	RoomMessage => "m.room.message",
	RoomPowerLevels => "m.room.power_levels",
	RoomRedaction => "m.room.redaction",
	RoomThirdPartyInvite => "m.room.third_party_invite",
	RoomTopic => "m.room.topic",
	RoomEncrypted => "m.room.encrypted",
	RoomEncryption => "m.room.encryption",
	RoomName => "m.room.name",
	RoomServerAcl => "m.room.server_acl",
	RoomTombstone => "m.room.tombstone",
	SpaceChild => "m.space.child",
	Beacon => "org.matrix.msc3488.beacon",
	CallInvite => "m.call.invite",
	Audio => "m.audio",
	Emote => "m.emote",
	Image => "m.image",
	KeyVerificationStart => "m.key.verification.start",
	Location => "m.location",
	Reaction => "m.reaction",
	Video => "m.video",
	Voice => "m.voice",
	CallNotify => "m.call.notify",
	PollStart => "org.matrix.msc3381.poll.start",
	Sticker => "m.sticker",
	UnstablePollStart => "org.matrix.msc3381.poll.start",
});

event_type!(StateEventType {
	RoomAliases => "m.room.aliases",
	RoomCreate => "m.room.create",
	RoomJoinRules => "m.room.join_rules",
	RoomMember => "m.room.member",
	RoomPowerLevels => "m.room.power_levels",
	RoomThirdPartyInvite => "m.room.third_party_invite",
	RoomTopic => "m.room.topic",
	RoomEncryption => "m.room.encryption",
	RoomName => "m.room.name",
	RoomAvatar => "m.room.avatar",
	RoomCanonicalAlias => "m.room.canonical_alias",
	RoomGuestAccess => "m.room.guest_access",
	RoomHistoryVisibility => "m.room.history_visibility",
	RoomServerAcl => "m.room.server_acl",
	RoomTombstone => "m.room.tombstone",
	RoomPolicy => "m.policy.rule.room",
	SpaceChild => "m.space.child",
	SpaceParent => "m.space.parent",
});

event_type!(MessageLikeEventType {
	RoomMessage => "m.room.message",
	RoomRedaction => "m.room.redaction",
	RoomEncrypted => "m.room.encrypted",
	CallInvite => "m.call.invite",
});

macro_rules! convert {
	($from:ident => $to:ident) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				Self::from(value.as_str())
			}
		}
	};
}
convert!(TimelineEventType => StateEventType);
convert!(TimelineEventType => MessageLikeEventType);
convert!(StateEventType => TimelineEventType);
convert!(MessageLikeEventType => TimelineEventType);

event_type!(GlobalAccountDataEventType {
	PushRules => "m.push_rules",
	Direct => "m.direct",
	IgnoredUserList => "m.ignored_user_list",
});

event_type!(RoomAccountDataEventType {
	Tag => "m.tag",
	FullyRead => "m.fully_read",
});

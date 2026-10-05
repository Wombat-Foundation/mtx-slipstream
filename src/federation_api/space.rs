//! Space hierarchy over federation.

use alloc::{string::String, vec::Vec};

use crate::{
	OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, RoomVersionId, UInt,
	codec::{DeError, Deserialize, Serialize},
	events::{
		room::{encryption::EventEncryptionAlgorithm, join_rules::JoinRule},
		space::child::HierarchySpaceChildEvent,
	},
	json::Value,
	room::RoomType,
	serde::Raw,
};

/// How a room in a space hierarchy can be joined.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum SpaceRoomJoinRule {
	Invite,
	Knock,
	Private,
	Restricted,
	KnockRestricted,
	#[default]
	Public,
	/// A join rule this server does not know.
	_Custom(String),
}

impl SpaceRoomJoinRule {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Invite => "invite",
			Self::Knock => "knock",
			Self::Private => "private",
			Self::Restricted => "restricted",
			Self::KnockRestricted => "knock_restricted",
			Self::Public => "public",
			Self::_Custom(other) => other,
		}
	}
}

impl From<&str> for SpaceRoomJoinRule {
	fn from(value: &str) -> Self {
		match value {
			"invite" => Self::Invite,
			"knock" => Self::Knock,
			"private" => Self::Private,
			"restricted" => Self::Restricted,
			"knock_restricted" => Self::KnockRestricted,
			"public" => Self::Public,
			other => Self::_Custom(other.into()),
		}
	}
}

impl Serialize for SpaceRoomJoinRule {
	fn to_json(&self) -> Value {
		Value::String(self.as_str().into())
	}
}

impl Deserialize for SpaceRoomJoinRule {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		value.as_str().map(Self::from).ok_or_else(|| DeError::expected("join rule"))
	}
}

impl From<JoinRule> for SpaceRoomJoinRule {
	fn from(rule: JoinRule) -> Self {
		match rule {
			JoinRule::Public => Self::Public,
			JoinRule::Knock => Self::Knock,
			JoinRule::Invite => Self::Invite,
			JoinRule::Private => Self::Private,
			JoinRule::Restricted(_) => Self::Restricted,
			JoinRule::KnockRestricted(_) => Self::KnockRestricted,
		}
	}
}

impl JoinRule {
	/// The rooms whose members may join, for restricted join rules.
	pub fn allowed_rooms(&self) -> impl Iterator<Item = OwnedRoomId> + '_ {
		let rule = match self {
			Self::Restricted(rule) | Self::KnockRestricted(rule) => Some(rule),
			_ => None,
		};
		rule.into_iter().flat_map(|rule| rule.allow.iter()).filter_map(|allow| match allow {
			crate::events::room::join_rules::AllowRule::RoomMembership(membership) => {
				Some(membership.room_id.clone())
			}
			_ => None,
		})
	}
}

impl Serialize for RoomType {
	fn to_json(&self) -> Value {
		Value::String(
			match self {
				Self::Room => "",
				Self::Space => "m.space",
			}
			.into(),
		)
	}
}

impl Deserialize for RoomType {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		match value.as_str() {
			Some("m.space") => Ok(Self::Space),
			Some(_) => Ok(Self::Room),
			None => Err(DeError::expected("room type")),
		}
	}
}

impl Serialize for EventEncryptionAlgorithm {
	fn to_json(&self) -> Value {
		Value::String("m.megolm.v1.aes-sha2".into())
	}
}

impl Deserialize for EventEncryptionAlgorithm {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		match value.as_str() {
			Some("m.megolm.v1.aes-sha2") => Ok(Self::MegolmV1AesSha2),
			_ => Err(DeError::expected("encryption algorithm")),
		}
	}
}

macro_rules! summary {
	(
		$(#[$doc:meta])* $summary:ident, $init:ident,
		$($extra:ident : $extra_ty:ty),*
	) => {
		$(#[$doc])*
		#[derive(Clone, Debug)]
		pub struct $summary {
			pub canonical_alias: Option<OwnedRoomAliasId>,
			pub name: Option<String>,
			pub num_joined_members: UInt,
			pub room_id: OwnedRoomId,
			pub topic: Option<String>,
			pub world_readable: bool,
			pub guest_can_join: bool,
			pub avatar_url: Option<OwnedMxcUri>,
			pub join_rule: SpaceRoomJoinRule,
			pub room_type: Option<RoomType>,
			pub allowed_room_ids: Vec<OwnedRoomId>,
			pub encryption: Option<EventEncryptionAlgorithm>,
			pub room_version: Option<RoomVersionId>,
			$(pub $extra: $extra_ty,)*
		}

		/// The required fields of the summary.
		#[derive(Clone, Debug)]
		pub struct $init {
			pub num_joined_members: UInt,
			pub room_id: OwnedRoomId,
			pub world_readable: bool,
			pub guest_can_join: bool,
			pub join_rule: SpaceRoomJoinRule,
			pub allowed_room_ids: Vec<OwnedRoomId>,
			$(pub $extra: $extra_ty,)*
		}

		impl From<$init> for $summary {
			fn from(init: $init) -> Self {
				Self {
					canonical_alias: None,
					name: None,
					num_joined_members: init.num_joined_members,
					room_id: init.room_id,
					topic: None,
					world_readable: init.world_readable,
					guest_can_join: init.guest_can_join,
					avatar_url: None,
					join_rule: init.join_rule,
					room_type: None,
					allowed_room_ids: init.allowed_room_ids,
					encryption: None,
					room_version: None,
					$($extra: init.$extra,)*
				}
			}
		}

		crate::impl_codec_struct!($summary {
			canonical_alias: Option<OwnedRoomAliasId>,
			name: Option<String>,
			num_joined_members: UInt,
			room_id: OwnedRoomId,
			topic: Option<String>,
			world_readable: bool,
			guest_can_join: bool,
			avatar_url: Option<OwnedMxcUri>,
			join_rule: SpaceRoomJoinRule,
			room_type: Option<RoomType>,
			encryption: Option<EventEncryptionAlgorithm>,
			room_version: Option<RoomVersionId>,
		}
		default {
			allowed_room_ids: Vec<OwnedRoomId>
			$(, $extra: $extra_ty)*
		});
	};
}

summary!(
	/// A room in the hierarchy, with its child events.
	SpaceHierarchyParentSummary, SpaceHierarchyParentSummaryInit,
	children_state: Vec<Raw<HierarchySpaceChildEvent>>
);
summary!(
	/// A child room in the hierarchy.
	SpaceHierarchyChildSummary,
	SpaceHierarchyChildSummaryInit,
);

summary!(
	/// A room in a client-facing hierarchy listing.
	SpaceHierarchyRoomsChunk, SpaceHierarchyRoomsChunkInit,
	children_state: Vec<Raw<HierarchySpaceChildEvent>>
);

impl From<SpaceHierarchyParentSummary> for SpaceHierarchyRoomsChunk {
	fn from(summary: SpaceHierarchyParentSummary) -> Self {
		Self {
			canonical_alias: summary.canonical_alias,
			name: summary.name,
			num_joined_members: summary.num_joined_members,
			room_id: summary.room_id,
			topic: summary.topic,
			world_readable: summary.world_readable,
			guest_can_join: summary.guest_can_join,
			avatar_url: summary.avatar_url,
			join_rule: summary.join_rule,
			room_type: summary.room_type,
			allowed_room_ids: summary.allowed_room_ids,
			encryption: summary.encryption,
			room_version: summary.room_version,
			children_state: summary.children_state,
		}
	}
}

impl From<SpaceHierarchyParentSummary> for SpaceHierarchyChildSummary {
	fn from(summary: SpaceHierarchyParentSummary) -> Self {
		Self {
			canonical_alias: summary.canonical_alias,
			name: summary.name,
			num_joined_members: summary.num_joined_members,
			room_id: summary.room_id,
			topic: summary.topic,
			world_readable: summary.world_readable,
			guest_can_join: summary.guest_can_join,
			avatar_url: summary.avatar_url,
			join_rule: summary.join_rule,
			room_type: summary.room_type,
			allowed_room_ids: summary.allowed_room_ids,
			encryption: summary.encryption,
			room_version: summary.room_version,
		}
	}
}

/// The client-server hierarchy endpoint.
pub mod client {
	pub mod get_hierarchy {
		pub mod v1 {
			use alloc::{string::String, vec::Vec};

			use super::super::super::SpaceHierarchyRoomsChunk;
			use crate::{OwnedRoomId, UInt, endpoint};

			endpoint! {
				method: "GET", path: "/_matrix/client/v1/rooms/{room_id}/hierarchy",
				request {
					path { room_id: OwnedRoomId }
					query {
						from: Option<String>,
						limit: Option<UInt>,
						max_depth: Option<UInt>,
						suggested_only: bool
					}
					body {}
				}
				response { next_batch: Option<String>, rooms: Vec<SpaceHierarchyRoomsChunk> }
			}
		}
	}
}

pub mod get_hierarchy {
	pub mod v1 {
		use alloc::vec::Vec;

		use super::super::{SpaceHierarchyChildSummary, SpaceHierarchyParentSummary};
		use crate::{OwnedRoomId, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/hierarchy/{room_id}",
			request {
				path { room_id: OwnedRoomId }
				query { suggested_only: bool }
				body {}
			}
			response {
				room: SpaceHierarchyParentSummary,
				children: Vec<SpaceHierarchyChildSummary>,
				inaccessible_children: Vec<OwnedRoomId>
			}
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId, suggested_only: bool) -> Self {
				Self {
					room_id,
					suggested_only,
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn summary_round_trips_and_defaults_lists() {
		let summary: SpaceHierarchyParentSummary = SpaceHierarchyParentSummaryInit {
			num_joined_members: 3,
			room_id: OwnedRoomId::from("!r:b"),
			world_readable: true,
			guest_can_join: false,
			join_rule: SpaceRoomJoinRule::Restricted,
			allowed_room_ids: alloc::vec![OwnedRoomId::from("!p:b")],
			children_state: Vec::new(),
		}
		.into();
		let back: SpaceHierarchyParentSummary = from_str(&to_string(&summary)).unwrap();
		assert_eq!(back.room_id, summary.room_id);
		assert_eq!(back.join_rule, SpaceRoomJoinRule::Restricted);
		assert_eq!(back.allowed_room_ids.len(), 1);

		let minimal: SpaceHierarchyChildSummary = from_str(
			r#"{"num_joined_members":1,"room_id":"!c:b","world_readable":false,"guest_can_join":false,"join_rule":"public"}"#,
		)
		.unwrap();
		assert!(minimal.allowed_room_ids.is_empty());
	}

	#[test]
	fn restricted_join_rule_lists_allowed_rooms() {
		let rule: crate::events::room::join_rules::RoomJoinRulesEventContent = from_str(
			r#"{"join_rule":"restricted","allow":[{"type":"m.room_membership","room_id":"!a:b"},{"type":"x"}]}"#,
		)
		.unwrap();
		let rooms: Vec<OwnedRoomId> = rule.join_rule.allowed_rooms().collect();
		assert_eq!(rooms, alloc::vec![OwnedRoomId::from("!a:b")]);
		assert_eq!(SpaceRoomJoinRule::from(rule.join_rule), SpaceRoomJoinRule::Restricted);
	}
}

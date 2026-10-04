//! Room-level client endpoints.

use crate::impl_codec_enum;

/// Whether a room is listed in the public directory.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Visibility {
	Public,
	#[default]
	Private,
}

impl_codec_enum!(Visibility { Public => "public", Private => "private" });

use alloc::string::String;

use crate::api::Direction;

impl_codec_enum!(Direction { Forward => "f", Backward => "b" });

/// An initial state event of a new room; kept raw because only `type`,
/// `state_key` and `content` are read.
#[derive(Clone, Debug, Default)]
pub struct AnyInitialStateEvent;

/// Extra `m.room.create` content a client supplies.
#[derive(Clone, Debug, Default)]
pub struct CreationContent;

/// A third-party invite in a room creation request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Invite3pid {
	pub id_server: String,
	pub id_access_token: String,
	pub medium: String,
	pub address: String,
}

crate::impl_codec_struct!(Invite3pid {
	id_server: String,
	id_access_token: String,
	medium: String,
	address: String,
});

/// The preset a room is created with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoomPreset {
	PrivateChat,
	PublicChat,
	TrustedPrivateChat,
}

impl_codec_enum!(RoomPreset {
	PrivateChat => "private_chat",
	PublicChat => "public_chat",
	TrustedPrivateChat => "trusted_private_chat",
});

/// `POST /_matrix/client/v3/createRoom`.
pub mod create_room {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use super::super::{AnyInitialStateEvent, CreationContent, Visibility};
		use crate::{
			OwnedRoomId, OwnedUserId, RoomVersionId,
			events::room::power_levels::RoomPowerLevelsEventContent, serde::Raw,
		};
		use crate::{
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointRequest, Metadata},
			json::Value,
		};

		pub use super::super::{Invite3pid, RoomPreset};

		#[derive(Clone, Debug, Default)]
		pub struct Request {
			pub creation_content: Option<Raw<CreationContent>>,
			pub initial_state: Vec<Raw<AnyInitialStateEvent>>,
			pub invite: Vec<OwnedUserId>,
			pub invite_3pid: Vec<Invite3pid>,
			pub is_direct: bool,
			pub name: Option<String>,
			pub power_level_content_override: Option<Raw<RoomPowerLevelsEventContent>>,
			pub preset: Option<RoomPreset>,
			pub room_alias_name: Option<String>,
			pub room_version: Option<RoomVersionId>,
			pub topic: Option<String>,
			pub visibility: Visibility,
			/// Requested room ID, for appservices that may choose it.
			pub room_id: Option<OwnedRoomId>,
		}

		impl Request {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
		}

		crate::impl_codec_struct!(Request {} default {
			creation_content: Option<Raw<CreationContent>>,
			initial_state: Vec<Raw<AnyInitialStateEvent>>,
			invite: Vec<OwnedUserId>,
			invite_3pid: Vec<Invite3pid>,
			is_direct: bool,
			name: Option<String>,
			power_level_content_override: Option<Raw<RoomPowerLevelsEventContent>>,
			preset: Option<RoomPreset>,
			room_alias_name: Option<String>,
			room_version: Option<RoomVersionId>,
			topic: Option<String>,
			visibility: Visibility,
			room_id: Option<OwnedRoomId>,
		});

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata::new("POST", "/_matrix/client/v3/createRoom");

			fn path_args(&self) -> Vec<String> {
				Vec::new()
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				Some(self.to_json())
			}

			fn from_parts(
				_path: &[String],
				_query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				body.map_or_else(|| Ok(Self::default()), Self::from_json)
			}
		}

		crate::endpoint_response!(response {
			room_id: OwnedRoomId
		});

		impl Response {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}
	}
}

/// `POST /_matrix/client/v3/rooms/{roomId}/upgrade`.
pub mod upgrade_room {
	pub mod v3 {
		use crate::{OwnedRoomId, RoomVersionId};

		crate::endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/upgrade",
			request {
				path { room_id: OwnedRoomId }
				query {}
				body { new_version: RoomVersionId }
			}
			response { replacement_room: OwnedRoomId }
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId, new_version: RoomVersionId) -> Self {
				Self {
					room_id,
					new_version,
				}
			}
		}

		impl Response {
			#[must_use]
			pub fn new(replacement_room: OwnedRoomId) -> Self {
				Self {
					replacement_room,
				}
			}
		}
	}
}

/// `GET /_matrix/client/v3/rooms/{roomId}/aliases`.
pub mod aliases {
	pub mod v3 {
		use alloc::vec::Vec;

		use crate::{OwnedRoomAliasId, OwnedRoomId};

		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/rooms/{room_id}/aliases",
			request { path { room_id: OwnedRoomId } query {} body {} }
			response { aliases: Vec<OwnedRoomAliasId> }
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}
	}
}

/// `GET /_matrix/client/v3/rooms/{roomId}/event/{eventId}`.
pub mod get_room_event {
	pub mod v3 {
		use crate::{OwnedEventId, OwnedRoomId, events::AnyTimelineEvent, serde::Raw};

		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/rooms/{room_id}/event/{event_id}",
			request {
				path { room_id: OwnedRoomId, event_id: OwnedEventId }
				query {}
				body {}
			}
		}
		crate::endpoint_response_flat!(event: Raw<AnyTimelineEvent>);

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId) -> Self {
				Self {
					room_id,
					event_id,
				}
			}
		}
	}
}

/// MSC3030: the event closest to a timestamp.
pub mod get_event_by_timestamp {
	pub mod v1 {
		use crate::{MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, api::Direction};

		crate::endpoint! {
			method: "GET",
			path: "/_matrix/client/unstable/org.matrix.msc3030/rooms/{room_id}/timestamp_to_event",
			request {
				path { room_id: OwnedRoomId }
				query { ts: MilliSecondsSinceUnixEpoch, dir: Direction }
				body {}
			}
			response { event_id: OwnedEventId, origin_server_ts: MilliSecondsSinceUnixEpoch }
		}

		impl Request {
			#[must_use]
			pub fn new(
				room_id: OwnedRoomId,
				ts: MilliSecondsSinceUnixEpoch,
				dir: Direction,
			) -> Self {
				Self {
					room_id,
					ts,
					dir,
				}
			}
		}

		impl Response {
			#[must_use]
			pub fn new(
				event_id: OwnedEventId,
				origin_server_ts: MilliSecondsSinceUnixEpoch,
			) -> Self {
				Self {
					event_id,
					origin_server_ts,
				}
			}
		}
	}
}

/// MSC3266: a short description of a room.
pub mod get_summary {
	pub mod msc3266 {
		use alloc::{string::String, vec::Vec};

		use crate::{EventEncryptionAlgorithm, room::RoomType, space::SpaceRoomJoinRule};
		use crate::{
			OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, OwnedRoomOrAliasId, OwnedServerName,
			RoomVersionId, UInt, events::room::member::MembershipState,
		};

		crate::endpoint_request! {
			method: "GET",
			path: "/_matrix/client/v1/room_summary/{room_id_or_alias}",
			request {
				path { room_id_or_alias: OwnedRoomOrAliasId }
				query { via: Vec<OwnedServerName> }
				body {}
			}
		}

		impl Request {
			#[must_use]
			pub fn new(room_id_or_alias: OwnedRoomOrAliasId, via: Vec<OwnedServerName>) -> Self {
				Self {
					room_id_or_alias,
					via,
				}
			}
		}

		#[derive(Clone, Debug)]
		pub struct Response {
			pub room_id: OwnedRoomId,
			pub canonical_alias: Option<OwnedRoomAliasId>,
			pub avatar_url: Option<OwnedMxcUri>,
			pub guest_can_join: bool,
			pub name: Option<String>,
			pub num_joined_members: UInt,
			pub topic: Option<String>,
			pub world_readable: bool,
			pub join_rule: SpaceRoomJoinRule,
			pub room_type: Option<RoomType>,
			pub room_version: Option<RoomVersionId>,
			pub encryption: Option<EventEncryptionAlgorithm>,
			pub allowed_room_ids: Vec<OwnedRoomId>,
			/// The requester's membership, when authenticated.
			pub membership: Option<MembershipState>,
		}

		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				let mut fields = alloc::vec![
					("room_id", crate::codec::Serialize::to_json(&self.room_id)),
					("canonical_alias", crate::codec::Serialize::to_json(&self.canonical_alias)),
					("avatar_url", crate::codec::Serialize::to_json(&self.avatar_url)),
					("guest_can_join", crate::codec::Serialize::to_json(&self.guest_can_join)),
					("name", crate::codec::Serialize::to_json(&self.name)),
					(
						"num_joined_members",
						crate::codec::Serialize::to_json(&self.num_joined_members),
					),
					("topic", crate::codec::Serialize::to_json(&self.topic)),
					("world_readable", crate::codec::Serialize::to_json(&self.world_readable)),
					("join_rule", crate::codec::Serialize::to_json(&self.join_rule)),
					("room_type", crate::codec::Serialize::to_json(&self.room_type)),
					("room_version", crate::codec::Serialize::to_json(&self.room_version)),
					("encryption", crate::codec::Serialize::to_json(&self.encryption)),
					("membership", crate::codec::Serialize::to_json(&self.membership)),
				];
				if !self.allowed_room_ids.is_empty() {
					fields.push((
						"allowed_room_ids",
						crate::codec::Serialize::to_json(&self.allowed_room_ids),
					));
				}
				crate::json::Value::Object(crate::endpoint::object_from(fields))
			}

			fn from_body(body: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let input = crate::endpoint::Input::new(&[], &[], Some(body));
				Ok(Self {
					room_id: input.body("room_id")?,
					canonical_alias: input.body("canonical_alias")?,
					avatar_url: input.body("avatar_url")?,
					guest_can_join: input.body_or_default("guest_can_join")?,
					name: input.body("name")?,
					num_joined_members: input.body_or_default("num_joined_members")?,
					topic: input.body("topic")?,
					world_readable: input.body_or_default("world_readable")?,
					join_rule: input.body("join_rule")?,
					room_type: input.body("room_type")?,
					room_version: input.body("room_version")?,
					encryption: input.body("encryption")?,
					allowed_room_ids: input.body_or_default("allowed_room_ids")?,
					membership: input.body("membership")?,
				})
			}
		}
	}
}

/// `GET /_matrix/client/v3/rooms/{roomId}/initialSync` (deprecated).
pub mod initial_sync {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use super::super::Visibility;
		use crate::codec::{DeError, Serialize};
		use crate::{
			OwnedRoomId,
			events::{
				AnyMessageLikeEvent, AnyRoomAccountDataEvent, AnyStateEvent,
				room::member::MembershipState,
			},
			serde::Raw,
		};

		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/rooms/{room_id}/initialSync",
			request { path { room_id: OwnedRoomId } query {} body {} }
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}

		/// A page of room events.
		#[derive(Clone, Debug, Default)]
		pub struct PaginationChunk {
			pub start: Option<String>,
			pub end: String,
			pub chunk: Vec<Raw<AnyMessageLikeEvent>>,
		}

		crate::impl_codec_struct!(PaginationChunk { end: String } default {
			start: Option<String>,
			chunk: Vec<Raw<AnyMessageLikeEvent>>,
		});

		#[derive(Clone, Debug)]
		pub struct Response {
			pub room_id: OwnedRoomId,
			pub account_data: Option<Vec<Raw<AnyRoomAccountDataEvent>>>,
			pub state: Option<Vec<Raw<AnyStateEvent>>>,
			pub messages: Option<PaginationChunk>,
			pub visibility: Visibility,
			pub membership: Option<MembershipState>,
		}

		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::json::Value::Object(crate::endpoint::object_from(alloc::vec![
					("room_id", self.room_id.to_json()),
					("account_data", self.account_data.to_json()),
					("state", self.state.to_json()),
					("messages", self.messages.to_json()),
					("visibility", self.visibility.to_json()),
					("membership", self.membership.to_json()),
				]))
			}

			fn from_body(body: &crate::json::Value) -> Result<Self, DeError> {
				let input = crate::endpoint::Input::new(&[], &[], Some(body));
				Ok(Self {
					room_id: input.body("room_id")?,
					account_data: input.body("account_data")?,
					state: input.body("state")?,
					messages: input.body("messages")?,
					visibility: input.body_or_default("visibility")?,
					membership: input.body("membership")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod room_endpoint_tests {
	use alloc::{string::String, vec::Vec};

	use super::*;
	use crate::{
		MilliSecondsSinceUnixEpoch, OwnedRoomId, RoomVersionId,
		api::{IncomingRequest, OutgoingRequest, SendAccessToken},
		codec::{from_str, to_string},
		endpoint::{EndpointRequest, EndpointResponse, MatrixVersion},
		events::room::member::MembershipState,
		serde::Raw,
	};

	#[test]
	fn create_room_request_defaults_missing_fields() {
		let body = from_str::<crate::json::Value>(
			r#"{"name":"R","preset":"trusted_private_chat","invite":["@a:x"],"is_direct":true}"#,
		)
		.unwrap();
		let request = create_room::v3::Request::from_parts(&[], &[], Some(&body)).unwrap();
		assert_eq!(request.name.as_deref(), Some("R"));
		assert_eq!(request.preset, Some(RoomPreset::TrustedPrivateChat));
		assert!(request.is_direct && request.initial_state.is_empty());
		assert_eq!(request.visibility, Visibility::Private);
		let bare = create_room::v3::Request::from_parts(&[], &[], None).unwrap();
		assert!(bare.invite.is_empty() && bare.room_id.is_none());
	}

	#[test]
	fn timestamp_request_uses_f_and_b() {
		let request = get_event_by_timestamp::v1::Request::new(
			OwnedRoomId::from("!r:x"),
			MilliSecondsSinceUnixEpoch(42),
			Direction::Backward,
		);
		let http: http::Request<Vec<u8>> = request
			.try_into_http_request("https://hs", SendAccessToken::None, &[MatrixVersion::V1_11])
			.unwrap();
		assert!(http.uri().to_string().ends_with("timestamp_to_event?ts=42&dir=b"));
		let parsed =
			get_event_by_timestamp::v1::Request::try_from_http_request(http, &["!r:x"]).unwrap();
		assert_eq!(parsed.dir, Direction::Backward);
		assert_eq!(parsed.ts, MilliSecondsSinceUnixEpoch(42));
	}

	#[test]
	fn summary_response_omits_empty_allowed_rooms_and_round_trips() {
		let response = get_summary::msc3266::Response {
			room_id: OwnedRoomId::from("!r:x"),
			canonical_alias: None,
			avatar_url: None,
			guest_can_join: false,
			name: Some("N".into()),
			num_joined_members: 3,
			topic: None,
			world_readable: true,
			join_rule: crate::space::SpaceRoomJoinRule::Public,
			room_type: None,
			room_version: Some(RoomVersionId::V11),
			encryption: None,
			allowed_room_ids: Vec::new(),
			membership: Some(MembershipState::Join),
		};
		let body = response.to_body();
		assert!(body.get("allowed_room_ids").is_none());
		let again = get_summary::msc3266::Response::from_body(&body).unwrap();
		assert_eq!(again.num_joined_members, 3);
		assert_eq!(again.membership, Some(MembershipState::Join));
		assert!(to_string(&body).contains("\"membership\":\"join\""));
	}

	#[test]
	fn raw_get_field_reads_one_key() {
		let raw = Raw::<AnyInitialStateEvent>::from_json_text(
			r#"{"type":"m.room.name","state_key":"","content":{"name":"x"}}"#,
		)
		.unwrap();
		assert_eq!(raw.get_field::<String>("type").unwrap().as_deref(), Some("m.room.name"));
		assert!(raw.get_field::<String>("missing").unwrap().is_none());
	}
}

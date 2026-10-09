//! Room-level client endpoints.

/// Whether a room is listed in the public directory.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Visibility {
	Public,
	#[default]
	Private,
}

impl crate::codec::Serialize for Visibility {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Public => "public",
			Self::Private => "private",
		}))
	}
}
impl crate::codec::Deserialize for Visibility {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("public") => Ok(Self::Public),
			Some("private") => Ok(Self::Private),
			_ => Err(crate::codec::DeError::expected(stringify!(Visibility))),
		}
	}
}

use alloc::string::String;

use crate::api::Direction;

impl crate::codec::Serialize for Direction {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Forward => "f",
			Self::Backward => "b",
		}))
	}
}
impl crate::codec::Deserialize for Direction {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("f") => Ok(Self::Forward),
			Some("b") => Ok(Self::Backward),
			_ => Err(crate::codec::DeError::expected(stringify!(Direction))),
		}
	}
}

/// An initial state event of a new room; kept raw because only `type`,
/// `state_key` and `content` are read.
#[derive(Debug, Default)]
pub struct AnyInitialStateEvent;

/// Extra `m.room.create` content a client supplies.
#[derive(Debug, Default)]
pub struct CreationContent;

/// A third-party invite in a room creation request.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Invite3pid {
	pub id_server: String,
	pub id_access_token: String,
	pub medium: String,
	pub address: String,
}

impl crate::codec::Serialize for Invite3pid {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(id_server), crate::endpoint::enc(&self.id_server)),
			(stringify!(id_access_token), crate::endpoint::enc(&self.id_access_token)),
			(stringify!(medium), crate::endpoint::enc(&self.medium)),
			(stringify!(address), crate::endpoint::enc(&self.address)),
		])
	}
}
impl crate::codec::Deserialize for Invite3pid {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Invite3pid)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			id_server: input.body(stringify!(id_server))?,
			id_access_token: input.body(stringify!(id_access_token))?,
			medium: input.body(stringify!(medium))?,
			address: input.body(stringify!(address))?,
		})
	}
}

/// The preset a room is created with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoomPreset {
	PrivateChat,
	PublicChat,
	TrustedPrivateChat,
}

impl crate::codec::Serialize for RoomPreset {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::PrivateChat => "private_chat",
			Self::PublicChat => "public_chat",
			Self::TrustedPrivateChat => "trusted_private_chat",
		}))
	}
}
impl crate::codec::Deserialize for RoomPreset {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("private_chat") => Ok(Self::PrivateChat),
			Some("public_chat") => Ok(Self::PublicChat),
			Some("trusted_private_chat") => Ok(Self::TrustedPrivateChat),
			_ => Err(crate::codec::DeError::expected(stringify!(RoomPreset))),
		}
	}
}

/// `POST /_matrix/client/v3/createRoom`.
pub mod create_room {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use super::super::{AnyInitialStateEvent, CreationContent, Visibility};
		use crate::{
			OwnedRoomId, OwnedUserId, RoomVersionId,
			events::room::power_levels::RoomPowerLevelsEventContent, sswire::Raw,
		};
		use crate::{
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointRequest, Metadata},
			json::Value,
		};

		pub use super::super::{Invite3pid, RoomPreset};

		#[derive(Debug, Default)]
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
			/// Requested room ID (local part or fully qualified), for
			/// appservices that may choose it. Also read from `fi.mau.room_id`.
			pub room_id: Option<String>,
			/// Requested PDU timestamp, to permit custom v12 hash prefixes.
			/// Also read from `fi.mau.origin_server_ts`.
			pub origin_server_ts: Option<crate::MilliSecondsSinceUnixEpoch>,
		}

		impl Request {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
		}

		impl crate::codec::Serialize for Request {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(creation_content), crate::endpoint::enc(&self.creation_content)),
					(stringify!(initial_state), crate::endpoint::enc(&self.initial_state)),
					(stringify!(invite), crate::endpoint::enc(&self.invite)),
					(stringify!(invite_3pid), crate::endpoint::enc(&self.invite_3pid)),
					(stringify!(is_direct), crate::endpoint::enc(&self.is_direct)),
					(stringify!(name), crate::endpoint::enc(&self.name)),
					(
						stringify!(power_level_content_override),
						crate::endpoint::enc(&self.power_level_content_override),
					),
					(stringify!(preset), crate::endpoint::enc(&self.preset)),
					(stringify!(room_alias_name), crate::endpoint::enc(&self.room_alias_name)),
					(stringify!(room_version), crate::endpoint::enc(&self.room_version)),
					(stringify!(topic), crate::endpoint::enc(&self.topic)),
					(stringify!(visibility), crate::endpoint::enc(&self.visibility)),
					(stringify!(room_id), crate::endpoint::enc(&self.room_id)),
					(stringify!(origin_server_ts), crate::endpoint::enc(&self.origin_server_ts)),
				])
			}
		}
		impl crate::codec::Deserialize for Request {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(Request)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					creation_content: input.body_or_default(stringify!(creation_content))?,
					initial_state: input.body_or_default(stringify!(initial_state))?,
					invite: input.body_or_default(stringify!(invite))?,
					invite_3pid: input.body_or_default(stringify!(invite_3pid))?,
					is_direct: input.body_or_default(stringify!(is_direct))?,
					name: input.body_or_default(stringify!(name))?,
					power_level_content_override: input
						.body_or_default(stringify!(power_level_content_override))?,
					preset: input.body_or_default(stringify!(preset))?,
					room_alias_name: input.body_or_default(stringify!(room_alias_name))?,
					room_version: input.body_or_default(stringify!(room_version))?,
					topic: input.body_or_default(stringify!(topic))?,
					visibility: input.body_or_default(stringify!(visibility))?,
					room_id: input.body_or_default(stringify!(room_id))?,
					origin_server_ts: input.body_or_default(stringify!(origin_server_ts))?,
				})
			}
		}

		const _: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/createRoom");
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
				let Some(Value::Object(fields)) = body else {
					return body.map_or_else(|| Ok(Self::default()), Self::from_json);
				};
				// Appservices may use the namespaced spellings; the plain key wins.
				let mut fields = fields.clone();
				for (alias, key) in [
					("fi.mau.room_id", "room_id"),
					("fi.mau.origin_server_ts", "origin_server_ts"),
				] {
					if let Some(value) = fields.remove(alias) {
						fields.entry(key.into()).or_insert(value);
					}
				}
				Self::from_json(&Value::Object(fields))
			}
		}

		pub struct Response {
			pub room_id: OwnedRoomId,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"room_id",
					crate::endpoint::enc(&self.room_id),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					room_id: input.body("room_id")?,
				})
			}
		}

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

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub new_version: RoomVersionId,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/rooms/{room_id}/upgrade",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("new_version", crate::endpoint::enc(&self.new_version))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					new_version: input.body("new_version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub replacement_room: OwnedRoomId,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"replacement_room",
					crate::endpoint::enc(&self.replacement_room),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					replacement_room: input.body("replacement_room")?,
				})
			}
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

		pub struct Request {
			pub room_id: OwnedRoomId,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/rooms/{room_id}/aliases",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub aliases: Vec<OwnedRoomAliasId>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"aliases",
					crate::endpoint::enc(&self.aliases),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					aliases: input.body("aliases")?,
				})
			}
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
		use crate::{OwnedEventId, OwnedRoomId, events::AnyTimelineEvent, sswire::Raw};

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/rooms/{room_id}/event/{event_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					event_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub event: Raw<AnyTimelineEvent>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.event)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					event: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}

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

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub ts: MilliSecondsSinceUnixEpoch,
			pub dir: Direction,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/unstable/org.matrix.msc3030/rooms/{room_id}/timestamp_to_event",
			)
			.with_aliases(&["/_matrix/client/v1/rooms/{room_id}/timestamp_to_event"]);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("ts", crate::endpoint::enc(&self.ts)),
					("dir", crate::endpoint::enc(&self.dir)),
				])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					ts: input.query("ts")?,
					dir: input.query("dir")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub event_id: OwnedEventId,
			pub origin_server_ts: MilliSecondsSinceUnixEpoch,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("event_id", crate::endpoint::enc(&self.event_id)),
					("origin_server_ts", crate::endpoint::enc(&self.origin_server_ts)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					event_id: input.body("event_id")?,
					origin_server_ts: input.body("origin_server_ts")?,
				})
			}
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

		pub struct Request {
			pub room_id_or_alias: OwnedRoomOrAliasId,
			pub via: Vec<OwnedServerName>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v1/room_summary/{room_id_or_alias}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.room_id_or_alias,
				)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [("via", crate::endpoint::enc(&self.via))])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id_or_alias: input.path()?,
					via: input.query("via")?,
				};
				input.finish()?;
				Ok(value)
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

		#[derive(Debug)]
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
			sswire::Raw,
		};

		pub struct Request {
			pub room_id: OwnedRoomId,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/rooms/{room_id}/initialSync",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
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
		#[derive(Debug, Default)]
		pub struct PaginationChunk {
			pub start: Option<String>,
			pub end: String,
			pub chunk: Vec<Raw<AnyMessageLikeEvent>>,
		}

		impl crate::codec::Serialize for PaginationChunk {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(end), crate::endpoint::enc(&self.end)),
					(stringify!(start), crate::endpoint::enc(&self.start)),
					(stringify!(chunk), crate::endpoint::enc(&self.chunk)),
				])
			}
		}
		impl crate::codec::Deserialize for PaginationChunk {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(PaginationChunk)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					end: input.body(stringify!(end))?,
					start: input.body_or_default(stringify!(start))?,
					chunk: input.body_or_default(stringify!(chunk))?,
				})
			}
		}

		#[derive(Debug)]
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
		sswire::Raw,
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
		let aliased = from_str::<crate::json::Value>(
			r#"{"fi.mau.room_id":"custom","fi.mau.origin_server_ts":42}"#,
		)
		.unwrap();
		let aliased = create_room::v3::Request::from_parts(&[], &[], Some(&aliased)).unwrap();
		assert_eq!(aliased.room_id.as_deref(), Some("custom"));
		assert_eq!(aliased.origin_server_ts, Some(MilliSecondsSinceUnixEpoch(42)));
		let plain = from_str::<crate::json::Value>(
			r#"{"room_id":"a","fi.mau.room_id":"b","origin_server_ts":7}"#,
		)
		.unwrap();
		let plain = create_room::v3::Request::from_parts(&[], &[], Some(&plain)).unwrap();
		assert_eq!(plain.room_id.as_deref(), Some("a"));
		assert_eq!(plain.origin_server_ts, Some(MilliSecondsSinceUnixEpoch(7)));
		let bare = create_room::v3::Request::from_parts(&[], &[], None).unwrap();
		assert!(bare.invite.is_empty() && bare.room_id.is_none());
	}

	#[test]
	fn timestamp_request_uses_f_and_b() {
		let request = get_event_by_timestamp::v1::Request::new(
			OwnedRoomId::parse("!r:x").unwrap(),
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
			room_id: OwnedRoomId::parse("!r:x").unwrap(),
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

#[cfg(test)]
mod timestamp_alias_tests {
	use crate::endpoint::EndpointRequest;

	#[test]
	fn timestamp_to_event_is_also_served_on_the_stable_v1_path() {
		let metadata = super::get_event_by_timestamp::v1::Request::METADATA;
		assert!(metadata.path.contains("org.matrix.msc3030"));
		assert_eq!(metadata.aliases, &["/_matrix/client/v1/rooms/{room_id}/timestamp_to_event"]);
	}
}

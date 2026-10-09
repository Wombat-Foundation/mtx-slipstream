//! Public room directory types and endpoints.

use alloc::{string::String, vec::Vec};

use crate::{
	OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, UInt,
	codec::{DeError, Deserialize, Serialize},
	json::Value,
	room::RoomType,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
	Public,
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

/// Which network a directory query targets.
#[derive(Debug, Default, Eq, PartialEq)]
pub enum RoomNetwork {
	/// Rooms on the Matrix network only.
	#[default]
	Matrix,
	/// Rooms on every network.
	All,
	/// Rooms on one third-party network.
	ThirdParty(String),
}

impl RoomNetwork {
	fn fields(&self) -> (bool, Option<&str>) {
		match self {
			Self::Matrix => (false, None),
			Self::All => (true, None),
			Self::ThirdParty(id) => (false, Some(id)),
		}
	}

	fn from_fields(include_all: bool, third_party: Option<String>) -> Self {
		match (include_all, third_party) {
			(_, Some(id)) => Self::ThirdParty(id),
			(true, None) => Self::All,
			(false, None) => Self::Matrix,
		}
	}
}

/// A room type to filter the directory by.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum RoomTypeFilter {
	/// Rooms without a type.
	#[default]
	Default,
	Space,
	/// Any other room type.
	_Custom(String),
}

impl RoomTypeFilter {
	#[must_use]
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::Default => None,
			Self::Space => Some("m.space"),
			Self::_Custom(kind) => Some(kind),
		}
	}
}

impl From<Option<RoomType>> for RoomTypeFilter {
	fn from(value: Option<RoomType>) -> Self {
		match value {
			Some(RoomType::Space) => Self::Space,
			_ => Self::Default,
		}
	}
}

impl Serialize for RoomTypeFilter {
	fn to_json(&self) -> Value {
		self.as_str().map_or(Value::Null, |kind| Value::String(kind.into()))
	}
}

impl Deserialize for RoomTypeFilter {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		match value {
			Value::Null => Ok(Self::Default),
			Value::String(kind) if kind == "m.space" => Ok(Self::Space),
			Value::String(kind) => Ok(Self::_Custom(kind.clone())),
			_ => Err(DeError::expected("room type")),
		}
	}
}

/// Search criteria for a filtered directory query.
#[derive(Debug, Default)]
pub struct Filter {
	pub generic_search_term: Option<String>,
	pub room_types: Vec<RoomTypeFilter>,
}

impl crate::codec::Serialize for Filter {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(generic_search_term), crate::endpoint::enc(&self.generic_search_term)),
			(stringify!(room_types), crate::endpoint::enc(&self.room_types)),
		])
	}
}
impl crate::codec::Deserialize for Filter {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Filter)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			generic_search_term: input.body_or_default(stringify!(generic_search_term))?,
			room_types: input.body_or_default(stringify!(room_types))?,
		})
	}
}

/// How a public room can be joined.
#[derive(Debug, Default, Eq, PartialEq)]
pub enum PublicRoomJoinRule {
	#[default]
	Public,
	Knock,
	/// Any other join rule.
	_Custom(String),
}

impl PublicRoomJoinRule {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Public => "public",
			Self::Knock => "knock",
			Self::_Custom(rule) => rule,
		}
	}
}

impl From<&str> for PublicRoomJoinRule {
	fn from(value: &str) -> Self {
		match value {
			"public" => Self::Public,
			"knock" => Self::Knock,
			other => Self::_Custom(other.into()),
		}
	}
}

impl crate::codec::Serialize for PublicRoomJoinRule {
	fn to_json(&self) -> crate::json::Value {
		let v = self;
		crate::json::Value::String(::alloc::string::String::from(v.as_str()))
	}
}
impl crate::codec::Deserialize for PublicRoomJoinRule {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		let s = value.as_str().ok_or_else(|| crate::codec::DeError::expected("string"))?;
		Ok(Self::from(s))
	}
}

/// One room in the public directory.
#[derive(Debug)]
pub struct PublicRoomsChunk {
	pub avatar_url: Option<OwnedMxcUri>,
	pub canonical_alias: Option<OwnedRoomAliasId>,
	pub guest_can_join: bool,
	pub join_rule: PublicRoomJoinRule,
	pub name: Option<String>,
	pub num_joined_members: UInt,
	pub room_id: OwnedRoomId,
	pub room_type: Option<RoomType>,
	pub topic: Option<String>,
	pub world_readable: bool,
}

impl crate::codec::Serialize for PublicRoomsChunk {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(num_joined_members), crate::endpoint::enc(&self.num_joined_members)),
			(stringify!(room_id), crate::endpoint::enc(&self.room_id)),
			(stringify!(avatar_url), crate::endpoint::enc(&self.avatar_url)),
			(stringify!(canonical_alias), crate::endpoint::enc(&self.canonical_alias)),
			(stringify!(guest_can_join), crate::endpoint::enc(&self.guest_can_join)),
			(stringify!(join_rule), crate::endpoint::enc(&self.join_rule)),
			(stringify!(name), crate::endpoint::enc(&self.name)),
			(stringify!(room_type), crate::endpoint::enc(&self.room_type)),
			(stringify!(topic), crate::endpoint::enc(&self.topic)),
			(stringify!(world_readable), crate::endpoint::enc(&self.world_readable)),
		])
	}
}
impl crate::codec::Deserialize for PublicRoomsChunk {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(PublicRoomsChunk)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			num_joined_members: input.body(stringify!(num_joined_members))?,
			room_id: input.body(stringify!(room_id))?,
			avatar_url: input.body_or_default(stringify!(avatar_url))?,
			canonical_alias: input.body_or_default(stringify!(canonical_alias))?,
			guest_can_join: input.body_or_default(stringify!(guest_can_join))?,
			join_rule: input.body_or_default(stringify!(join_rule))?,
			name: input.body_or_default(stringify!(name))?,
			room_type: input.body_or_default(stringify!(room_type))?,
			topic: input.body_or_default(stringify!(topic))?,
			world_readable: input.body_or_default(stringify!(world_readable))?,
		})
	}
}

pub mod get_public_rooms {
	pub mod v3 {
		pub struct Request {
			pub limit: Option<crate::UInt>,
			pub since: Option<alloc::string::String>,
			pub server: Option<crate::OwnedServerName>,
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
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/publicRooms");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("limit", crate::endpoint::enc(&self.limit)),
					("since", crate::endpoint::enc(&self.since)),
					("server", crate::endpoint::enc(&self.server)),
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
					limit: input.query("limit")?,
					since: input.query("since")?,
					server: input.query("server")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub chunk: alloc::vec::Vec<crate::directory::PublicRoomsChunk>,
			pub next_batch: Option<alloc::string::String>,
			pub prev_batch: Option<alloc::string::String>,
			pub total_room_count_estimate: Option<crate::UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("chunk", crate::endpoint::enc(&self.chunk)),
					("next_batch", crate::endpoint::enc(&self.next_batch)),
					("prev_batch", crate::endpoint::enc(&self.prev_batch)),
					(
						"total_room_count_estimate",
						crate::endpoint::enc(&self.total_room_count_estimate),
					),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					chunk: input.body("chunk")?,
					next_batch: input.body("next_batch")?,
					prev_batch: input.body("prev_batch")?,
					total_room_count_estimate: input.body("total_room_count_estimate")?,
				})
			}
		}
	}
}

pub mod get_public_rooms_filtered {
	pub mod v3 {
		#[derive(Debug, Default)]
		pub struct Request {
			pub server: Option<crate::OwnedServerName>,
			pub limit: Option<crate::UInt>,
			pub since: Option<::alloc::string::String>,
			pub filter: crate::directory::Filter,
			pub room_network: crate::directory::RoomNetwork,
		}
		const _: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/publicRooms");
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/publicRooms");
			fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
				::alloc::vec::Vec::new()
			}
			fn query(
				&self,
			) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)>
			{
				crate::endpoint::query_pairs(::alloc::vec![(
					stringify!(server),
					crate::codec::Serialize::to_json(&self.server)
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				let (include_all, third_party) = self.room_network.fields();
				Some(crate::json::Value::Object(crate::endpoint::object_from(::alloc::vec![
					("limit", crate::codec::Serialize::to_json(&self.limit)),
					("since", crate::codec::Serialize::to_json(&self.since)),
					("filter", crate::codec::Serialize::to_json(&self.filter)),
					("include_all_networks", crate::codec::Serialize::to_json(&include_all)),
					("third_party_instance_id", crate::codec::Serialize::to_json(&third_party)),
				])))
			}
			fn from_parts(
				path: &[::alloc::string::String],
				query: &[(::alloc::string::String, ::alloc::string::String)],
				body: Option<&crate::json::Value>,
			) -> Result<Self, crate::codec::DeError> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					server: input.query(stringify!(server))?,
					limit: input.body("limit")?,
					since: input.body("since")?,
					filter: input.body_or_default("filter")?,
					room_network: crate::directory::RoomNetwork::from_fields(
						input.body_or_default("include_all_networks")?,
						input.body("third_party_instance_id")?,
					),
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub chunk: ::alloc::vec::Vec<crate::directory::PublicRoomsChunk>,
			pub next_batch: Option<::alloc::string::String>,
			pub prev_batch: Option<::alloc::string::String>,
			pub total_room_count_estimate: Option<crate::UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("chunk", crate::endpoint::enc(&self.chunk)),
					("next_batch", crate::endpoint::enc(&self.next_batch)),
					("prev_batch", crate::endpoint::enc(&self.prev_batch)),
					(
						"total_room_count_estimate",
						crate::endpoint::enc(&self.total_room_count_estimate),
					),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					chunk: input.body("chunk")?,
					next_batch: input.body("next_batch")?,
					prev_batch: input.body("prev_batch")?,
					total_room_count_estimate: input.body("total_room_count_estimate")?,
				})
			}
		}
	}
}

pub mod federation {
	pub mod get_public_rooms {
		pub mod v1 {
			use crate::{directory::RoomNetwork, json::Value};

			pub struct Response {
				pub chunk: alloc::vec::Vec<crate::directory::PublicRoomsChunk>,
				pub next_batch: Option<alloc::string::String>,
				pub prev_batch: Option<alloc::string::String>,
				pub total_room_count_estimate: Option<crate::UInt>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("chunk", crate::endpoint::enc(&self.chunk)),
						("next_batch", crate::endpoint::enc(&self.next_batch)),
						("prev_batch", crate::endpoint::enc(&self.prev_batch)),
						(
							"total_room_count_estimate",
							crate::endpoint::enc(&self.total_room_count_estimate),
						),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						chunk: input.body("chunk")?,
						next_batch: input.body("next_batch")?,
						prev_batch: input.body("prev_batch")?,
						total_room_count_estimate: input.body("total_room_count_estimate")?,
					})
				}
			}
			pub struct Request {
				pub limit: Option<crate::UInt>,
				pub since: Option<alloc::string::String>,
				pub room_network: crate::directory::RoomNetwork,
			}
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata =
					crate::endpoint::Metadata::new("GET", "/_matrix/federation/v1/publicRooms");
				fn path_args(&self) -> crate::endpoint::Strs {
					alloc::vec::Vec::new()
				}
				fn query(&self) -> crate::endpoint::Pairs {
					let (all, third_party) = self.room_network.fields();
					let mut params = alloc::vec![
						("limit", crate::codec::Serialize::to_json(&self.limit)),
						("since", crate::codec::Serialize::to_json(&self.since)),
						(
							"third_party_instance_id",
							crate::codec::Serialize::to_json(&third_party)
						),
					];
					if all {
						params.push(("include_all_networks", Value::Bool(true)));
					}
					crate::endpoint::query_pairs(params)
				}
				fn body(&self) -> Option<crate::json::Value> {
					None
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> Result<Self, crate::codec::DeError> {
					let input = crate::endpoint::Input::new(path, query, body);
					let all =
						input.query::<Option<bool>>("include_all_networks")?.unwrap_or(false);
					let third_party: Option<String> = input.query("third_party_instance_id")?;
					if all && third_party.is_some() {
						return Err(crate::codec::DeError::expected("exclusive room network"));
					}
					let value = Self {
						limit: input.query("limit")?,
						since: input.query("since")?,
						room_network: RoomNetwork::from_fields(all, third_party),
					};
					input.finish()?;
					Ok(value)
				}
			}
		}
	}

	pub mod get_public_rooms_filtered {
		pub mod v1 {
			#[derive(Debug, Default)]
			pub struct Request {
				pub limit: Option<crate::UInt>,
				pub since: Option<::alloc::string::String>,
				pub filter: crate::directory::Filter,
				pub room_network: crate::directory::RoomNetwork,
			}
			const _: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/federation/v1/publicRooms");
			impl crate::endpoint::EndpointRequest for Request {
				type Response = Response;
				const METADATA: crate::endpoint::Metadata =
					crate::endpoint::Metadata::new("POST", "/_matrix/federation/v1/publicRooms");
				fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
					::alloc::vec::Vec::new()
				}
				fn query(
					&self,
				) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)>
				{
					crate::endpoint::query_pairs(::alloc::vec![])
				}
				fn body(&self) -> Option<crate::json::Value> {
					let (include_all, third_party) = self.room_network.fields();
					Some(crate::json::Value::Object(crate::endpoint::object_from(::alloc::vec![
						("limit", crate::codec::Serialize::to_json(&self.limit)),
						("since", crate::codec::Serialize::to_json(&self.since)),
						("filter", crate::codec::Serialize::to_json(&self.filter)),
						("include_all_networks", crate::codec::Serialize::to_json(&include_all)),
						(
							"third_party_instance_id",
							crate::codec::Serialize::to_json(&third_party)
						),
					])))
				}
				fn from_parts(
					path: &[::alloc::string::String],
					query: &[(::alloc::string::String, ::alloc::string::String)],
					body: Option<&crate::json::Value>,
				) -> Result<Self, crate::codec::DeError> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						limit: input.body("limit")?,
						since: input.body("since")?,
						filter: input.body_or_default("filter")?,
						room_network: crate::directory::RoomNetwork::from_fields(
							input.body_or_default("include_all_networks")?,
							input.body("third_party_instance_id")?,
						),
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub chunk: ::alloc::vec::Vec<crate::directory::PublicRoomsChunk>,
				pub next_batch: Option<::alloc::string::String>,
				pub prev_batch: Option<::alloc::string::String>,
				pub total_room_count_estimate: Option<crate::UInt>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("chunk", crate::endpoint::enc(&self.chunk)),
						("next_batch", crate::endpoint::enc(&self.next_batch)),
						("prev_batch", crate::endpoint::enc(&self.prev_batch)),
						(
							"total_room_count_estimate",
							crate::endpoint::enc(&self.total_room_count_estimate),
						),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						chunk: input.body("chunk")?,
						next_batch: input.body("next_batch")?,
						prev_batch: input.body("prev_batch")?,
						total_room_count_estimate: input.body("total_room_count_estimate")?,
					})
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::endpoint::EndpointRequest;

	#[test]
	fn filtered_request_round_trips_network() {
		let request = get_public_rooms_filtered::v3::Request {
			server: None,
			limit: Some(5),
			since: None,
			filter: Filter {
				generic_search_term: Some("x".into()),
				room_types: Vec::new(),
			},
			room_network: RoomNetwork::All,
		};
		let body = request.body().unwrap();
		let back =
			get_public_rooms_filtered::v3::Request::from_parts(&[], &[], Some(&body)).unwrap();
		assert_eq!(back.room_network, RoomNetwork::All);
		assert_eq!(back.limit, Some(5));
	}

	#[test]
	fn federation_request_flattens_room_network() {
		use federation::get_public_rooms::v1::Request;

		let matrix = Request {
			limit: None,
			since: None,
			room_network: RoomNetwork::Matrix,
		};
		assert!(matrix.query().is_empty());

		let all = Request {
			limit: None,
			since: None,
			room_network: RoomNetwork::All,
		};
		assert_eq!(all.query(), [("include_all_networks".into(), "true".into())]);

		let third_party = Request {
			limit: None,
			since: None,
			room_network: RoomNetwork::ThirdParty("example.org".into()),
		};
		assert_eq!(
			third_party.query(),
			[("third_party_instance_id".into(), "example.org".into())]
		);

		let parsed =
			Request::from_parts(&[], &[("include_all_networks".into(), "true".into())], None)
				.unwrap();
		assert_eq!(parsed.room_network, RoomNetwork::All);
		assert!(
			Request::from_parts(
				&[],
				&[
					("include_all_networks".into(), "true".into()),
					("third_party_instance_id".into(), "example.org".into()),
				],
				None,
			)
			.is_err()
		);
	}

	#[test]
	fn federation_request_networks_round_trip() {
		use federation::get_public_rooms::v1::Request;

		for room_network in
			[RoomNetwork::Matrix, RoomNetwork::All, RoomNetwork::ThirdParty("example.org".into())]
		{
			let request = Request {
				limit: None,
				since: None,
				room_network,
			};
			let query = request.query();
			let decoded = Request::from_parts(&[], &query, None).unwrap();
			assert_eq!(decoded.room_network, request.room_network);
			assert!(!query.iter().any(|(key, _)| key == "room_network"));
		}
	}

	#[test]
	fn room_type_filter_null_is_default() {
		assert_eq!(RoomTypeFilter::from_json(&Value::Null).unwrap(), RoomTypeFilter::Default);
	}
}

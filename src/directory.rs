//! Public room directory types and endpoints.

use alloc::{string::String, vec::Vec};

use crate::{
	OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, UInt,
	codec::{DeError, Deserialize, Serialize},
	impl_codec_struct,
	json::Value,
	room::RoomType,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
	Public,
	Private,
}
crate::impl_codec_enum!(Visibility { Public => "public", Private => "private" });

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

impl_codec_struct!(Filter {} default { generic_search_term: Option<String>, room_types: Vec<RoomTypeFilter> });

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

crate::impl_codec_string!(PublicRoomJoinRule, |s| Ok(Self::from(s)), |v| v.as_str());

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

impl_codec_struct!(PublicRoomsChunk {
	num_joined_members: UInt,
	room_id: OwnedRoomId,
} default {
	avatar_url: Option<OwnedMxcUri>,
	canonical_alias: Option<OwnedRoomAliasId>,
	guest_can_join: bool,
	join_rule: PublicRoomJoinRule,
	name: Option<String>,
	room_type: Option<RoomType>,
	topic: Option<String>,
	world_readable: bool,
});

macro_rules! filtered_request {
	($method:literal, $path:literal $(, $server:ident)?) => {
		#[derive(Debug, Default)]
		pub struct Request {
			$(pub $server: Option<$crate::OwnedServerName>,)?
			pub limit: Option<$crate::UInt>,
			pub since: Option<::alloc::string::String>,
			pub filter: $crate::directory::Filter,
			pub room_network: $crate::directory::RoomNetwork,
		}

		const _: $crate::endpoint::Metadata = $crate::endpoint::Metadata::new($method, $path);
		impl $crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: $crate::endpoint::Metadata =
				$crate::endpoint::Metadata::new($method, $path);

			fn path_args(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
				::alloc::vec::Vec::new()
			}

			fn query(&self) -> ::alloc::vec::Vec<(::alloc::string::String, ::alloc::string::String)> {
				$crate::endpoint::query_pairs(::alloc::vec![
					$((stringify!($server), $crate::codec::Serialize::to_json(&self.$server)))?
				])
			}

			fn body(&self) -> Option<$crate::json::Value> {
				let (include_all, third_party) = self.room_network.fields();
				Some($crate::json::Value::Object($crate::endpoint::object_from(::alloc::vec![
					("limit", $crate::codec::Serialize::to_json(&self.limit)),
					("since", $crate::codec::Serialize::to_json(&self.since)),
					("filter", $crate::codec::Serialize::to_json(&self.filter)),
					("include_all_networks", $crate::codec::Serialize::to_json(&include_all)),
					("third_party_instance_id", $crate::codec::Serialize::to_json(&third_party)),
				])))
			}

			fn from_parts(
				path: &[::alloc::string::String],
				query: &[(::alloc::string::String, ::alloc::string::String)],
				body: Option<&$crate::json::Value>,
			) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(path, query, body);
				let value = Self {
					$($server: input.query(stringify!($server))?,)?
					limit: input.body("limit")?,
					since: input.body("since")?,
					filter: input.body_or_default("filter")?,
					room_network: $crate::directory::RoomNetwork::from_fields(
						input.body_or_default("include_all_networks")?,
						input.body("third_party_instance_id")?,
					),
				};
				input.finish()?;
				Ok(value)
			}
		}

		$crate::endpoint_response! {
			response {
				chunk: ::alloc::vec::Vec<$crate::directory::PublicRoomsChunk>,
				next_batch: Option<::alloc::string::String>,
				prev_batch: Option<::alloc::string::String>,
				total_room_count_estimate: Option<$crate::UInt>,
			}
		}
	};
}

pub mod get_public_rooms {
	pub mod v3 {
		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/publicRooms",
			request {
				path {}
				query { limit: Option<crate::UInt>, since: Option<alloc::string::String>, server: Option<crate::OwnedServerName> }
				body {}
			}
			response {
				chunk: alloc::vec::Vec<crate::directory::PublicRoomsChunk>,
				next_batch: Option<alloc::string::String>,
				prev_batch: Option<alloc::string::String>,
				total_room_count_estimate: Option<crate::UInt>,
			}
		}
	}
}

pub mod get_public_rooms_filtered {
	pub mod v3 {
		filtered_request!("POST", "/_matrix/client/v3/publicRooms", server);
	}
}

pub mod federation {
	pub mod get_public_rooms {
		pub mod v1 {
			use crate::{directory::RoomNetwork, json::Value};

			crate::endpoint_response! { response {
					chunk: alloc::vec::Vec<crate::directory::PublicRoomsChunk>,
					next_batch: Option<alloc::string::String>,
					prev_batch: Option<alloc::string::String>,
					total_room_count_estimate: Option<crate::UInt>,
			} }
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
					let all = input.query::<Option<bool>>("include_all_networks")?.unwrap_or(false);
					let third_party = input.query("third_party_instance_id")?;
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
			filtered_request!("POST", "/_matrix/federation/v1/publicRooms");
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

		for room_network in [RoomNetwork::Matrix, RoomNetwork::All, RoomNetwork::ThirdParty("example.org".into())] {
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

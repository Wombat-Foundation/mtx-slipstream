//! Client membership endpoints: join, leave, invite, kick, ban and member listings.
//!
//! Wire details (field names, optionality, legacy `server_name` query) follow the
//! pinned ruwuma definitions; only the `v3` paths are declared here.

use alloc::string::String;

use crate::{
	OwnedMxcUri, OwnedRoomId, OwnedRoomOrAliasId, OwnedServerName, OwnedUserId, Signatures,
	codec::{DeError, Deserialize, Serialize},
	endpoint::{EndpointRequest, EndpointResponse, Input, Metadata, object_from},
	json::Value,
	room_api::Invite3pid,
};

/// Signed token proving a third-party invite, supplied when joining.
#[derive(Debug)]
pub struct ThirdPartySigned {
	pub sender: OwnedUserId,
	pub mxid: OwnedUserId,
	pub token: String,
	pub signatures: Signatures,
}

crate::impl_codec_struct!(ThirdPartySigned {
	sender: OwnedUserId,
	mxid: OwnedUserId,
	token: String,
	signatures: Signatures,
});

/// Who an invitation is for.
#[derive(Debug)]
pub enum InvitationRecipient {
	UserId {
		user_id: OwnedUserId,
	},
	ThirdPartyId(Invite3pid),
}

/// Filters member listings by membership state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MembershipEventFilter {
	Join,
	Invite,
	Leave,
	Ban,
	Knock,
}

crate::impl_codec_enum!(MembershipEventFilter {
	Join => "join",
	Invite => "invite",
	Leave => "leave",
	Ban => "ban",
	Knock => "knock",
});

/// A joined member's profile.
#[derive(Debug, Default)]
pub struct RoomMember {
	pub display_name: Option<String>,
	pub avatar_url: Option<OwnedMxcUri>,
}

impl Serialize for RoomMember {
	fn to_json(&self) -> Value {
		Value::Object(object_from(alloc::vec![
			("display_name", self.display_name.to_json()),
			("avatar_url", self.avatar_url.to_json()),
		]))
	}
}

impl Deserialize for RoomMember {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		if value.as_object().is_none() {
			return Err(DeError::expected("room member object"));
		}
		let input = Input::new(&[], &[], Some(value));
		// Some servers send an empty string for "no avatar".
		let avatar: Option<String> = input.body_or_default("avatar_url")?;
		Ok(Self {
			display_name: input.body_or_default("display_name")?,
			avatar_url: avatar.filter(|url| !url.is_empty()).map(OwnedMxcUri::from),
		})
	}
}

/// Declares an endpoint that has no response fields.
macro_rules! empty_response_endpoint {
	(method: $method:literal, path: $path:literal, request { path { $($pf:ident : $pt:ty),* } body { $($bf:ident : $bt:ty),* } }) => {
		crate::endpoint! {
			method: $method, path: $path,
			request {
				path { $($pf : $pt),* }
				query {}
				body { $($bf : $bt),* }
			}
			response {}
		}
		impl Response {
			#[must_use]
			pub fn new() -> Self { Self {} }
		}
		impl Default for Response {
			fn default() -> Self { Self::new() }
		}
	};
}

pub mod ban_user {
	pub mod v3 {
		use alloc::string::String;

		use crate::{OwnedRoomId, OwnedUserId};

		empty_response_endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/ban",
			request {
				path { room_id: OwnedRoomId }
				body { user_id: OwnedUserId, reason: Option<String>, redact_events: Option<bool> }
			}
		}
	}
}

pub mod kick_user {
	pub mod v3 {
		use alloc::string::String;

		use crate::{OwnedRoomId, OwnedUserId};

		empty_response_endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/kick",
			request {
				path { room_id: OwnedRoomId }
				body { user_id: OwnedUserId, reason: Option<String> }
			}
		}
	}
}

pub mod unban_user {
	pub mod v3 {
		use alloc::string::String;

		use crate::{OwnedRoomId, OwnedUserId};

		empty_response_endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/unban",
			request {
				path { room_id: OwnedRoomId }
				body { user_id: OwnedUserId, reason: Option<String> }
			}
		}
	}
}

pub mod forget_room {
	pub mod v3 {
		use crate::OwnedRoomId;

		empty_response_endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/forget",
			request {
				path { room_id: OwnedRoomId }
				body {}
			}
		}
	}
}

pub mod leave_room {
	pub mod v3 {
		use alloc::string::String;

		use crate::OwnedRoomId;

		empty_response_endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/leave",
			request {
				path { room_id: OwnedRoomId }
				body { reason: Option<String> }
			}
		}
	}
}

pub mod joined_rooms {
	pub mod v1 {}
	pub mod v3 {
		use alloc::vec::Vec;

		use crate::OwnedRoomId;

		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/joined_rooms",
			request {
				path {}
				query {}
				body {}
			}
			response { joined_rooms: Vec<OwnedRoomId> }
		}
	}
}

pub mod joined_members {
	pub mod v3 {
		use alloc::collections::BTreeMap;

		use super::super::RoomMember;
		use crate::{OwnedRoomId, OwnedUserId};

		pub use super::super::RoomMember as Member;

		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/rooms/{room_id}/joined_members",
			request {
				path { room_id: OwnedRoomId }
				query {}
				body {}
			}
			response { joined: BTreeMap<OwnedUserId, RoomMember> }
		}
	}
}

pub mod get_member_events {
	pub mod v3 {
		use alloc::string::String;
		use alloc::vec::Vec;

		pub use super::super::MembershipEventFilter;
		use crate::{OwnedRoomId, serde::RawJsonValue};

		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/rooms/{room_id}/members",
			request {
				path { room_id: OwnedRoomId }
				query {
					at: Option<String>,
					membership: Option<MembershipEventFilter>,
					not_membership: Option<MembershipEventFilter>
				}
				body {}
			}
			response { chunk: Vec<RawJsonValue> }
		}
	}
}

pub mod join_room_by_id {
	pub mod v3 {
		use alloc::string::String;

		use super::super::ThirdPartySigned;
		use crate::OwnedRoomId;

		crate::endpoint! {
			method: "POST", path: "/_matrix/client/v3/rooms/{room_id}/join",
			request {
				path { room_id: OwnedRoomId }
				query {}
				body { third_party_signed: Option<ThirdPartySigned>, reason: Option<String> }
			}
			response { room_id: OwnedRoomId }
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

pub mod invite_user {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		pub use super::super::InvitationRecipient;
		use super::super::{
			DeError, Deserialize, EndpointRequest, EndpointResponse, Input, Invite3pid, Metadata,
			OwnedRoomId, Serialize, Value, object_from,
		};

		const _: Metadata = Metadata::new("POST", "/_matrix/client/v3/rooms/{room_id}/invite");

		/// An invitation, by user ID or by third-party identifier.
		#[derive(Debug)]
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub recipient: InvitationRecipient,
			pub reason: Option<String>,
		}

		impl Request {
			#[must_use]
			pub fn new(room_id: OwnedRoomId, recipient: InvitationRecipient) -> Self {
				Self {
					room_id,
					recipient,
					reason: None,
				}
			}
		}

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata =
				Metadata::new("POST", "/_matrix/client/v3/rooms/{room_id}/invite");

			fn path_args(&self) -> Vec<String> {
				alloc::vec![crate::endpoint::to_param(&self.room_id).unwrap_or_default()]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				let mut fields = match &self.recipient {
					InvitationRecipient::UserId {
						user_id,
					} => alloc::vec![("user_id", user_id.to_json())],
					InvitationRecipient::ThirdPartyId(third_party) => {
						let Value::Object(object) = third_party.to_json() else {
							return None;
						};
						return Some(Value::Object({
							let mut object = object;
							if let Some(reason) = &self.reason {
								object.insert("reason".into(), reason.to_json());
							}
							object
						}));
					}
				};
				fields.push(("reason", self.reason.to_json()));
				Some(Value::Object(object_from(fields)))
			}

			fn from_parts(
				path: &[String],
				_query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let body = body.ok_or_else(|| DeError::expected("request body"))?;
				let input = Input::new(path, &[], Some(body));
				let room_id = input.path()?;
				let recipient = if body.get("user_id").is_some() {
					InvitationRecipient::UserId {
						user_id: input.body("user_id")?,
					}
				} else {
					InvitationRecipient::ThirdPartyId(Invite3pid::from_json(body)?)
				};
				Ok(Self {
					room_id,
					recipient,
					reason: input.body_or_default("reason")?,
				})
			}
		}

		/// Empty response.
		#[derive(Debug, Default)]
		pub struct Response {}

		impl Response {
			#[must_use]
			pub fn new() -> Self {
				Self {}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(crate::json::Object::default())
			}

			fn from_body(_body: &Value) -> Result<Self, DeError> {
				Ok(Self {})
			}
		}
	}
}

pub mod join_room_by_id_or_alias {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use super::super::{
			DeError, EndpointRequest, EndpointResponse, Input, Metadata, OwnedRoomId,
			OwnedRoomOrAliasId, OwnedServerName, Serialize, ThirdPartySigned, Value, object_from,
		};

		const _: Metadata = Metadata::new("POST", "/_matrix/client/v3/join/{room_id_or_alias}");

		/// Joins by room ID or alias. `via` is sent as both `via` and the legacy
		/// `server_name` query parameter, and read from either.
		#[derive(Debug)]
		pub struct Request {
			pub room_id_or_alias: OwnedRoomOrAliasId,
			pub third_party_signed: Option<ThirdPartySigned>,
			pub reason: Option<String>,
			pub via: Vec<OwnedServerName>,
		}

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata =
				Metadata::new("POST", "/_matrix/client/v3/join/{room_id_or_alias}");

			fn path_args(&self) -> Vec<String> {
				alloc::vec![crate::endpoint::to_param(&self.room_id_or_alias).unwrap_or_default()]
			}

			fn query(&self) -> Vec<(String, String)> {
				crate::endpoint::query_pairs(alloc::vec![
					("server_name", self.via.to_json()),
					("via", self.via.to_json()),
				])
			}

			fn body(&self) -> Option<Value> {
				Some(Value::Object(object_from(alloc::vec![
					("third_party_signed", self.third_party_signed.to_json()),
					("reason", self.reason.to_json()),
				])))
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = Input::new(path, query, body);
				let mut via: Vec<OwnedServerName> = if query.iter().any(|(k, _)| k == "via") {
					input.query("via")?
				} else {
					Vec::new()
				};
				if query.iter().any(|(k, _)| k == "server_name") {
					let legacy: Vec<OwnedServerName> = input.query("server_name")?;
					for server in legacy {
						if !via.contains(&server) {
							via.push(server);
						}
					}
				}
				Ok(Self {
					room_id_or_alias: input.path()?,
					third_party_signed: if body.is_some() {
						input.body_or_default("third_party_signed")?
					} else {
						None
					},
					reason: if body.is_some() {
						input.body_or_default("reason")?
					} else {
						None
					},
					via,
				})
			}
		}

		/// The room that was joined.
		#[derive(Debug)]
		pub struct Response {
			pub room_id: OwnedRoomId,
		}

		impl Response {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(object_from(alloc::vec![("room_id", self.room_id.to_json())]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				Ok(Self {
					room_id: Input::new(&[], &[], Some(body)).body("room_id")?,
				})
			}
		}
	}
}

pub mod knock_room {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use super::super::{
			DeError, EndpointRequest, EndpointResponse, Input, Metadata, OwnedRoomId,
			OwnedRoomOrAliasId, OwnedServerName, Serialize, Value, object_from,
		};

		const _: Metadata = Metadata::new("POST", "/_matrix/client/v3/knock/{room_id_or_alias}");

		/// Knocks on a room. `via` is sent as both `via` and the legacy `server_name`
		/// query parameter, and read from either.
		#[derive(Debug)]
		pub struct Request {
			pub room_id_or_alias: OwnedRoomOrAliasId,
			pub reason: Option<String>,
			pub via: Vec<OwnedServerName>,
		}

		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata =
				Metadata::new("POST", "/_matrix/client/v3/knock/{room_id_or_alias}");

			fn path_args(&self) -> Vec<String> {
				alloc::vec![crate::endpoint::to_param(&self.room_id_or_alias).unwrap_or_default()]
			}

			fn query(&self) -> Vec<(String, String)> {
				crate::endpoint::query_pairs(alloc::vec![
					("server_name", self.via.to_json()),
					("via", self.via.to_json()),
				])
			}

			fn body(&self) -> Option<Value> {
				Some(Value::Object(object_from(alloc::vec![("reason", self.reason.to_json())])))
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = Input::new(path, query, body);
				let mut via: Vec<OwnedServerName> = if query.iter().any(|(k, _)| k == "via") {
					input.query("via")?
				} else {
					Vec::new()
				};
				if query.iter().any(|(k, _)| k == "server_name") {
					for server in input.query::<Vec<OwnedServerName>>("server_name")? {
						if !via.contains(&server) {
							via.push(server);
						}
					}
				}
				Ok(Self {
					room_id_or_alias: input.path()?,
					reason: if body.is_some() {
						input.body_or_default("reason")?
					} else {
						None
					},
					via,
				})
			}
		}

		/// The room that was knocked on.
		#[derive(Debug)]
		pub struct Response {
			pub room_id: OwnedRoomId,
		}

		impl Response {
			#[must_use]
			pub fn new(room_id: OwnedRoomId) -> Self {
				Self {
					room_id,
				}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(object_from(alloc::vec![("room_id", self.room_id.to_json())]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				Ok(Self {
					room_id: Input::new(&[], &[], Some(body)).body("room_id")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::vec;

	use super::*;
	use crate::codec::from_str;

	fn json(text: &str) -> crate::json::Value {
		from_str(text).unwrap()
	}

	#[test]
	fn invite_by_user_id_round_trips() {
		let request = invite_user::v3::Request {
			room_id: OwnedRoomId::from("!r:example.org"),
			recipient: InvitationRecipient::UserId {
				user_id: OwnedUserId::from("@carl:example.org"),
			},
			reason: Some("hi".into()),
		};
		let body = request.body().unwrap();
		assert_eq!(body, json(r#"{"user_id": "@carl:example.org", "reason": "hi"}"#));
		let back = invite_user::v3::Request::from_parts(
			&["!r:example.org".to_owned()],
			&[],
			Some(&body),
		)
		.unwrap();
		assert!(matches!(back.recipient, InvitationRecipient::UserId { .. }));
		assert_eq!(back.reason.as_deref(), Some("hi"));
	}

	#[test]
	fn invite_by_third_party_id_parses() {
		let body: Value = from_str(
			r#"{"id_server":"example.org","id_access_token":"tok","medium":"email","address":"carl@example.org"}"#,
		)
		.unwrap();
		let request = invite_user::v3::Request::from_parts(
			&["!r:example.org".to_owned()],
			&[],
			Some(&body),
		)
		.unwrap();
		let InvitationRecipient::ThirdPartyId(third_party) = request.recipient else {
			panic!("expected a third-party recipient");
		};
		assert_eq!(third_party.medium, "email");
		assert_eq!(third_party.address, "carl@example.org");
	}

	#[test]
	fn join_by_alias_merges_via_and_legacy_server_name() {
		let query = vec![
			("via".to_owned(), "a.example".to_owned()),
			("server_name".to_owned(), "b.example".to_owned()),
			("server_name".to_owned(), "a.example".to_owned()),
		];
		let body = json(r#"{"reason": "because"}"#);
		let request = join_room_by_id_or_alias::v3::Request::from_parts(
			&["#room:example.org".to_owned()],
			&query,
			Some(&body),
		)
		.unwrap();
		let via: Vec<&str> = request.via.iter().map(crate::OwnedServerName::as_str).collect();
		assert_eq!(via, ["a.example", "b.example"]);
		assert_eq!(request.reason.as_deref(), Some("because"));
		assert!(request.third_party_signed.is_none());
	}

	#[test]
	fn joined_member_empty_avatar_is_none() {
		let member: RoomMember = from_str(r#"{"display_name":"Carl","avatar_url":""}"#).unwrap();
		assert_eq!(member.display_name.as_deref(), Some("Carl"));
		assert!(member.avatar_url.is_none());
	}

	#[test]
	fn knock_merges_via_and_legacy_server_name() {
		let query = vec![
			("server_name".to_owned(), "legacy.example".to_owned()),
			("via".to_owned(), "new.example".to_owned()),
		];
		let request = knock_room::v3::Request::from_parts(
			&["!r:example.org".to_owned()],
			&query,
			Some(&json(r#"{"reason": "let me in"}"#)),
		)
		.unwrap();
		let via: Vec<&str> = request.via.iter().map(crate::OwnedServerName::as_str).collect();
		assert_eq!(via, ["new.example", "legacy.example"]);
		assert_eq!(request.reason.as_deref(), Some("let me in"));
	}

	#[test]
	fn membership_filter_uses_lowercase_names() {
		assert_eq!(MembershipEventFilter::Knock.to_json(), json(r#""knock""#));
		let parsed: MembershipEventFilter = from_str("\"leave\"").unwrap();
		assert_eq!(parsed, MembershipEventFilter::Leave);
	}
}

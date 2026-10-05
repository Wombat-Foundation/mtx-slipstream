//! Federation query endpoints: profiles and room aliases.

pub mod get_room_information {
	pub mod v1 {
		use alloc::vec::Vec;

		use crate::{OwnedRoomAliasId, OwnedRoomId, OwnedServerName, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/federation/v1/query/directory",
			request {
				path {}
				query { room_alias: OwnedRoomAliasId }
				body {}
			}
			response { room_id: OwnedRoomId, servers: Vec<OwnedServerName> }
		}

		impl Response {
			#[must_use]
			pub fn new(room_id: OwnedRoomId, servers: Vec<OwnedServerName>) -> Self {
				Self {
					room_id,
					servers,
				}
			}
		}
	}
}

pub mod get_profile_information {
	pub mod v1 {
		use alloc::{collections::BTreeMap, string::String};
		use core::fmt;

		use crate::{
			OwnedMxcUri, OwnedUserId,
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointResponse, Input},
			json::{Object, Value},
		};

		/// A single profile field to query.
		#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
		pub enum ProfileField {
			DisplayName,
			AvatarUrl,
			/// Any other profile key.
			_Custom(String),
		}

		impl ProfileField {
			#[must_use]
			pub fn as_str(&self) -> &str {
				match self {
					Self::DisplayName => "displayname",
					Self::AvatarUrl => "avatar_url",
					Self::_Custom(key) => key,
				}
			}
		}

		impl fmt::Display for ProfileField {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.write_str(self.as_str())
			}
		}

		impl From<&str> for ProfileField {
			fn from(value: &str) -> Self {
				match value {
					"displayname" => Self::DisplayName,
					"avatar_url" => Self::AvatarUrl,
					other => Self::_Custom(other.into()),
				}
			}
		}

		impl Serialize for ProfileField {
			fn to_json(&self) -> Value {
				Value::String(self.as_str().into())
			}
		}

		impl Deserialize for ProfileField {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				value.as_str().map(Self::from).ok_or_else(|| DeError::expected("profile field"))
			}
		}

		crate::endpoint_request! {
			method: "GET", path: "/_matrix/federation/v1/query/profile",
			request {
				path {}
				query { user_id: OwnedUserId, field: Option<ProfileField> }
				body {}
			}
		}

		/// A profile; unknown keys are kept in `custom_profile_fields`.
		#[derive(Debug, Default)]
		pub struct Response {
			pub displayname: Option<String>,
			pub avatar_url: Option<OwnedMxcUri>,
			pub blurhash: Option<String>,
			pub custom_profile_fields: BTreeMap<String, Value>,
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				let mut object: Object = self.custom_profile_fields.clone().into_iter().collect();
				for (key, value) in [
					("displayname", self.displayname.to_json()),
					("avatar_url", self.avatar_url.to_json()),
					("xyz.amorgan.blurhash", self.blurhash.to_json()),
				] {
					if !value.is_null() {
						object.insert(key.into(), value);
					}
				}
				Value::Object(object)
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let input = Input::new(&[], &[], Some(body));
				let object = body.as_object().ok_or_else(|| DeError::expected("object"))?;
				let custom_profile_fields = object
					.iter()
					.filter(|(key, _)| {
						!matches!(
							key.as_str(),
							"displayname" | "avatar_url" | "xyz.amorgan.blurhash"
						)
					})
					.map(|(key, value)| (key.clone(), value.clone()))
					.collect();
				let response = Self {
					displayname: input.body("displayname")?,
					avatar_url: input.body("avatar_url")?,
					blurhash: input.body("xyz.amorgan.blurhash")?,
					custom_profile_fields,
				};
				input.finish()?;
				Ok(response)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		codec::from_str,
		endpoint::{EndpointResponse, IncomingResponse},
		federation_api::query::get_profile_information::v1::{ProfileField, Response},
	};

	#[test]
	fn profile_response_keeps_custom_fields() {
		let body = r#"{"displayname":"Bob","avatar_url":"mxc://b/x","m.tz":"UTC"}"#;
		let response =
			http::Response::builder().status(200).body(body.as_bytes().to_vec()).unwrap();
		let profile = Response::try_from_http_response(response).unwrap();
		assert_eq!(profile.displayname.as_deref(), Some("Bob"));
		assert_eq!(profile.custom_profile_fields.len(), 1);
		let again = profile.to_body();
		assert_eq!(crate::codec::to_string(&again).matches("m.tz").count(), 1);
		assert_eq!(
			from_str::<ProfileField>("\"displayname\"").unwrap(),
			ProfileField::DisplayName
		);
	}
}

pub use crate::client_api::profile_keys::{delete_profile_key, get_profile_key, set_profile_key};

/// MSC2448's unstable prefix for the avatar `BlurHash`.
pub(crate) const BLURHASH: &str = "xyz.amorgan.blurhash";

/// Keys that `get_profile` renders itself, so they must not be swallowed by
/// the `custom_profile_fields` flatten (ruwuma's `#[serde(flatten)]`).
const RESERVED_PROFILE_KEYS: [&str; 2] = ["avatar_url", "displayname"];

pub mod get_profile {
	pub mod v3 {
		use crate::{
			OwnedMxcUri, OwnedUserId,
			codec::{DeError, Deserialize, Serialize},
			endpoint,
			json::{Object, Value},
		};
		use std::collections::BTreeMap;

		#[derive(Debug, Default)]
		pub struct Response {
			pub avatar_url: Option<OwnedMxcUri>,
			pub displayname: Option<String>,
			/// MSC2448 `BlurHash`, serialized as `xyz.amorgan.blurhash`.
			pub blurhash: Option<String>,
			/// MSC4133 custom fields, flattened into the response object.
			pub custom_profile_fields: BTreeMap<String, Value>,
		}

		impl Response {
			#[must_use]
			pub fn new(avatar_url: Option<OwnedMxcUri>, displayname: Option<String>) -> Self {
				Self {
					avatar_url,
					displayname,
					..Self::default()
				}
			}
		}

		impl Serialize for Response {
			fn to_json(&self) -> Value {
				let mut object = Object::new();

				// Each optional field is written only when present, matching
				// ruwuma's `skip_serializing_if = "Option::is_none"`.
				if let Some(avatar_url) = &self.avatar_url {
					object.insert("avatar_url".into(), avatar_url.to_json());
				}
				if let Some(displayname) = &self.displayname {
					object.insert("displayname".into(), Value::String(displayname.clone()));
				}
				if let Some(blurhash) = &self.blurhash {
					object.insert(
						crate::client_api::profile::BLURHASH.into(),
						Value::String(blurhash.clone()),
					);
				}

				// Flattened fields sit alongside the known ones, so a reserved key
				// cannot shadow them.
				for (key, value) in &self.custom_profile_fields {
					if crate::client_api::profile::RESERVED_PROFILE_KEYS.contains(&key.as_str()) {
						continue;
					}
					object.insert(key.clone(), value.clone());
				}

				Value::Object(object)
			}
		}

		impl endpoint::EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Serialize::to_json(self)
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let object =
					body.as_object().ok_or_else(|| DeError::expected("get profile response"))?;

				Ok(Self {
					avatar_url: Deserialize::from_json(
						object.get("avatar_url").unwrap_or(&Value::Null),
					)?,
					displayname: Deserialize::from_json(
						object.get("displayname").unwrap_or(&Value::Null),
					)?,
					blurhash: Deserialize::from_json(
						object.get(crate::client_api::profile::BLURHASH).unwrap_or(&Value::Null),
					)?,
					custom_profile_fields: object
						.iter()
						.filter(|(key, _)| {
							key.as_str() != "avatar_url"
								&& key.as_str() != "displayname"
								&& key.as_str() != crate::client_api::profile::BLURHASH
						})
						.map(|(key, value)| (key.clone(), value.clone()))
						.collect(),
				})
			}
		}

		#[derive(Debug)]
		pub struct Request {
			pub user_id: OwnedUserId,
		}

		impl Request {
			#[must_use]
			pub fn new(user_id: OwnedUserId) -> Self {
				Self {
					user_id,
				}
			}
		}

		impl endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata =
				endpoint::Metadata::new("GET", "/_matrix/client/v3/profile/{userId}");

			fn path_args(&self) -> Vec<String> {
				alloc::vec![self.user_id.to_string()]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				None
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = endpoint::Input::new(path, query, body);
				let value = Self {
					user_id: input.path()?,
				};
				input.finish()?;

				Ok(value)
			}
		}
	}
}

pub mod get_display_name {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/client/v3/profile/{userId}/displayname",
			request { path { user_id: OwnedUserId } query {} body {} }
			response { displayname: Option<String> }
		}
	}
}

pub mod set_display_name {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};

		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/profile/{userId}/displayname",
			request { path { user_id: OwnedUserId } query {} body { displayname: Option<String> } }
			response {}
		}
	}
}

pub mod get_avatar_url {
	pub mod v3 {
		use crate::{
			OwnedUserId,
			codec::{DeError, Deserialize, Serialize},
			endpoint,
			json::{Object, Value},
		};

		#[derive(Debug, Default)]
		pub struct Response {
			pub avatar_url: Option<crate::OwnedMxcUri>,
			/// MSC2448 `BlurHash`, serialized as `xyz.amorgan.blurhash`.
			pub blurhash: Option<String>,
		}

		impl Response {
			#[must_use]
			pub fn new(avatar_url: Option<crate::OwnedMxcUri>) -> Self {
				Self {
					avatar_url,
					blurhash: None,
				}
			}
		}

		impl Serialize for Response {
			fn to_json(&self) -> Value {
				let mut object = Object::new();

				if let Some(avatar_url) = &self.avatar_url {
					object.insert("avatar_url".into(), avatar_url.to_json());
				}
				if let Some(blurhash) = &self.blurhash {
					object.insert(
						crate::client_api::profile::BLURHASH.into(),
						Value::String(blurhash.clone()),
					);
				}

				Value::Object(object)
			}
		}

		impl endpoint::EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Serialize::to_json(self)
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let object = body
					.as_object()
					.ok_or_else(|| DeError::expected("get avatar url response"))?;

				Ok(Self {
					avatar_url: Deserialize::from_json(
						object.get("avatar_url").unwrap_or(&Value::Null),
					)?,
					blurhash: Deserialize::from_json(
						object.get(crate::client_api::profile::BLURHASH).unwrap_or(&Value::Null),
					)?,
				})
			}
		}

		#[derive(Debug)]
		pub struct Request {
			pub user_id: OwnedUserId,
		}

		impl Request {
			#[must_use]
			pub fn new(user_id: OwnedUserId) -> Self {
				Self {
					user_id,
				}
			}
		}

		impl endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata =
				endpoint::Metadata::new("GET", "/_matrix/client/v3/profile/{userId}/avatar_url");

			fn path_args(&self) -> Vec<String> {
				alloc::vec![self.user_id.to_string()]
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				None
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = endpoint::Input::new(path, query, body);
				let value = Self {
					user_id: input.path()?,
				};
				input.finish()?;

				Ok(value)
			}
		}
	}
}

pub mod set_avatar_url {
	pub mod v3 {
		use crate::{OwnedMxcUri, OwnedUserId, endpoint};

		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/profile/{userId}/avatar_url",
			request { path { user_id: OwnedUserId } query {} body { avatar_url: Option<OwnedMxcUri> } }
			response {}
		}
	}
}

//! Session endpoints: login types, login, login tokens and logout.

pub mod get_login_types {
	pub mod v3 {
		use alloc::vec::Vec;

		use crate::{
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointResponse, object_from},
			json::Value,
		};

		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/login",
			request {
				path {}
				query {}
				body {}
			}
		}

		/// `m.login.password`.
		#[derive(Clone, Debug, Default)]
		pub struct PasswordLoginType {}

		/// `m.login.application_service`.
		#[derive(Clone, Debug, Default)]
		pub struct ApplicationServiceLoginType {}

		/// `m.login.token`.
		#[derive(Clone, Debug, Default)]
		pub struct TokenLoginType {
			/// Whether the server also supports `POST /login/get_token`.
			pub get_login_token: bool,
		}

		/// A login flow the server supports.
		#[derive(Clone, Debug)]
		pub enum LoginType {
			Password(PasswordLoginType),
			ApplicationService(ApplicationServiceLoginType),
			Token(TokenLoginType),
			/// Any other flow, kept as received.
			_Custom(Value),
		}

		impl Serialize for LoginType {
			fn to_json(&self) -> Value {
				match self {
					Self::Password(_) => Value::Object(object_from(alloc::vec![(
						"type",
						"m.login.password".to_json()
					)])),
					Self::ApplicationService(_) => Value::Object(object_from(alloc::vec![(
						"type",
						"m.login.application_service".to_json()
					)])),
					Self::Token(token) => Value::Object(object_from(alloc::vec![
						("type", "m.login.token".to_json()),
						("get_login_token", token.get_login_token.to_json()),
					])),
					Self::_Custom(value) => value.clone(),
				}
			}
		}

		impl Deserialize for LoginType {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				let kind = value.get("type").and_then(Value::as_str);
				Ok(match kind {
					Some("m.login.password") => Self::Password(PasswordLoginType {}),
					Some("m.login.application_service") => {
						Self::ApplicationService(ApplicationServiceLoginType {})
					}
					Some("m.login.token") => Self::Token(TokenLoginType {
						get_login_token: value
							.get("get_login_token")
							.and_then(Value::as_bool)
							.unwrap_or(false),
					}),
					_ => Self::_Custom(value.clone()),
				})
			}
		}

		/// The supported login flows.
		#[derive(Clone, Debug)]
		pub struct Response {
			pub flows: Vec<LoginType>,
		}

		impl Response {
			#[must_use]
			pub fn new(flows: Vec<LoginType>) -> Self {
				Self {
					flows,
				}
			}
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(object_from(alloc::vec![("flows", self.flows.to_json())]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let flows = body
					.get("flows")
					.ok_or_else(|| DeError::expected("flows"))
					.and_then(Deserialize::from_json)?;
				Ok(Self {
					flows,
				})
			}
		}
	}
}

pub mod login {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};
		use core::time::Duration;

		use crate::{
			OwnedDeviceId, OwnedServerName, OwnedUserId,
			codec::{DeError, Deserialize, Serialize},
			endpoint::{EndpointRequest, EndpointResponse, Input, Metadata, object_from},
			json::Value,
			uiaa::UserIdentifier,
		};

		/// `m.login.password`.
		#[derive(Clone, Debug)]
		pub struct Password {
			pub identifier: Option<UserIdentifier>,
			pub password: String,
			/// Deprecated: the user's localpart or ID.
			pub user: Option<String>,
		}

		/// `m.login.token`.
		#[derive(Clone, Debug)]
		pub struct Token {
			pub token: String,
		}

		/// `m.login.application_service`.
		#[derive(Clone, Debug)]
		pub struct ApplicationService {
			pub identifier: Option<UserIdentifier>,
			/// Deprecated: the user's localpart or ID.
			pub user: Option<String>,
		}

		/// How the client authenticates, discriminated by `type`.
		#[derive(Clone, Debug)]
		pub enum LoginInfo {
			Password(Password),
			Token(Token),
			ApplicationService(ApplicationService),
			/// Any other login type, kept as received.
			_Custom(Value),
		}

		impl LoginInfo {
			fn fields(&self) -> Vec<(&'static str, Value)> {
				match self {
					Self::Password(login) => alloc::vec![
						("type", "m.login.password".to_json()),
						("identifier", login.identifier.to_json()),
						("password", login.password.to_json()),
						("user", login.user.to_json()),
					],
					Self::Token(login) => alloc::vec![
						("type", "m.login.token".to_json()),
						("token", login.token.to_json()),
					],
					Self::ApplicationService(login) => alloc::vec![
						("type", "m.login.application_service".to_json()),
						("identifier", login.identifier.to_json()),
						("user", login.user.to_json()),
					],
					Self::_Custom(_) => Vec::new(),
				}
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let optional = |name: &str| -> Result<Option<String>, DeError> {
					body.get(name).map_or(Ok(None), Deserialize::from_json)
				};
				let identifier = || -> Result<Option<UserIdentifier>, DeError> {
					body.get("identifier").map_or(Ok(None), Deserialize::from_json)
				};
				Ok(match body.get("type").and_then(Value::as_str) {
					Some("m.login.password") => Self::Password(Password {
						identifier: identifier()?,
						password: optional("password")?
							.ok_or_else(|| DeError::expected("password"))?,
						user: optional("user")?,
					}),
					Some("m.login.token") => Self::Token(Token {
						token: optional("token")?.ok_or_else(|| DeError::expected("token"))?,
					}),
					Some("m.login.application_service") => {
						Self::ApplicationService(ApplicationService {
							identifier: identifier()?,
							user: optional("user")?,
						})
					}
					_ => Self::_Custom(body.clone()),
				})
			}
		}

		/// A login request.
		#[derive(Clone, Debug)]
		pub struct Request {
			pub login_info: LoginInfo,
			pub device_id: Option<OwnedDeviceId>,
			pub initial_device_display_name: Option<String>,
			pub refresh_token: bool,
		}

		const _: crate::endpoint::Metadata =
			crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/login");
		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata = Metadata::new("POST", "/_matrix/client/v3/login");

			fn path_args(&self) -> Vec<String> {
				Vec::new()
			}

			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}

			fn body(&self) -> Option<Value> {
				let mut object = match &self.login_info {
					LoginInfo::_Custom(Value::Object(object)) => object.clone(),
					other => object_from(other.fields()),
				};
				let extra = [
					("device_id", self.device_id.to_json()),
					("initial_device_display_name", self.initial_device_display_name.to_json()),
					("refresh_token", self.refresh_token.then_some(true).to_json()),
				];
				for (name, value) in extra {
					if !value.is_null() {
						object.insert(name.to_string(), value);
					}
				}
				Some(Value::Object(object))
			}

			fn from_parts(
				_path: &[String],
				_query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let body = body.ok_or_else(|| DeError::expected("request body"))?;
				let input = Input::new(&[], &[], Some(body));
				Ok(Self {
					login_info: LoginInfo::from_body(body)?,
					device_id: input.body_or_default("device_id")?,
					initial_device_display_name: input
						.body_or_default("initial_device_display_name")?,
					refresh_token: input.body_or_default("refresh_token")?,
				})
			}
		}

		/// The homeserver base URL.
		#[derive(Clone, Debug)]
		pub struct HomeserverInfo {
			pub base_url: String,
		}

		impl HomeserverInfo {
			#[must_use]
			pub fn new(base_url: String) -> Self {
				Self {
					base_url,
				}
			}
		}

		crate::impl_codec_struct!(HomeserverInfo {
			base_url: String
		});

		/// The identity server base URL.
		#[derive(Clone, Debug)]
		pub struct IdentityServerInfo {
			pub base_url: String,
		}

		crate::impl_codec_struct!(IdentityServerInfo {
			base_url: String
		});

		/// Client discovery information (`m.homeserver`, `m.identity_server`).
		#[derive(Clone, Debug)]
		pub struct DiscoveryInfo {
			pub homeserver: HomeserverInfo,
			pub identity_server: Option<IdentityServerInfo>,
		}

		impl DiscoveryInfo {
			#[must_use]
			pub fn new(homeserver: HomeserverInfo) -> Self {
				Self {
					homeserver,
					identity_server: None,
				}
			}
		}

		impl Serialize for DiscoveryInfo {
			fn to_json(&self) -> Value {
				Value::Object(object_from(alloc::vec![
					("m.homeserver", self.homeserver.to_json()),
					("m.identity_server", self.identity_server.to_json()),
				]))
			}
		}

		impl Deserialize for DiscoveryInfo {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				Ok(Self {
					homeserver: value
						.get("m.homeserver")
						.ok_or_else(|| DeError::expected("m.homeserver"))
						.and_then(Deserialize::from_json)?,
					identity_server: value
						.get("m.identity_server")
						.map_or(Ok(None), Deserialize::from_json)?,
				})
			}
		}

		/// A successful login.
		#[derive(Clone, Debug)]
		pub struct Response {
			pub user_id: OwnedUserId,
			pub access_token: String,
			pub device_id: OwnedDeviceId,
			pub well_known: Option<DiscoveryInfo>,
			/// How long the access token is valid for, if it expires.
			pub expires_in: Option<Duration>,
			/// Deprecated: the server name part of `user_id`.
			pub home_server: Option<OwnedServerName>,
			pub refresh_token: Option<String>,
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				let expires_in_ms = self
					.expires_in
					.map(|duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX));
				Value::Object(object_from(alloc::vec![
					("user_id", self.user_id.to_json()),
					("access_token", self.access_token.to_json()),
					("device_id", self.device_id.to_json()),
					("well_known", self.well_known.to_json()),
					("expires_in_ms", expires_in_ms.to_json()),
					("home_server", self.home_server.to_json()),
					("refresh_token", self.refresh_token.to_json()),
				]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let input = Input::new(&[], &[], Some(body));
				let expires_in_ms: Option<u64> = input.body_or_default("expires_in_ms")?;
				Ok(Self {
					user_id: input.body("user_id")?,
					access_token: input.body("access_token")?,
					device_id: input.body("device_id")?,
					well_known: input.body_or_default("well_known")?,
					expires_in: expires_in_ms.map(Duration::from_millis),
					home_server: input.body_or_default("home_server")?,
					refresh_token: input.body_or_default("refresh_token")?,
				})
			}
		}
	}
}

pub mod get_login_token {
	pub mod v1 {
		use core::time::Duration;

		use crate::{
			codec::{DeError, Serialize},
			endpoint::{EndpointResponse, Input, object_from},
			json::Value,
		};

		crate::endpoint_request! {
			method: "POST", path: "/_matrix/client/v1/login/get_token",
			request {
				path {}
				query {}
				body { auth: Option<crate::api::client::uiaa::AuthData> }
			}
		}

		/// A short-lived token for `m.login.token`.
		#[derive(Clone, Debug)]
		pub struct Response {
			pub expires_in: Duration,
			pub login_token: alloc::string::String,
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				let millis = u64::try_from(self.expires_in.as_millis()).unwrap_or(u64::MAX);
				Value::Object(object_from(alloc::vec![
					("expires_in_ms", millis.to_json()),
					("login_token", self.login_token.to_json()),
				]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let input = Input::new(&[], &[], Some(body));
				let millis: u64 = input.body("expires_in_ms")?;
				Ok(Self {
					expires_in: Duration::from_millis(millis),
					login_token: input.body("login_token")?,
				})
			}
		}
	}
}

macro_rules! empty_post {
	($name:ident, $path:literal) => {
		pub mod $name {
			pub mod v3 {
				crate::endpoint! {
					method: "POST", path: $path,
					request {
						path {}
						query {}
						body {}
					}
					response {}
				}

				impl Response {
					#[must_use]
					pub fn new() -> Self {
						Self {}
					}
				}

				impl Default for Response {
					fn default() -> Self {
						Self::new()
					}
				}
			}
		}
	};
}

empty_post!(logout, "/_matrix/client/v3/logout");
empty_post!(logout_all, "/_matrix/client/v3/logout/all");

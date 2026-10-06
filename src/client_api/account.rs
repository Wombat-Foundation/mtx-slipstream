#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThirdPartyIdRemovalStatus {
	Success,
	NoSupport,
}
crate::impl_codec_enum!(ThirdPartyIdRemovalStatus { Success => "success", NoSupport => "no-support" });

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistrationKind {
	Guest,
	User,
}
crate::impl_codec_enum!(RegistrationKind { Guest => "guest", User => "user" });

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoginType {
	ApplicationService,
}
crate::impl_codec_enum!(LoginType { ApplicationService => "m.login.application_service" });

pub mod request_openid_token {
	pub mod v3 {
		use crate::{OwnedServerName, OwnedUserId, endpoint};
		use std::time::Duration;
		endpoint! {
			method: "POST", path: "/_matrix/client/v3/user/{userId}/openid/request_token",
			request { path { user_id: OwnedUserId } query {} body {} }
			response {
				access_token: String,
				token_type: crate::api::client::authentication::TokenType,
				matrix_server_name: OwnedServerName,
				expires_in: Duration,
			}
		}
	}
}
pub mod change_password {
	pub mod v3 {
		use crate::endpoint;
		endpoint! {
			method: "POST", path: "/_matrix/client/v3/account/password",
			request { path {} query {} body { new_password: String, logout_devices: bool = true, auth: Option<crate::uiaa::AuthData> } }
			response {}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::change_password::v3::Request;
	use crate::{
		endpoint::EndpointRequest,
		json::{Object, Value},
	};

	#[test]
	fn change_password_defaults_logout_devices_to_true() {
		let body = Value::Object(Object::from([(
			"new_password".into(),
			Value::String("secret".into()),
		)]));
		let request = Request::from_parts(&[], &[], Some(&body)).unwrap();
		assert!(request.logout_devices);
	}
}
pub mod deactivate {
	pub mod v3 {
		use crate::codec::Serialize;
		use crate::{endpoint, json::Value};

		#[derive(Debug)]
		pub struct Request {
			pub auth: Option<crate::uiaa::AuthData>,
			pub id_server: Option<String>,
			pub id_server_access_token: Option<String>,
			pub erase: bool,
		}

		impl endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata =
				endpoint::Metadata::new("POST", "/_matrix/client/v3/account/deactivate");

			fn path_args(&self) -> Vec<String> {
				Vec::new()
			}
			fn query(&self) -> Vec<(String, String)> {
				Vec::new()
			}
			fn body(&self) -> Option<Value> {
				let object = endpoint::object_from(vec![
					("auth", self.auth.to_json()),
					("id_server", self.id_server.to_json()),
					("id_server_access_token", self.id_server_access_token.to_json()),
					("erase", self.erase.to_json()),
				]);
				Some(Value::Object(object))
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, crate::codec::DeError> {
				let input = endpoint::Input::new(path, query, body);
				let value = Self {
					auth: input.body_or_default("auth")?,
					id_server: input.body_or_default("id_server")?,
					id_server_access_token: input.body_or_default("id_server_access_token")?,
					erase: input.body_or_default("erase")?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		crate::endpoint_response! { response { id_server_unbind_result: super::super::ThirdPartyIdRemovalStatus } }
	}
}
pub mod get_username_availability {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "GET", path: "/_matrix/client/v3/register/available", request { path {} query { username: String } body {} } response { available: bool } }
	}
}
pub mod whoami {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "GET", path: "/_matrix/client/v3/account/whoami", request { path {} query {} body {} } response { user_id: crate::OwnedUserId, device_id: Option<crate::OwnedDeviceId>, is_guest: bool } }
	}
}
pub mod request_password_change_token_via_email {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/account/password/email/requestToken", request { path {} query {} body { client_secret: String, email: String, send_attempt: crate::UInt } } response { sid: String } }
	}
}
pub mod request_3pid_management_token_via_email {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/account/3pid/email/requestToken", request { path {} query {} body { client_secret: String, email: String, send_attempt: crate::UInt } } response { sid: String } }
	}
}
pub mod request_3pid_management_token_via_msisdn {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/account/3pid/msisdn/requestToken", request { path {} query {} body { client_secret: String, country: String, phone_number: String, send_attempt: crate::UInt } } response { sid: String } }
	}
}
pub mod get_3pids {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "GET", path: "/_matrix/client/v3/account/3pid", request { path {} query {} body {} } response { threepids: Vec<crate::thirdparty::ThirdPartyIdentifier> } }
	}
}
pub mod add_3pid {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/account/3pid/add", request { path {} query {} body { client_secret: String, sid: String, id_server: Option<String>, id_server_access_token: Option<String>, auth: Option<crate::uiaa::AuthData> } } response {} }
	}
}
pub mod delete_3pid {
	pub mod v3 {
		use crate::endpoint;
		endpoint! {
			method: "POST", path: "/_matrix/client/v3/account/3pid/delete",
			request {
				path {} query {}
				body {
					medium: crate::thirdparty::Medium,
					address: String,
					id_server: Option<String>,
					id_server_access_token: Option<String>,
				}
			}
			response { id_server_unbind_result: super::super::ThirdPartyIdRemovalStatus }
		}
	}
}
pub mod check_registration_token_validity {
	pub mod v1 {
		use crate::endpoint;
		endpoint! {
			method: "GET", path: "/_matrix/client/v1/register/m.login.registration_token/validity",
			request { path {} query { token: String } body {} }
			response { valid: bool }
		}
	}
}
pub mod request_registration_token_via_email {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/register/email/requestToken", request { path {} query {} body { client_secret: String, email: String, send_attempt: crate::UInt } } response { sid: String } }
	}
}
pub mod register {
	pub use super::LoginType;
	pub use super::RegistrationKind;
	impl Default for super::RegistrationKind {
		fn default() -> Self {
			Self::User
		}
	}

	pub mod v3 {
		use crate::codec::Serialize;
		use crate::{endpoint, json::Value};

		#[derive(Debug)]
		pub struct Request {
			pub kind: super::super::RegistrationKind,
			pub username: Option<String>,
			pub password: Option<String>,
			pub device_id: Option<crate::OwnedDeviceId>,
			pub initial_device_display_name: Option<String>,
			pub auth: Option<crate::uiaa::AuthData>,
			pub inhibit_login: bool,
			pub login_type: Option<super::super::LoginType>,
			pub refresh_token: bool,
			pub guest_access_token: Option<String>,
		}

		impl endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: endpoint::Metadata =
				endpoint::Metadata::new("POST", "/_matrix/client/v3/register");
			fn path_args(&self) -> Vec<String> {
				Vec::new()
			}
			fn query(&self) -> Vec<(String, String)> {
				if self.kind == super::super::RegistrationKind::User {
					Vec::new()
				} else {
					endpoint::query_pairs(vec![("kind", self.kind.to_json())])
				}
			}
			fn body(&self) -> Option<Value> {
				Some(Value::Object(endpoint::object_from(vec![
					("username", self.username.to_json()),
					("password", self.password.to_json()),
					("device_id", self.device_id.to_json()),
					("initial_device_display_name", self.initial_device_display_name.to_json()),
					("auth", self.auth.to_json()),
					("inhibit_login", self.inhibit_login.to_json()),
					("type", self.login_type.to_json()),
					("refresh_token", self.refresh_token.to_json()),
					("guest_access_token", self.guest_access_token.to_json()),
				])))
			}
			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, crate::codec::DeError> {
				let input = endpoint::Input::new(path, query, body);
				let value = Self {
					kind: input
						.query("kind")
						.or_else(|_| Ok(super::super::RegistrationKind::default()))?,
					username: input.body_or_default("username")?,
					password: input.body_or_default("password")?,
					device_id: input.body_or_default("device_id")?,
					initial_device_display_name: input
						.body_or_default("initial_device_display_name")?,
					auth: input.body_or_default("auth")?,
					inhibit_login: input.body_or_default("inhibit_login")?,
					login_type: input.body_or_default("type")?,
					refresh_token: input.body_or_default("refresh_token")?,
					guest_access_token: input.body_or_default("guest_access_token")?,
				};
				input.finish()?;
				Ok(value)
			}
		}

		#[derive(Debug)]
		pub struct Response {
			pub user_id: crate::OwnedUserId,
			pub access_token: Option<String>,
			pub device_id: Option<crate::OwnedDeviceId>,
			pub refresh_token: Option<String>,
			pub expires_in: Option<std::time::Duration>,
		}
		impl endpoint::EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(endpoint::object_from(vec![
					("user_id", self.user_id.to_json()),
					("access_token", self.access_token.to_json()),
					("device_id", self.device_id.to_json()),
					("refresh_token", self.refresh_token.to_json()),
					(
						"expires_in_ms",
						self.expires_in
							.map(|v| {
								crate::UInt::try_from(v.as_millis()).unwrap_or(crate::UInt::MAX)
							})
							.to_json(),
					),
				]))
			}
			fn from_body(body: &Value) -> Result<Self, crate::codec::DeError> {
				let input = endpoint::Input::new(&[], &[], Some(body));
				let expires_in_ms: Option<crate::UInt> =
					input.body_or_default("expires_in_ms")?;
				let value = Self {
					user_id: input.body("user_id")?,
					access_token: input.body_or_default("access_token")?,
					device_id: input.body_or_default("device_id")?,
					refresh_token: input.body_or_default("refresh_token")?,
					expires_in: expires_in_ms.map(std::time::Duration::from_millis),
				};
				input.finish()?;
				Ok(value)
			}
		}
	}
}

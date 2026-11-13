#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThirdPartyIdRemovalStatus {
	Success,
	NoSupport,
}
impl crate::codec::Serialize for ThirdPartyIdRemovalStatus {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Success => "success",
			Self::NoSupport => "no-support",
		}))
	}
}
impl crate::codec::Deserialize for ThirdPartyIdRemovalStatus {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("success") => Ok(Self::Success),
			Some("no-support") => Ok(Self::NoSupport),
			_ => Err(crate::codec::DeError::expected(stringify!(ThirdPartyIdRemovalStatus))),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistrationKind {
	Guest,
	User,
}
impl crate::codec::Serialize for RegistrationKind {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Guest => "guest",
			Self::User => "user",
		}))
	}
}
impl crate::codec::Deserialize for RegistrationKind {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("guest") => Ok(Self::Guest),
			Some("user") => Ok(Self::User),
			_ => Err(crate::codec::DeError::expected(stringify!(RegistrationKind))),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoginType {
	ApplicationService,
}
impl crate::codec::Serialize for LoginType {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::ApplicationService => "m.login.application_service",
		}))
	}
}
impl crate::codec::Deserialize for LoginType {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("m.login.application_service") => Ok(Self::ApplicationService),
			_ => Err(crate::codec::DeError::expected(stringify!(LoginType))),
		}
	}
}

pub mod request_openid_token {
	pub mod v3 {
		use crate::{OwnedServerName, OwnedUserId};
		use std::time::Duration;
		pub struct Request {
			pub user_id: OwnedUserId,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/user/{userId}/openid/request_token",
			path { user_id }
			query {}
			body {}
		}
		pub struct Response {
			pub access_token: String,
			pub token_type: crate::api::client::authentication::TokenType,
			pub matrix_server_name: OwnedServerName,
			pub expires_in: Duration,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { access_token => "access_token", token_type => "token_type", matrix_server_name => "matrix_server_name", expires_in => "expires_in" }
		}
	}
}
pub mod change_password {
	pub mod v3 {

		pub struct Request {
			pub new_password: String,
			pub logout_devices: bool,
			pub auth: Option<crate::uiaa::AuthData>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/account/password",
			path {}
			query {}
			body { new_password => "new_password", logout_devices => "logout_devices" = true, auth => "auth" }
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body {}
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

		pub struct Response {
			pub id_server_unbind_result: super::super::ThirdPartyIdRemovalStatus,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { id_server_unbind_result => "id_server_unbind_result" }
		}
	}
}
pub mod get_username_availability {
	pub mod v3 {

		pub struct Request {
			pub username: String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/register/available",
			path {}
			query { username => "username" }
			body {}
		}
		pub struct Response {
			pub available: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { available => "available" }
		}
	}
}
pub mod whoami {
	pub mod v3 {

		pub struct Request {}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/account/whoami",
			path {}
			query {}
			body {}
		}
		pub struct Response {
			pub user_id: crate::OwnedUserId,
			pub device_id: Option<crate::OwnedDeviceId>,
			pub is_guest: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { user_id => "user_id", device_id => "device_id", is_guest => "is_guest" }
		}
	}
}
pub mod request_password_change_token_via_email {
	pub mod v3 {

		pub struct Request {
			pub client_secret: String,
			pub email: String,
			pub send_attempt: crate::UInt,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/account/password/email/requestToken",
			path {}
			query {}
			body { client_secret => "client_secret", email => "email", send_attempt => "send_attempt" }
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { sid => "sid" }
		}
	}
}
pub mod request_3pid_management_token_via_email {
	pub mod v3 {

		pub struct Request {
			pub client_secret: String,
			pub email: String,
			pub send_attempt: crate::UInt,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/account/3pid/email/requestToken",
			path {}
			query {}
			body { client_secret => "client_secret", email => "email", send_attempt => "send_attempt" }
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { sid => "sid" }
		}
	}
}
pub mod request_3pid_management_token_via_msisdn {
	pub mod v3 {

		pub struct Request {
			pub client_secret: String,
			pub country: String,
			pub phone_number: String,
			pub send_attempt: crate::UInt,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/account/3pid/msisdn/requestToken",
			path {}
			query {}
			body { client_secret => "client_secret", country => "country", phone_number => "phone_number", send_attempt => "send_attempt" }
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { sid => "sid" }
		}
	}
}
pub mod get_3pids {
	pub mod v3 {

		pub struct Request {}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/account/3pid",
			path {}
			query {}
			body {}
		}
		pub struct Response {
			pub threepids: Vec<crate::thirdparty::ThirdPartyIdentifier>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { threepids => "threepids" }
		}
	}
}
pub mod add_3pid {
	pub mod v3 {

		pub struct Request {
			pub client_secret: String,
			pub sid: String,
			pub id_server: Option<String>,
			pub id_server_access_token: Option<String>,
			pub auth: Option<crate::uiaa::AuthData>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/account/3pid/add",
			path {}
			query {}
			body { client_secret => "client_secret", sid => "sid", id_server => "id_server", id_server_access_token => "id_server_access_token", auth => "auth" }
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body {}
		}
	}
}
pub mod delete_3pid {
	pub mod v3 {

		pub struct Request {
			pub medium: crate::thirdparty::Medium,
			pub address: String,
			pub id_server: Option<String>,
			pub id_server_access_token: Option<String>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/account/3pid/delete",
			path {}
			query {}
			body { medium => "medium", address => "address", id_server => "id_server", id_server_access_token => "id_server_access_token" }
		}
		pub struct Response {
			pub id_server_unbind_result: super::super::ThirdPartyIdRemovalStatus,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { id_server_unbind_result => "id_server_unbind_result" }
		}
	}
}
pub mod check_registration_token_validity {
	pub mod v1 {

		pub struct Request {
			pub token: String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v1/register/m.login.registration_token/validity",
			path {}
			query { token => "token" }
			body {}
		}
		pub struct Response {
			pub valid: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { valid => "valid" }
		}
	}
}
pub mod request_registration_token_via_email {
	pub mod v3 {

		pub struct Request {
			pub client_secret: String,
			pub email: String,
			pub send_attempt: crate::UInt,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/register/email/requestToken",
			path {}
			query {}
			body { client_secret => "client_secret", email => "email", send_attempt => "send_attempt" }
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { sid => "sid" }
		}
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

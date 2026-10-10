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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/user/{userId}/openid/request_token",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.user_id)])
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
					user_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("access_token", crate::endpoint::enc(&self.access_token)),
					("token_type", crate::endpoint::enc(&self.token_type)),
					("matrix_server_name", crate::endpoint::enc(&self.matrix_server_name)),
					("expires_in", crate::endpoint::enc(&self.expires_in)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					access_token: input.body("access_token")?,
					token_type: input.body("token_type")?,
					matrix_server_name: input.body("matrix_server_name")?,
					expires_in: input.body("expires_in")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/account/password");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("new_password", crate::endpoint::enc(&self.new_password)),
						("logout_devices", crate::endpoint::enc(&self.logout_devices)),
						("auth", crate::endpoint::enc(&self.auth)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					new_password: input.body("new_password")?,
					logout_devices: input.body_or("logout_devices", true)?,
					auth: input.body("auth")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [])
			}
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"id_server_unbind_result",
					crate::endpoint::enc(&self.id_server_unbind_result),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					id_server_unbind_result: input.body("id_server_unbind_result")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/register/available");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"username",
					crate::endpoint::enc(&self.username),
				)])
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
					username: input.query("username")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub available: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"available",
					crate::endpoint::enc(&self.available),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					available: input.body("available")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/account/whoami");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
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
				let value = Self {};
				input.finish()?;
				Ok(value)
			}
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("user_id", crate::endpoint::enc(&self.user_id)),
					("device_id", crate::endpoint::enc(&self.device_id)),
					("is_guest", crate::endpoint::enc(&self.is_guest)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					user_id: input.body("user_id")?,
					device_id: input.body("device_id")?,
					is_guest: input.body("is_guest")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/account/password/email/requestToken",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("client_secret", crate::endpoint::enc(&self.client_secret)),
						("email", crate::endpoint::enc(&self.email)),
						("send_attempt", crate::endpoint::enc(&self.send_attempt)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					client_secret: input.body("client_secret")?,
					email: input.body("email")?,
					send_attempt: input.body("send_attempt")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("sid", crate::endpoint::enc(&self.sid))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					sid: input.body("sid")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/account/3pid/email/requestToken",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("client_secret", crate::endpoint::enc(&self.client_secret)),
						("email", crate::endpoint::enc(&self.email)),
						("send_attempt", crate::endpoint::enc(&self.send_attempt)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					client_secret: input.body("client_secret")?,
					email: input.body("email")?,
					send_attempt: input.body("send_attempt")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("sid", crate::endpoint::enc(&self.sid))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					sid: input.body("sid")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/account/3pid/msisdn/requestToken",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("client_secret", crate::endpoint::enc(&self.client_secret)),
						("country", crate::endpoint::enc(&self.country)),
						("phone_number", crate::endpoint::enc(&self.phone_number)),
						("send_attempt", crate::endpoint::enc(&self.send_attempt)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					client_secret: input.body("client_secret")?,
					country: input.body("country")?,
					phone_number: input.body("phone_number")?,
					send_attempt: input.body("send_attempt")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("sid", crate::endpoint::enc(&self.sid))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					sid: input.body("sid")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/account/3pid");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
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
				let value = Self {};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub threepids: Vec<crate::thirdparty::ThirdPartyIdentifier>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"threepids",
					crate::endpoint::enc(&self.threepids),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					threepids: input.body("threepids")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/account/3pid/add");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("client_secret", crate::endpoint::enc(&self.client_secret)),
						("sid", crate::endpoint::enc(&self.sid)),
						("id_server", crate::endpoint::enc(&self.id_server)),
						(
							"id_server_access_token",
							crate::endpoint::enc(&self.id_server_access_token),
						),
						("auth", crate::endpoint::enc(&self.auth)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					client_secret: input.body("client_secret")?,
					sid: input.body("sid")?,
					id_server: input.body("id_server")?,
					id_server_access_token: input.body("id_server_access_token")?,
					auth: input.body("auth")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [])
			}
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/account/3pid/delete");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("medium", crate::endpoint::enc(&self.medium)),
						("address", crate::endpoint::enc(&self.address)),
						("id_server", crate::endpoint::enc(&self.id_server)),
						(
							"id_server_access_token",
							crate::endpoint::enc(&self.id_server_access_token),
						),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					medium: input.body("medium")?,
					address: input.body("address")?,
					id_server: input.body("id_server")?,
					id_server_access_token: input.body("id_server_access_token")?,
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"id_server_unbind_result",
					crate::endpoint::enc(&self.id_server_unbind_result),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					id_server_unbind_result: input.body("id_server_unbind_result")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v1/register/m.login.registration_token/validity",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"token",
					crate::endpoint::enc(&self.token),
				)])
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
					token: input.query("token")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub valid: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("valid", crate::endpoint::enc(&self.valid))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					valid: input.body("valid")?,
				})
			}
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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"POST",
				"/_matrix/client/v3/register/email/requestToken",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [
						("client_secret", crate::endpoint::enc(&self.client_secret)),
						("email", crate::endpoint::enc(&self.email)),
						("send_attempt", crate::endpoint::enc(&self.send_attempt)),
					],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					client_secret: input.body("client_secret")?,
					email: input.body("email")?,
					send_attempt: input.body("send_attempt")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub sid: String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("sid", crate::endpoint::enc(&self.sid))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					sid: input.body("sid")?,
				})
			}
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

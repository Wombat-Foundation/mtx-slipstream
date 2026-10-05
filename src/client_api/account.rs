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
			request { path {} query {} body { new_password: String, logout_devices: bool, auth: crate::uiaa::AuthData } }
			response {}
		}
	}
}
pub mod deactivate {
	pub mod v3 {
		use crate::endpoint;
		endpoint! {
			method: "POST", path: "/_matrix/client/v3/account/deactivate",
			request { path {} query {} body { auth: Option<crate::uiaa::AuthData>, id_server: Option<String>, id_server_access_token: Option<String>, erase: Option<bool> } }
			response { id_server_unbind_result: Option<super::super::ThirdPartyIdRemovalStatus> }
		}
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
		endpoint! { method: "GET", path: "/_matrix/client/v3/account/whoami", request { path {} query {} body {} } response { user_id: crate::OwnedUserId, device_id: Option<crate::OwnedDeviceId>, is_guest: Option<bool> } }
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
		endpoint! { method: "POST", path: "/_matrix/client/v3/account/3pid/add", request { path {} query {} body { client_secret: String, sid: String, id_server: Option<String>, id_server_access_token: Option<String> } } response {} }
	}
}
pub mod delete_3pid {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/account/3pid/delete", request { path {} query {} body { medium: crate::thirdparty::Medium, address: String, id_server: Option<String>, id_server_access_token: Option<String> } } response { id_server_unbind_result: Option<super::super::ThirdPartyIdRemovalStatus> } }
	}
}
pub mod check_registration_token_validity {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "GET", path: "/_matrix/client/v3/register/m.login.registration_token/validity", request { path {} query { token: String } body {} } response { valid: bool } }
	}
}
pub mod request_registration_token_via_email {
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/register/email/requestToken", request { path {} query {} body { client_secret: String, email: String, send_attempt: crate::UInt } } response { sid: String } }
	}
}
pub mod register {
	pub use super::RegistrationKind as LoginType;
	pub mod v3 {
		use crate::endpoint;
		endpoint! { method: "POST", path: "/_matrix/client/v3/register", request { path {} query {} body { username: Option<String>, password: Option<String>, device_id: Option<crate::OwnedDeviceId>, initial_device_display_name: Option<String>, inhibit_login: bool, kind: super::super::RegistrationKind, auth: Option<crate::uiaa::AuthData> } } response { user_id: crate::OwnedUserId, access_token: Option<String>, device_id: Option<crate::OwnedDeviceId>, refresh_token: Option<String>, expires_in_ms: Option<crate::UInt> } }
	}
}

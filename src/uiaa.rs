//! User-interactive authentication types.

use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use core::fmt;

use crate::{
	api::client::error::ErrorKind,
	codec::{DeError, Deserialize, Serialize},
	endpoint::{Input, object_from},
	impl_codec_struct,
	json::Value,
};

/// An `errcode` and human-readable message, as found in error bodies.
#[derive(Clone, Debug)]
pub struct StandardErrorBody {
	pub kind: ErrorKind,
	pub message: String,
}

impl Serialize for StandardErrorBody {
	fn to_json(&self) -> Value {
		Value::Object(object_from(alloc::vec![
			("errcode", Value::String(self.kind.errcode().into())),
			("error", Value::String(self.message.clone())),
		]))
	}
}

impl Deserialize for StandardErrorBody {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let errcode: Option<String> = input.body("errcode")?;
		Ok(Self {
			kind: ErrorKind::from_errcode(errcode.as_deref().unwrap_or("M_UNKNOWN")),
			message: input.body_or_default("error")?,
		})
	}
}

/// The type of one authentication stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthType {
	Password,
	ReCaptcha,
	EmailIdentity,
	Msisdn,
	Sso,
	Dummy,
	RegistrationToken,
	Terms,
	_Custom(String),
}

impl AuthType {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Password => "m.login.password",
			Self::ReCaptcha => "m.login.recaptcha",
			Self::EmailIdentity => "m.login.email.identity",
			Self::Msisdn => "m.login.msisdn",
			Self::Sso => "m.login.sso",
			Self::Dummy => "m.login.dummy",
			Self::RegistrationToken => "m.login.registration_token",
			Self::Terms => "m.login.terms",
			Self::_Custom(other) => other,
		}
	}
}

impl From<&str> for AuthType {
	fn from(value: &str) -> Self {
		match value {
			"m.login.password" => Self::Password,
			"m.login.recaptcha" => Self::ReCaptcha,
			"m.login.email.identity" => Self::EmailIdentity,
			"m.login.msisdn" => Self::Msisdn,
			"m.login.sso" => Self::Sso,
			"m.login.dummy" => Self::Dummy,
			"m.login.registration_token" => Self::RegistrationToken,
			"m.login.terms" => Self::Terms,
			other => Self::_Custom(other.into()),
		}
	}
}

impl fmt::Display for AuthType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

crate::impl_codec_string!(AuthType, |s| Ok(Self::from(s)), |v| v.as_str());

/// A sequence of stages that together complete authentication.
#[derive(Clone, Debug, Default)]
pub struct AuthFlow {
	pub stages: Vec<AuthType>,
}

impl AuthFlow {
	#[must_use]
	pub fn new(stages: Vec<AuthType>) -> Self {
		Self {
			stages,
		}
	}
}

impl_codec_struct!(AuthFlow { stages: Vec<AuthType> });

/// The state of a UIAA session, sent to the client with a 401.
#[derive(Clone, Debug, Default)]
pub struct UiaaInfo {
	pub flows: Vec<AuthFlow>,
	pub completed: Vec<AuthType>,
	/// Per-stage parameters; `null` is sent as an empty object.
	pub params: Value,
	pub session: Option<String>,
	pub auth_error: Option<StandardErrorBody>,
}

impl UiaaInfo {
	#[must_use]
	pub fn new(flows: Vec<AuthFlow>, params: Value) -> Self {
		Self {
			flows,
			params,
			..Self::default()
		}
	}
}

impl Serialize for UiaaInfo {
	fn to_json(&self) -> Value {
		let params = if self.params.is_null() {
			Value::Object(crate::json::Object::new())
		} else {
			self.params.clone()
		};
		let mut object = object_from(alloc::vec![
			("flows", self.flows.to_json()),
			("completed", self.completed.to_json()),
			("params", params),
			("session", self.session.to_json()),
		]);
		if let Some(Value::Object(fields)) = self.auth_error.as_ref().map(Serialize::to_json) {
			object.extend(fields);
		}
		Value::Object(object)
	}
}

impl Deserialize for UiaaInfo {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let auth_error = if value.get("errcode").is_some() {
			Some(StandardErrorBody::from_json(value)?)
		} else {
			None
		};
		Ok(Self {
			flows: input.body("flows")?,
			completed: input.body_or_default("completed")?,
			params: input.body_or_default("params")?,
			session: input.body("session")?,
			auth_error,
		})
	}
}

/// How the user identifies themselves in an authentication stage.
#[derive(Debug)]
pub enum UserIdentifier {
	UserIdOrLocalpart(String),
	Email {
		address: String,
	},
	Msisdn {
		number: String,
	},
	/// Any other identifier type, kept as received.
	_Custom(Value),
}

impl Serialize for UserIdentifier {
	fn to_json(&self) -> Value {
		let pairs = match self {
			Self::UserIdOrLocalpart(user) => {
				alloc::vec![("type", "m.id.user".to_json()), ("user", user.to_json())]
			}
			Self::Email {
				address,
			} => alloc::vec![
				("type", "m.id.thirdparty".to_json()),
				("medium", "email".to_json()),
				("address", address.to_json()),
			],
			Self::Msisdn {
				number,
			} => alloc::vec![
				("type", "m.id.thirdparty".to_json()),
				("medium", "msisdn".to_json()),
				("address", number.to_json()),
			],
			Self::_Custom(value) => return value.clone(),
		};
		Value::Object(object_from(pairs))
	}
}

impl Deserialize for UserIdentifier {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let field = |name: &str| value.get(name).and_then(Value::as_str).map(ToString::to_string);
		Ok(match (field("type").as_deref(), field("medium").as_deref()) {
			(Some("m.id.user"), _) => field("user")
				.map(Self::UserIdOrLocalpart)
				.ok_or_else(|| DeError::expected("user"))?,
			(Some("m.id.thirdparty"), Some("email")) => Self::Email {
				address: field("address").ok_or_else(|| DeError::expected("address"))?,
			},
			(Some("m.id.thirdparty"), Some("msisdn")) => Self::Msisdn {
				number: field("address").ok_or_else(|| DeError::expected("address"))?,
			},
			_ => Self::_Custom(value.clone()),
		})
	}
}

#[derive(Debug)]
pub struct ThirdpartyIdCredentials {
	pub sid: String,
	pub client_secret: String,
	pub id_server: Option<String>,
	pub id_access_token: Option<String>,
}

impl_codec_struct!(ThirdpartyIdCredentials {
	sid: String,
	client_secret: String,
} default {
	id_server: Option<String>,
	id_access_token: Option<String>,
});

macro_rules! auth_stage {
	($(#[$meta:meta])* $name:ident { $($field:ident : $ty:ty),* }) => {
		$(#[$meta])*
		#[derive(Debug)]
		pub struct $name {
			$(pub $field: $ty,)*
			pub session: Option<String>,
		}
	};
}

auth_stage!(Password { identifier: Option<UserIdentifier>, password: String });
auth_stage!(ReCaptcha {
	response: String
});
auth_stage!(RegistrationToken {
	token: String
});
auth_stage!(EmailIdentity {
	thirdparty_id_creds: ThirdpartyIdCredentials
});
auth_stage!(Dummy {});
auth_stage!(Terms {});
auth_stage!(
	/// A client checking whether fallback authentication has completed.
	FallbackAcknowledgement {}
);

/// Authentication data a client sends to continue a UIAA session.
#[derive(Debug)]
pub enum AuthData {
	Password(Password),
	ReCaptcha(ReCaptcha),
	EmailIdentity(EmailIdentity),
	RegistrationToken(RegistrationToken),
	Dummy(Dummy),
	Terms(Terms),
	FallbackAcknowledgement(FallbackAcknowledgement),
	/// Any other stage, kept as received.
	_Custom(Value),
}

impl AuthData {
	/// The stage type, or `None` for a fallback acknowledgement.
	#[must_use]
	pub fn auth_type(&self) -> Option<AuthType> {
		Some(match self {
			Self::Password(_) => AuthType::Password,
			Self::ReCaptcha(_) => AuthType::ReCaptcha,
			Self::EmailIdentity(_) => AuthType::EmailIdentity,
			Self::RegistrationToken(_) => AuthType::RegistrationToken,
			Self::Dummy(_) => AuthType::Dummy,
			Self::Terms(_) => AuthType::Terms,
			Self::FallbackAcknowledgement(_) => return None,
			Self::_Custom(value) => {
				value.get("type").and_then(Value::as_str).map(AuthType::from)?
			}
		})
	}

	#[must_use]
	pub fn session(&self) -> Option<&str> {
		match self {
			Self::Password(Password {
				session,
				..
			})
			| Self::ReCaptcha(ReCaptcha {
				session,
				..
			})
			| Self::EmailIdentity(EmailIdentity {
				session,
				..
			})
			| Self::RegistrationToken(RegistrationToken {
				session,
				..
			})
			| Self::Dummy(Dummy {
				session,
			})
			| Self::Terms(Terms {
				session,
			})
			| Self::FallbackAcknowledgement(FallbackAcknowledgement {
				session,
			}) => session.as_deref(),
			Self::_Custom(value) => value.get("session").and_then(Value::as_str),
		}
	}
}

impl Serialize for AuthData {
	fn to_json(&self) -> Value {
		let typed =
			|kind: &AuthType, session: &Option<String>, mut rest: Vec<(&'static str, Value)>| {
				rest.insert(0, ("type", kind.to_json()));
				rest.push(("session", session.to_json()));
				Value::Object(object_from(rest))
			};
		match self {
			Self::Password(p) => typed(
				&AuthType::Password,
				&p.session,
				alloc::vec![
					("identifier", p.identifier.to_json()),
					("password", p.password.to_json()),
				],
			),
			Self::ReCaptcha(r) => typed(
				&AuthType::ReCaptcha,
				&r.session,
				alloc::vec![("response", r.response.to_json())],
			),
			Self::EmailIdentity(e) => typed(
				&AuthType::EmailIdentity,
				&e.session,
				alloc::vec![("threepid_creds", e.thirdparty_id_creds.to_json())],
			),
			Self::RegistrationToken(t) => typed(
				&AuthType::RegistrationToken,
				&t.session,
				alloc::vec![("token", t.token.to_json())],
			),
			Self::Dummy(d) => typed(&AuthType::Dummy, &d.session, Vec::new()),
			Self::Terms(t) => typed(&AuthType::Terms, &t.session, Vec::new()),
			Self::FallbackAcknowledgement(f) => {
				Value::Object(object_from(alloc::vec![("session", f.session.to_json())]))
			}
			Self::_Custom(value) => value.clone(),
		}
	}
}

impl Deserialize for AuthData {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let session: Option<String> = input.body("session")?;
		let kind: Option<String> = input.body("type")?;
		Ok(match kind.as_deref().map(AuthType::from) {
			None => Self::FallbackAcknowledgement(FallbackAcknowledgement {
				session,
			}),
			Some(AuthType::Password) => Self::Password(Password {
				identifier: input.body("identifier")?,
				password: input.body("password")?,
				session,
			}),
			Some(AuthType::ReCaptcha) => Self::ReCaptcha(ReCaptcha {
				response: input.body("response")?,
				session,
			}),
			Some(AuthType::EmailIdentity) => Self::EmailIdentity(EmailIdentity {
				thirdparty_id_creds: input.body("threepid_creds")?,
				session,
			}),
			Some(AuthType::RegistrationToken) => Self::RegistrationToken(RegistrationToken {
				token: input.body("token")?,
				session,
			}),
			Some(AuthType::Dummy) => Self::Dummy(Dummy {
				session,
			}),
			Some(AuthType::Terms) => Self::Terms(Terms {
				session,
			}),
			Some(_) => Self::_Custom(value.clone()),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::from_str;

	#[test]
	fn password_stage_round_trips() {
		let auth: AuthData = from_str(
			r#"{"type":"m.login.password","session":"s",
			"identifier":{"type":"m.id.user","user":"bob"},"password":"pw"}"#,
		)
		.unwrap();
		assert_eq!(auth.session(), Some("s"));
		assert_eq!(auth.auth_type(), Some(AuthType::Password));
		let again = AuthData::from_json(&auth.to_json()).unwrap();
		assert!(matches!(
			again,
			AuthData::Password(Password {
				identifier: Some(UserIdentifier::UserIdOrLocalpart(ref u)), ..
			}) if u == "bob"
		));
	}

	#[test]
	fn missing_type_is_fallback_acknowledgement() {
		let auth: AuthData = from_str(r#"{"session":"s"}"#).unwrap();
		assert!(matches!(auth, AuthData::FallbackAcknowledgement(_)));
		assert_eq!(auth.auth_type(), None);
	}

	#[test]
	fn info_flattens_auth_error() {
		let mut info =
			UiaaInfo::new(alloc::vec![AuthFlow::new(alloc::vec![AuthType::Dummy])], Value::Null);
		info.auth_error = Some(StandardErrorBody {
			kind: ErrorKind::forbidden(),
			message: "nope".into(),
		});
		let json = info.to_json();
		assert_eq!(json.get("errcode").and_then(Value::as_str), Some("M_FORBIDDEN"));
		assert!(json.get("params").is_some_and(|p| p.as_object().is_some()));
		assert!(UiaaInfo::from_json(&json).unwrap().auth_error.is_some());
	}
}

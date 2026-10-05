//! Application-service registration and the homeserver-to-appservice API.

use alloc::{string::String, vec::Vec};

use crate::{
	codec::{DeError, Deserialize, Serialize},
	impl_codec_struct,
	json::Value,
};

/// A namespace of user IDs, room aliases or room IDs an appservice claims.
#[derive(Debug, Default)]
pub struct Namespace {
	pub exclusive: bool,
	pub regex: String,
}

impl_codec_struct!(Namespace { regex: String } default { exclusive: bool });

#[derive(Debug, Default)]
pub struct Namespaces {
	pub users: Vec<Namespace>,
	pub aliases: Vec<Namespace>,
	pub rooms: Vec<Namespace>,
}

impl_codec_struct!(Namespaces {} default {
	users: Vec<Namespace>,
	aliases: Vec<Namespace>,
	rooms: Vec<Namespace>,
});

/// An appservice registration file.
#[derive(Debug, Default)]
pub struct Registration {
	pub id: String,
	/// Where the appservice listens; `None` (or the string `null`) for an
	/// appservice that only uses the client-server API.
	pub url: Option<String>,
	pub as_token: String,
	pub hs_token: String,
	pub sender_localpart: String,
	pub namespaces: Namespaces,
	pub rate_limited: Option<bool>,
	pub protocols: Option<Vec<String>>,
	/// MSC2409: receive ephemeral events.
	pub receive_ephemeral: bool,
	/// MSC4190: let the appservice manage its users' devices.
	pub device_management: bool,
}

impl_codec_struct!(Registration {
	id: String,
	as_token: String,
	hs_token: String,
	sender_localpart: String,
} default {
	url: Option<String>,
	namespaces: Namespaces,
	rate_limited: Option<bool>,
	protocols: Option<Vec<String>>,
	receive_ephemeral: bool,
	device_management: bool,
});

/// One ephemeral event (typing, receipt or presence) for an appservice.
#[derive(Debug)]
pub struct EphemeralData(pub Value);

impl From<Value> for EphemeralData {
	fn from(value: Value) -> Self {
		Self(value)
	}
}

impl Serialize for EphemeralData {
	fn to_json(&self) -> Value {
		self.0.clone()
	}
}

impl Deserialize for EphemeralData {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self(value.clone()))
	}
}

pub mod event {
	pub mod push_events {
		pub mod v1 {
			use alloc::vec::Vec;

			use crate::{OwnedTransactionId, federation_api::RawPdu};

			pub use crate::appservice::EphemeralData;

			crate::endpoint! {
				method: "PUT", path: "/_matrix/app/v1/transactions/{txn_id}",
				request {
					path { txn_id: OwnedTransactionId }
					query {}
					body {
						events: Vec<RawPdu>,
						ephemeral: Vec<EphemeralData>,
						to_device: Vec<crate::json::Value>
					}
				}
				response {}
			}
		}
	}
}

pub mod query {
	pub mod query_room_alias {
		pub mod v1 {
			crate::endpoint! {
				method: "GET", path: "/_matrix/app/v1/rooms/{room_alias}",
				request {
					path { room_alias: crate::OwnedRoomAliasId }
					query {}
					body {}
				}
				response {}
			}
		}
	}

	pub mod query_user_id {
		pub mod v1 {
			crate::endpoint! {
				method: "GET", path: "/_matrix/app/v1/users/{user_id}",
				request {
					path { user_id: crate::OwnedUserId }
					query {}
					body {}
				}
				response {}
			}
		}
	}
}

pub mod ping {
	pub mod send_ping {
		pub mod v1 {
			crate::endpoint! {
				method: "POST", path: "/_matrix/app/v1/ping",
				request {
					path {}
					query {}
					body { transaction_id: Option<alloc::string::String> }
				}
				response {}
			}
		}
	}
}

pub mod request_ping {
	pub mod v1 {
		use core::time::Duration;

		use crate::{
			codec::{DeError, Serialize},
			endpoint::{EndpointResponse, Input},
			json::Value,
		};

		crate::endpoint_request! {
			method: "POST", path: "/_matrix/client/v1/appservice/{appservice_id}/ping",
			request {
				path { appservice_id: alloc::string::String }
				query {}
				body { transaction_id: Option<alloc::string::String> }
			}
		}

		/// How long the appservice took to answer, in the `duration_ms` field.
		#[derive(Debug)]
		pub struct Response {
			pub duration: Duration,
		}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				let millis = u64::try_from(self.duration.as_millis()).unwrap_or(u64::MAX);
				Value::Object(crate::endpoint::object_from(alloc::vec![(
					"duration_ms",
					millis.to_json()
				)]))
			}

			fn from_body(body: &Value) -> Result<Self, DeError> {
				let input = Input::new(&[], &[], Some(body));
				let millis: u64 = input.body("duration_ms")?;
				Ok(Self {
					duration: Duration::from_millis(millis),
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::from_str;

	#[test]
	fn registration_parses_with_defaults() {
		let registration: Registration = from_str(
			r#"{"id":"x","as_token":"a","hs_token":"h","sender_localpart":"bot",
			"namespaces":{"users":[{"exclusive":true,"regex":"@x_.*"}]},
			"de.sorunome.msc2409.push_ephemeral":true}"#,
		)
		.unwrap();
		assert!(registration.namespaces.users[0].exclusive);
		assert!(registration.namespaces.aliases.is_empty());
		assert_eq!(registration.url, None);
		assert!(!registration.receive_ephemeral);
	}
}

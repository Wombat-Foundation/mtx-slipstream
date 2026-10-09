//! Application-service registration and the homeserver-to-appservice API.

use alloc::{string::String, vec::Vec};

use crate::{
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

/// A namespace of user IDs, room aliases or room IDs an appservice claims.
#[derive(Clone, Debug, Default)]
pub struct Namespace {
	pub exclusive: bool,
	pub regex: String,
}

impl crate::codec::Serialize for Namespace {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(regex), crate::endpoint::enc(&self.regex)),
			(stringify!(exclusive), crate::endpoint::enc(&self.exclusive)),
		])
	}
}
impl crate::codec::Deserialize for Namespace {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Namespace)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			regex: input.body(stringify!(regex))?,
			exclusive: input.body_or_default(stringify!(exclusive))?,
		})
	}
}

#[derive(Clone, Debug, Default)]
pub struct Namespaces {
	pub users: Vec<Namespace>,
	pub aliases: Vec<Namespace>,
	pub rooms: Vec<Namespace>,
}

impl crate::codec::Serialize for Namespaces {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(users), crate::endpoint::enc(&self.users)),
			(stringify!(aliases), crate::endpoint::enc(&self.aliases)),
			(stringify!(rooms), crate::endpoint::enc(&self.rooms)),
		])
	}
}
impl crate::codec::Deserialize for Namespaces {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Namespaces)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			users: input.body_or_default(stringify!(users))?,
			aliases: input.body_or_default(stringify!(aliases))?,
			rooms: input.body_or_default(stringify!(rooms))?,
		})
	}
}

/// An appservice registration file.
#[derive(Clone, Debug, Default)]
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

impl crate::codec::Serialize for Registration {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(id), crate::endpoint::enc(&self.id)),
			(stringify!(as_token), crate::endpoint::enc(&self.as_token)),
			(stringify!(hs_token), crate::endpoint::enc(&self.hs_token)),
			(stringify!(sender_localpart), crate::endpoint::enc(&self.sender_localpart)),
			(stringify!(url), crate::endpoint::enc(&self.url)),
			(stringify!(namespaces), crate::endpoint::enc(&self.namespaces)),
			(stringify!(rate_limited), crate::endpoint::enc(&self.rate_limited)),
			(stringify!(protocols), crate::endpoint::enc(&self.protocols)),
			(stringify!(receive_ephemeral), crate::endpoint::enc(&self.receive_ephemeral)),
			(stringify!(device_management), crate::endpoint::enc(&self.device_management)),
		])
	}
}
impl crate::codec::Deserialize for Registration {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Registration)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			id: input.body(stringify!(id))?,
			as_token: input.body(stringify!(as_token))?,
			hs_token: input.body(stringify!(hs_token))?,
			sender_localpart: input.body(stringify!(sender_localpart))?,
			url: input.body_or_default(stringify!(url))?,
			namespaces: input.body_or_default(stringify!(namespaces))?,
			rate_limited: input.body_or_default(stringify!(rate_limited))?,
			protocols: input.body_or_default(stringify!(protocols))?,
			receive_ephemeral: input.body_or_default(stringify!(receive_ephemeral))?,
			device_management: input.body_or_default(stringify!(device_management))?,
		})
	}
}

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

			pub struct Request {
				pub txn_id: OwnedTransactionId,
				pub events: Vec<RawPdu>,
				pub ephemeral: Vec<EphemeralData>,
				pub to_device: Vec<crate::json::Value>,
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
					"PUT",
					"/_matrix/app/v1/transactions/{txn_id}",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.txn_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("events", crate::endpoint::enc(&self.events)),
							("ephemeral", crate::endpoint::enc(&self.ephemeral)),
							("to_device", crate::endpoint::enc(&self.to_device)),
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
						txn_id: input.path()?,
						events: input.body("events")?,
						ephemeral: input.body("ephemeral")?,
						to_device: input.body("to_device")?,
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
}

pub mod query {
	pub mod query_room_alias {
		pub mod v1 {
			pub struct Request {
				pub room_alias: crate::OwnedRoomAliasId,
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
					crate::endpoint::Metadata::new("GET", "/_matrix/app/v1/rooms/{room_alias}");
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.room_alias,
					)])
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
						room_alias: input.path()?,
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

	pub mod query_user_id {
		pub mod v1 {
			pub struct Request {
				pub user_id: crate::OwnedUserId,
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
					crate::endpoint::Metadata::new("GET", "/_matrix/app/v1/users/{user_id}");
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.user_id,
					)])
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
}

pub mod ping {
	pub mod send_ping {
		pub mod v1 {
			pub struct Request {
				pub transaction_id: Option<alloc::string::String>,
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
					crate::endpoint::Metadata::new("POST", "/_matrix/app/v1/ping");
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("transaction_id", crate::endpoint::enc(&self.transaction_id))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						transaction_id: input.body("transaction_id")?,
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
}

pub mod request_ping {
	pub mod v1 {
		use core::time::Duration;

		use crate::{
			codec::{DeError, Serialize},
			endpoint::{EndpointResponse, Input},
			json::Value,
		};

		pub struct Request {
			pub appservice_id: alloc::string::String,
			pub transaction_id: Option<alloc::string::String>,
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
				"/_matrix/client/v1/appservice/{appservice_id}/ping",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.appservice_id,
				)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("transaction_id", crate::endpoint::enc(&self.transaction_id))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					appservice_id: input.path()?,
					transaction_id: input.body("transaction_id")?,
				};
				input.finish()?;
				Ok(value)
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

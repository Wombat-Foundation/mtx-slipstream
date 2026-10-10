//! Device management and dehydrated devices.

use alloc::string::String;

use crate::{
	MilliSecondsSinceUnixEpoch, OwnedDeviceId,
	codec::{DeError, Deserialize, Serialize},
	json::Value,
};

/// Metadata about one of a user's devices.
#[derive(Debug)]
pub struct Device {
	pub device_id: OwnedDeviceId,
	pub display_name: Option<String>,
	pub last_seen_ip: Option<String>,
	pub last_seen_ts: Option<MilliSecondsSinceUnixEpoch>,
}

impl Device {
	#[must_use]
	pub fn new(device_id: OwnedDeviceId) -> Self {
		Self {
			device_id,
			display_name: None,
			last_seen_ip: None,
			last_seen_ts: None,
		}
	}
}

impl crate::codec::Serialize for Device {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(device_id), crate::endpoint::enc(&self.device_id)),
			(stringify!(display_name), crate::endpoint::enc(&self.display_name)),
			(stringify!(last_seen_ip), crate::endpoint::enc(&self.last_seen_ip)),
			(stringify!(last_seen_ts), crate::endpoint::enc(&self.last_seen_ts)),
		])
	}
}
impl crate::codec::Deserialize for Device {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Device)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			device_id: input.body(stringify!(device_id))?,
			display_name: input.body_or_default(stringify!(display_name))?,
			last_seen_ip: input.body_or_default(stringify!(last_seen_ip))?,
			last_seen_ts: input.body_or_default(stringify!(last_seen_ts))?,
		})
	}
}

pub mod get_devices {
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
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/devices");
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
			pub devices: alloc::vec::Vec<crate::device::Device>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"devices",
					crate::endpoint::enc(&self.devices),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					devices: input.body("devices")?,
				})
			}
		}
	}
}

pub mod get_device {
	pub mod v3 {
		pub struct Request {
			pub device_id: crate::OwnedDeviceId,
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
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/devices/{device_id}");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.device_id,
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
					device_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub device: crate::device::Device,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.device)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					device: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	}
}

pub mod update_device {
	pub mod v3 {
		pub struct Request {
			pub device_id: crate::OwnedDeviceId,
			pub display_name: Option<alloc::string::String>,
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
				crate::endpoint::Metadata::new("PUT", "/_matrix/client/v3/devices/{device_id}");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.device_id,
				)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("display_name", crate::endpoint::enc(&self.display_name))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					device_id: input.path()?,
					display_name: input.body("display_name")?,
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

pub mod delete_device {
	pub mod v3 {
		pub struct Request {
			pub device_id: crate::OwnedDeviceId,
			pub auth: Option<crate::api::client::uiaa::AuthData>,
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
				"DELETE",
				"/_matrix/client/v3/devices/{device_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
					&self.device_id,
				)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("auth", crate::endpoint::enc(&self.auth))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					device_id: input.path()?,
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

pub mod delete_devices {
	pub mod v3 {
		pub struct Request {
			pub devices: alloc::vec::Vec<crate::OwnedDeviceId>,
			pub auth: Option<crate::api::client::uiaa::AuthData>,
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
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/delete_devices");
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
						("devices", crate::endpoint::enc(&self.devices)),
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
					devices: input.body("devices")?,
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

/// The encrypted private data of a dehydrated device (MSC3814).
#[derive(Debug)]
pub struct DehydratedDeviceData(pub Value);

impl Serialize for DehydratedDeviceData {
	fn to_json(&self) -> Value {
		self.0.clone()
	}
}

impl Deserialize for DehydratedDeviceData {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		Ok(Self(value.clone()))
	}
}

pub mod dehydrated_device {
	pub use crate::device::DehydratedDeviceData;

	pub mod put_dehydrated_device {
		pub mod unstable {
			use alloc::{collections::BTreeMap, string::String};

			use crate::{
				OwnedDeviceId, OwnedOneTimeKeyId,
				device::DehydratedDeviceData,
				encryption::{DeviceKeys, OneTimeKey},
				sswire::Raw,
			};

			pub struct Request {
				pub device_id: OwnedDeviceId,
				pub device_data: Raw<DehydratedDeviceData>,
				pub device_keys: Raw<DeviceKeys>,
				pub one_time_keys: BTreeMap<OwnedOneTimeKeyId, Raw<OneTimeKey>>,
				pub fallback_keys: BTreeMap<OwnedOneTimeKeyId, Raw<OneTimeKey>>,
				pub initial_device_display_name: Option<String>,
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
					"/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
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
							("device_id", crate::endpoint::enc(&self.device_id)),
							("device_data", crate::endpoint::enc(&self.device_data)),
							("device_keys", crate::endpoint::enc(&self.device_keys)),
							("one_time_keys", crate::endpoint::enc(&self.one_time_keys)),
							("fallback_keys", crate::endpoint::enc(&self.fallback_keys)),
							(
								"initial_device_display_name",
								crate::endpoint::enc(&self.initial_device_display_name),
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
						device_id: input.body("device_id")?,
						device_data: input.body("device_data")?,
						device_keys: input.body("device_keys")?,
						one_time_keys: input.body("one_time_keys")?,
						fallback_keys: input.body("fallback_keys")?,
						initial_device_display_name: input.body("initial_device_display_name")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub device_id: OwnedDeviceId,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [(
						"device_id",
						crate::endpoint::enc(&self.device_id),
					)])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						device_id: input.body("device_id")?,
					})
				}
			}
		}
	}

	pub mod get_dehydrated_device {
		pub mod unstable {
			use crate::{OwnedDeviceId, device::DehydratedDeviceData, sswire::Raw};

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
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"GET",
					"/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
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
				pub device_id: OwnedDeviceId,
				pub device_data: Raw<DehydratedDeviceData>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("device_id", crate::endpoint::enc(&self.device_id)),
						("device_data", crate::endpoint::enc(&self.device_data)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						device_id: input.body("device_id")?,
						device_data: input.body("device_data")?,
					})
				}
			}
		}
	}

	pub mod delete_dehydrated_device {
		pub mod unstable {
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
				const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
					"DELETE",
					"/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
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
				pub device_id: crate::OwnedDeviceId,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [(
						"device_id",
						crate::endpoint::enc(&self.device_id),
					)])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						device_id: input.body("device_id")?,
					})
				}
			}
		}
	}

	pub mod get_events {
		pub mod unstable {
			use alloc::{string::String, vec::Vec};

			use crate::{OwnedDeviceId, events::AnyToDeviceEvent, sswire::Raw};

			pub struct Request {
				pub device_id: OwnedDeviceId,
				pub next_batch: Option<String>,
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
					"/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device/{device_id}/events",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.device_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("next_batch", crate::endpoint::enc(&self.next_batch))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						device_id: input.path()?,
						next_batch: input.body("next_batch")?,
					};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub events: Vec<Raw<AnyToDeviceEvent>>,
				pub next_batch: Option<String>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [
						("events", crate::endpoint::enc(&self.events)),
						("next_batch", crate::endpoint::enc(&self.next_batch)),
					])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						events: input.body("events")?,
						next_batch: input.body("next_batch")?,
					})
				}
			}
		}
	}
}

/// A user's stored dehydrated device.
#[derive(Debug)]
pub struct DehydratedDevice {
	/// Unique ID of the device.
	pub device_id: OwnedDeviceId,
	/// Serialized and encrypted private data.
	pub device_data: crate::sswire::Raw<DehydratedDeviceData>,
}

impl crate::codec::Serialize for DehydratedDevice {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(device_id), crate::endpoint::enc(&self.device_id)),
			(stringify!(device_data), crate::endpoint::enc(&self.device_data)),
		])
	}
}
impl crate::codec::Deserialize for DehydratedDevice {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(DehydratedDevice)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			device_id: input.body(stringify!(device_id))?,
			device_data: input.body(stringify!(device_data))?,
		})
	}
}

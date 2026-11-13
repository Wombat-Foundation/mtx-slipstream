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
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/devices",
			path {}
			query {}
			body {}
		}
		pub struct Response {
			pub devices: alloc::vec::Vec<crate::device::Device>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { devices => "devices" }
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
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v3/devices/{device_id}",
			path { device_id }
			query {}
			body {}
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
		crate::endpoint_request_impl! {
			method: "PUT",
			path: "/_matrix/client/v3/devices/{device_id}",
			path { device_id }
			query {}
			body { display_name => "display_name" }
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
		crate::endpoint_request_impl! {
			method: "DELETE",
			path: "/_matrix/client/v3/devices/{device_id}",
			path { device_id }
			query {}
			body { auth => "auth" }
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
		crate::endpoint_request_impl! {
			method: "POST",
			path: "/_matrix/client/v3/delete_devices",
			path {}
			query {}
			body { devices => "devices", auth => "auth" }
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
			crate::endpoint_request_impl! {
				method: "PUT",
				path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
				path {}
				query {}
				body { device_id => "device_id", device_data => "device_data", device_keys => "device_keys", one_time_keys => "one_time_keys", fallback_keys => "fallback_keys", initial_device_display_name => "initial_device_display_name" }
			}
			pub struct Response {
				pub device_id: OwnedDeviceId,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { device_id => "device_id" }
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
			crate::endpoint_request_impl! {
				method: "GET",
				path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
				path {}
				query {}
				body {}
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
			crate::endpoint_response_impl! {
				body { device_id => "device_id", device_data => "device_data" }
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
			crate::endpoint_request_impl! {
				method: "DELETE",
				path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
				path {}
				query {}
				body {}
			}
			pub struct Response {
				pub device_id: crate::OwnedDeviceId,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			crate::endpoint_response_impl! {
				body { device_id => "device_id" }
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
			crate::endpoint_request_impl! {
				method: "POST",
				path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device/{device_id}/events",
				path { device_id }
				query {}
				body { next_batch => "next_batch" }
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
			crate::endpoint_response_impl! {
				body { events => "events", next_batch => "next_batch" }
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

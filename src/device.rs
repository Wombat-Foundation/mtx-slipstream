//! Device management and dehydrated devices.

use alloc::string::String;

use crate::{
	MilliSecondsSinceUnixEpoch, OwnedDeviceId,
	codec::{DeError, Deserialize, Serialize},
	impl_codec_struct,
	json::Value,
};

/// Metadata about one of a user's devices.
#[derive(Clone, Debug)]
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

impl_codec_struct!(Device { device_id: OwnedDeviceId } default {
	display_name: Option<String>,
	last_seen_ip: Option<String>,
	last_seen_ts: Option<MilliSecondsSinceUnixEpoch>,
});

pub mod get_devices {
	pub mod v3 {
		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/devices",
			request { path {} query {} body {} }
			response { devices: alloc::vec::Vec<crate::device::Device> }
		}
	}
}

pub mod get_device {
	pub mod v3 {
		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/devices/{device_id}",
			request { path { device_id: crate::OwnedDeviceId } query {} body {} }
		}
		crate::endpoint_response_flat!(device: crate::device::Device);
	}
}

pub mod update_device {
	pub mod v3 {
		crate::endpoint! {
			method: "PUT", path: "/_matrix/client/v3/devices/{device_id}",
			request {
				path { device_id: crate::OwnedDeviceId }
				query {}
				body { display_name: Option<alloc::string::String> }
			}
			response {}
		}
	}
}

pub mod delete_device {
	pub mod v3 {
		crate::endpoint! {
			method: "DELETE", path: "/_matrix/client/v3/devices/{device_id}",
			request {
				path { device_id: crate::OwnedDeviceId }
				query {}
				body { auth: Option<crate::api::client::uiaa::AuthData> }
			}
			response {}
		}
	}
}

pub mod delete_devices {
	pub mod v3 {
		crate::endpoint! {
			method: "POST", path: "/_matrix/client/v3/delete_devices",
			request {
				path {}
				query {}
				body {
					devices: alloc::vec::Vec<crate::OwnedDeviceId>,
					auth: Option<crate::api::client::uiaa::AuthData>
				}
			}
			response {}
		}
	}
}

/// The encrypted private data of a dehydrated device (MSC3814).
#[derive(Clone, Debug)]
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
	pub mod put_dehydrated_device {
		pub mod unstable {
			use alloc::{collections::BTreeMap, string::String};

			use crate::{
				OwnedDeviceId, OwnedOneTimeKeyId,
				device::DehydratedDeviceData,
				encryption::{DeviceKeys, OneTimeKey},
				serde::Raw,
			};

			crate::endpoint! {
				method: "PUT", path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
				request {
					path {}
					query {}
					body {
						device_id: OwnedDeviceId,
						device_data: Raw<DehydratedDeviceData>,
						device_keys: Raw<DeviceKeys>,
						one_time_keys: BTreeMap<OwnedOneTimeKeyId, Raw<OneTimeKey>>,
						fallback_keys: BTreeMap<OwnedOneTimeKeyId, Raw<OneTimeKey>>,
						initial_device_display_name: Option<String>
					}
				}
				response { device_id: OwnedDeviceId }
			}
		}
	}

	pub mod get_dehydrated_device {
		pub mod unstable {
			use crate::{OwnedDeviceId, device::DehydratedDeviceData, serde::Raw};

			crate::endpoint! {
				method: "GET", path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
				request { path {} query {} body {} }
				response { device_id: OwnedDeviceId, device_data: Raw<DehydratedDeviceData> }
			}
		}
	}

	pub mod delete_dehydrated_device {
		pub mod unstable {
			crate::endpoint! {
				method: "DELETE", path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device",
				request { path {} query {} body {} }
				response { device_id: crate::OwnedDeviceId }
			}
		}
	}

	pub mod get_events {
		pub mod unstable {
			use alloc::{string::String, vec::Vec};

			use crate::{OwnedDeviceId, events::AnyToDeviceEvent, serde::Raw};

			crate::endpoint! {
				method: "POST",
				path: "/_matrix/client/unstable/org.matrix.msc3814.v1/dehydrated_device/{device_id}/events",
				request {
					path { device_id: OwnedDeviceId }
					query {}
					body { next_batch: Option<String> }
				}
				response { events: Vec<Raw<AnyToDeviceEvent>>, next_batch: Option<String> }
			}
		}
	}
}

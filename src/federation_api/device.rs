//! Federation device-list endpoints.

use alloc::string::String;

use crate::{
	OwnedDeviceId, OwnedUserId, UInt,
	encryption::{CrossSigningKey, DeviceKeys},
	endpoint,
	serde::Raw,
};

/// Device-list queries.
pub mod get_devices {
	pub mod v1 {
		use super::super::{endpoint, OwnedUserId, UInt, Raw, CrossSigningKey, OwnedDeviceId, DeviceKeys, String};

		endpoint! {
			method: "GET",
			path: "/_matrix/federation/v1/user/devices/{user_id}",
			request {
				path { user_id: OwnedUserId }
				query {}
				body {}
			}
			response {
				user_id: OwnedUserId,
				stream_id: UInt,
				devices: alloc::vec::Vec<UserDevice>,
				master_key: Option<Raw<CrossSigningKey>>,
				self_signing_key: Option<Raw<CrossSigningKey>>,
			}
		}

		#[derive(Clone, Debug)]
		pub struct UserDevice {
			pub device_id: OwnedDeviceId,
			pub keys: Raw<DeviceKeys>,
			pub device_display_name: Option<String>,
		}

		crate::impl_codec_struct!(UserDevice {
			device_id: OwnedDeviceId,
			keys: Raw<DeviceKeys>,
			device_display_name: Option<String>,
		});
	}
}

#[cfg(test)]
mod tests {
	use super::get_devices::v1::UserDevice;
	use crate::{
		OwnedDeviceId, OwnedUserId,
		codec::{Deserialize, Serialize},
		encryption::DeviceKeys,
		serde::Raw,
	};
	use alloc::{collections::BTreeMap, vec::Vec};

	#[test]
	fn user_device_round_trip() {
		let device = UserDevice {
			device_id: OwnedDeviceId::from("DEVICE"),
			keys: Raw::from_value(&DeviceKeys::new(
				OwnedUserId::from("@alice:example.org"),
				OwnedDeviceId::from("DEVICE"),
				Vec::new(),
				BTreeMap::new(),
				BTreeMap::new(),
			)),
			device_display_name: Some("phone".into()),
		};
		let encoded = device.to_json();
		let decoded = UserDevice::from_json(&encoded).expect("device payload decodes");
		assert_eq!(decoded.device_id, device.device_id);
		assert_eq!(decoded.device_display_name, device.device_display_name);
		assert_eq!(decoded.keys, device.keys);
	}

	#[test]
	fn response_golden_and_request_path_round_trip() {
		use super::get_devices;
		use crate::endpoint::{EndpointRequest, EndpointResponse};
		let response = get_devices::v1::Response {
			user_id: OwnedUserId::from("@alice:example.org"),
			stream_id: 7_u64,
			devices: vec![UserDevice {
				device_id: OwnedDeviceId::from("DEVICE"),
				keys: Raw::from_json_text("{}").unwrap(),
				device_display_name: Some("phone".into()),
			}],
			master_key: None,
			self_signing_key: None,
		};
		assert_eq!(
			crate::codec::to_string(&response.to_body()),
			"{\"devices\":[{\"device_display_name\":\"phone\",\"device_id\":\"DEVICE\",\"keys\":{}}],\"stream_id\":7,\"user_id\":\"@alice:example.org\"}"
		);

		let request =
			get_devices::v1::Request::from_parts(&["@alice:example.org".to_owned()], &[], None)
				.unwrap();
		assert_eq!(request.path_args(), vec!["@alice:example.org"]);
	}
}

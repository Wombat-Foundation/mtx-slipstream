//! Federation device-list endpoints.

use alloc::string::String;

use crate::{
	OwnedDeviceId, OwnedUserId, UInt,
	encryption::{CrossSigningKey, DeviceKeys},
	sswire::Raw,
};

/// Device-list queries.
pub mod get_devices {
	pub mod v1 {
		use super::super::{
			CrossSigningKey, DeviceKeys, OwnedDeviceId, OwnedUserId, Raw, String, UInt,
		};

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
			method: "GET",
			path: "/_matrix/federation/v1/user/devices/{user_id}",
			path { user_id }
			query {}
			body {}
		}
		pub struct Response {
			pub user_id: OwnedUserId,
			pub stream_id: UInt,
			pub devices: alloc::vec::Vec<UserDevice>,
			pub master_key: Option<Raw<CrossSigningKey>>,
			pub self_signing_key: Option<Raw<CrossSigningKey>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { user_id => "user_id", stream_id => "stream_id", devices => "devices", master_key => "master_key", self_signing_key => "self_signing_key" }
		}

		#[derive(Debug)]
		pub struct UserDevice {
			pub device_id: OwnedDeviceId,
			pub keys: Raw<DeviceKeys>,
			pub device_display_name: Option<String>,
		}

		impl crate::codec::Serialize for UserDevice {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(device_id), crate::endpoint::enc(&self.device_id)),
					(stringify!(keys), crate::endpoint::enc(&self.keys)),
					(
						stringify!(device_display_name),
						crate::endpoint::enc(&self.device_display_name),
					),
				])
			}
		}
		impl crate::codec::Deserialize for UserDevice {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(UserDevice)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					device_id: input.body(stringify!(device_id))?,
					keys: input.body(stringify!(keys))?,
					device_display_name: input.body(stringify!(device_display_name))?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::get_devices::v1::UserDevice;
	use crate::{
		OwnedDeviceId, OwnedUserId,
		codec::{Deserialize, Serialize},
		encryption::DeviceKeys,
		sswire::Raw,
	};
	use alloc::{collections::BTreeMap, vec::Vec};

	#[test]
	fn user_device_round_trip() {
		let device = UserDevice {
			device_id: OwnedDeviceId::parse("DEVICE").unwrap(),
			keys: Raw::from_value(&DeviceKeys::new(
				OwnedUserId::parse("@alice:example.org").unwrap(),
				OwnedDeviceId::parse("DEVICE").unwrap(),
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
			user_id: OwnedUserId::parse("@alice:example.org").unwrap(),
			stream_id: 7_u64,
			devices: vec![UserDevice {
				device_id: OwnedDeviceId::parse("DEVICE").unwrap(),
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

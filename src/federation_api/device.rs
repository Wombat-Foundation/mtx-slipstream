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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/federation/v1/user/devices/{user_id}",
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
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("user_id", crate::endpoint::enc(&self.user_id)),
					("stream_id", crate::endpoint::enc(&self.stream_id)),
					("devices", crate::endpoint::enc(&self.devices)),
					("master_key", crate::endpoint::enc(&self.master_key)),
					("self_signing_key", crate::endpoint::enc(&self.self_signing_key)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					user_id: _input.body("user_id")?,
					stream_id: _input.body("stream_id")?,
					devices: _input.body("devices")?,
					master_key: _input.body("master_key")?,
					self_signing_key: _input.body("self_signing_key")?,
				})
			}
		}

		#[derive(Debug)]
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

//! Client to-device messaging.

use alloc::{collections::BTreeMap, string::String};

use crate::{OwnedTransactionId, OwnedUserId, endpoint, events::AnyToDeviceEvent, serde::Raw};

pub mod send_event_to_device {
	pub mod v3 {
		use super::super::*;

		pub type Messages = BTreeMap<
			OwnedUserId,
			BTreeMap<crate::to_device::DeviceIdOrAllDevices, Raw<AnyToDeviceEvent>>,
		>;

		endpoint! {
			method: "PUT",
			path: "/_matrix/client/v3/sendToDevice/{event_type}/{txn_id}",
			request {
				path { event_type: String, txn_id: OwnedTransactionId }
				query {}
				body { messages: Messages }
			}
			response {}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::send_event_to_device::v3::{Messages, Request};
	use crate::{
		OwnedTransactionId,
		endpoint::{AuthScheme, EndpointRequest},
	};

	#[test]
	fn request_shape_and_auth_are_pinned() {
		let request = Request {
			event_type: "m.test".into(),
			txn_id: OwnedTransactionId::from("txn"),
			messages: Messages::new(),
		};
		assert_eq!(Request::METADATA.authentication, AuthScheme::AccessToken);
		assert_eq!(request.path_args(), vec!["m.test", "txn"]);
		assert_eq!(crate::codec::to_string(&request.body().unwrap()), "{\"messages\":{}}");
	}
}

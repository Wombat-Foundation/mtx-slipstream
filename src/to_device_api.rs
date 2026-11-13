//! Client to-device messaging.

use alloc::{collections::BTreeMap, string::String};

use crate::{OwnedTransactionId, OwnedUserId, events::AnyToDeviceEvent, sswire::Raw};

pub mod send_event_to_device {
	pub mod v3 {
		use super::super::{
			AnyToDeviceEvent, BTreeMap, OwnedTransactionId, OwnedUserId, Raw, String,
		};

		pub type Messages = BTreeMap<
			OwnedUserId,
			BTreeMap<crate::to_device::DeviceIdOrAllDevices, Raw<AnyToDeviceEvent>>,
		>;

		pub struct Request {
			pub event_type: String,
			pub txn_id: OwnedTransactionId,
			pub messages: Messages,
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
			path: "/_matrix/client/v3/sendToDevice/{event_type}/{txn_id}",
			path { event_type, txn_id }
			query {}
			body { messages => "messages" }
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
			txn_id: OwnedTransactionId::parse("txn").unwrap(),
			messages: Messages::new(),
		};
		assert_eq!(Request::METADATA.authentication, AuthScheme::AccessToken);
		assert_eq!(request.path_args(), vec!["m.test", "txn"]);
		assert_eq!(crate::codec::to_string(&request.body().unwrap()), "{\"messages\":{}}");
	}
}

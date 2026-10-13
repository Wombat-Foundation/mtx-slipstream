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
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/v3/sendToDevice/{event_type}/{txn_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.event_type),
					crate::endpoint::path_param(&self.txn_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("messages", crate::endpoint::enc(&self.messages))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					event_type: input.path()?,
					txn_id: input.path()?,
					messages: input.body("messages")?,
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

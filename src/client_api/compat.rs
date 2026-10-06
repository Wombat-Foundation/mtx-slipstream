pub mod read_marker {
	pub mod set_read_marker {
		pub mod v3 {
			pub use crate::events::receipt::{ReceiptThread, ReceiptType};
			use crate::{OwnedEventId, OwnedRoomId, endpoint};
			endpoint! { method: "POST", path: "/_matrix/client/v3/rooms/{roomId}/read_markers", request { path { room_id: OwnedRoomId } query {} body { event_id: OwnedEventId, fully_read: Option<OwnedEventId>, read_receipt: Option<OwnedEventId>, private_read_receipt: Option<OwnedEventId>, receipt_type: ReceiptType, thread: ReceiptThread } } response {} }
		}
	}
}
pub mod receipt {
	pub mod create_receipt {
		pub mod v3 {
			pub use crate::events::receipt::{ReceiptThread, ReceiptType};
			use crate::{OwnedEventId, OwnedRoomId, endpoint};
			endpoint! {
				method: "POST",
				path: "/_matrix/client/v3/rooms/{roomId}/receipt/{receiptType}/{eventId}",
				request {
					path {
						room_id: OwnedRoomId,
						receipt_type: ReceiptType,
						event_id: OwnedEventId
					}
					query {}
					body { thread => "thread_id": ReceiptThread = default }
				}
				response {}
			}
		}
	}
}
pub mod thirdparty {
	pub mod get_protocols {
		pub mod v3 {
			use crate::endpoint;
			endpoint! { method: "GET", path: "/_matrix/client/v3/thirdparty/protocols", request { path {} query {} body {} } response { protocols: std::collections::BTreeMap<String, crate::json::Value> } }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{read_marker, receipt};
	use crate::{
		OwnedEventId, OwnedRoomId,
		endpoint::EndpointRequest,
		events::receipt::{ReceiptThread, ReceiptType},
	};

	fn request(thread: ReceiptThread) -> receipt::create_receipt::v3::Request {
		receipt::create_receipt::v3::Request {
			room_id: OwnedRoomId::parse("!room:example.org").unwrap(),
			receipt_type: ReceiptType::Read,
			event_id: OwnedEventId::parse("$event:example.org").unwrap(),
			thread,
		}
	}

	#[test]
	fn create_receipt_uses_ruma_field_and_wire_names() {
		let body = request(ReceiptThread::Main).body().unwrap();
		let object = body.as_object().unwrap();
		assert_eq!(object.get("thread_id").and_then(crate::json::Value::as_str), Some("main"));
		assert!(!object.contains_key("thread"));

		let parsed = receipt::create_receipt::v3::Request::from_parts(
			&["!room:example.org".into(), "m.read".into(), "$event:example.org".into()],
			&[],
			Some(&body),
		)
		.unwrap();
		assert_eq!(parsed.thread, ReceiptThread::Main);
	}

	#[test]
	fn create_receipt_defaults_missing_thread() {
		let parsed = receipt::create_receipt::v3::Request::from_parts(
			&["!room:example.org".into(), "m.read".into(), "$event:example.org".into()],
			&[],
			Some(&crate::json::Value::Object(crate::json::Object::new())),
		)
		.unwrap();
		assert_eq!(parsed.thread, ReceiptThread::Unthreaded);
	}

	#[test]
	fn create_receipt_has_distinct_route_metadata() {
		assert_eq!(
			<receipt::create_receipt::v3::Request as EndpointRequest>::METADATA.path,
			"/_matrix/client/v3/rooms/{roomId}/receipt/{receiptType}/{eventId}"
		);
		assert_ne!(
			<receipt::create_receipt::v3::Request as EndpointRequest>::METADATA.path,
			<read_marker::set_read_marker::v3::Request as EndpointRequest>::METADATA.path
		);
	}
}

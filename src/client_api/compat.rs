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
	pub use super::read_marker::set_read_marker as create_receipt;
}
pub mod thirdparty {
	pub mod get_protocols {
		pub mod v3 {
			use crate::endpoint;
			endpoint! { method: "GET", path: "/_matrix/client/v3/thirdparty/protocols", request { path {} query {} body {} } response { protocols: std::collections::BTreeMap<String, crate::json::Value> } }
		}
	}
}

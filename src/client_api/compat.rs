pub mod read_marker {
	pub mod set_read_marker {
		pub mod v3 {
			pub use crate::events::receipt::{ReceiptThread, ReceiptType};
			use crate::{OwnedEventId, OwnedRoomId};
			pub struct Request {
				pub room_id: OwnedRoomId,
				pub fully_read: Option<OwnedEventId>,
				pub read_receipt: Option<OwnedEventId>,
				pub private_read_receipt: Option<OwnedEventId>,
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
					"POST",
					"/_matrix/client/v3/rooms/{roomId}/read_markers",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(
						&self.room_id,
					)])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [
							("m.fully_read", crate::endpoint::enc(&self.fully_read)),
							("m.read", crate::endpoint::enc(&self.read_receipt)),
							("m.read.private", crate::endpoint::enc(&self.private_read_receipt)),
						],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						room_id: input.path()?,
						fully_read: input.body_or_default("m.fully_read")?,
						read_receipt: input.body_or_default("m.read")?,
						private_read_receipt: input.body_or_default("m.read.private")?,
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
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {})
				}
			}
		}
	}
}
pub mod receipt {
	pub mod create_receipt {
		pub mod v3 {
			pub use crate::events::receipt::{ReceiptThread, ReceiptType};
			use crate::{OwnedEventId, OwnedRoomId};
			pub struct Request {
				pub room_id: OwnedRoomId,
				pub receipt_type: ReceiptType,
				pub event_id: OwnedEventId,
				pub thread: ReceiptThread,
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
					"POST",
					"/_matrix/client/v3/rooms/{roomId}/receipt/{receiptType}/{eventId}",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [
						crate::endpoint::path_param(&self.room_id),
						crate::endpoint::path_param(&self.receipt_type),
						crate::endpoint::path_param(&self.event_id),
					])
				}
				fn query(&self) -> crate::endpoint::Pairs {
					crate::endpoint::query_pairs_mut(&mut [])
				}
				fn body(&self) -> Option<crate::json::Value> {
					crate::endpoint::body_value(
						<Self as crate::endpoint::EndpointRequest>::METADATA.method,
						&mut [("thread_id", crate::endpoint::enc(&self.thread))],
					)
				}
				fn from_parts(
					path: &[crate::endpoint::Str],
					query: &[crate::endpoint::Pair],
					body: Option<&crate::json::Value>,
				) -> crate::endpoint::Parsed<Self> {
					let input = crate::endpoint::Input::new(path, query, body);
					let value = Self {
						room_id: input.path()?,
						receipt_type: input.path()?,
						event_id: input.path()?,
						thread: input.body_or_default("thread_id")?,
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
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {})
				}
			}
		}
	}
}
pub mod thirdparty {
	pub mod get_protocols {
		pub mod v3 {

			pub struct Request {}
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
					"/_matrix/client/v3/thirdparty/protocols",
				);
				fn path_args(&self) -> crate::endpoint::Strs {
					crate::endpoint::path_args_from(&mut [])
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
					let value = Self {};
					input.finish()?;
					Ok(value)
				}
			}
			pub struct Response {
				pub protocols: std::collections::BTreeMap<String, crate::json::Value>,
			}
			impl ::core::fmt::Debug for Response {
				fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
					crate::endpoint::opaque_debug(f, "Response")
				}
			}
			impl crate::endpoint::EndpointResponse for Response {
				fn to_body(&self) -> crate::json::Value {
					crate::endpoint::body_object(&mut [(
						"protocols",
						crate::endpoint::enc(&self.protocols),
					)])
				}
				fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
					let _input = crate::endpoint::Input::body_only(body);
					Ok(Self {
						protocols: _input.body("protocols")?,
					})
				}
			}
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

#[cfg(test)]
mod read_marker_tests {
	use super::read_marker::set_read_marker::v3::Request;
	use crate::{
		endpoint::EndpointRequest,
		json::{Object, Value},
	};

	#[test]
	fn read_markers_body_fields_are_all_optional() {
		let path = ["!room:example.org".to_owned()];
		let empty = Value::Object(Object::new());
		let request = Request::from_parts(&path, &[], Some(&empty)).unwrap();
		assert!(request.fully_read.is_none());
		assert!(request.read_receipt.is_none());
		assert!(request.private_read_receipt.is_none());

		let request = Request::from_parts(&path, &[], None).unwrap();
		assert!(request.fully_read.is_none());
	}

	#[test]
	fn read_markers_use_the_spec_wire_keys() {
		let path = ["!room:example.org".to_owned()];
		let body = Value::parse(
			r#"{"m.fully_read":"$a","m.read":"$b","m.read.private":"$c","read_receipt":"$ignored"}"#,
		)
		.unwrap();
		let request = Request::from_parts(&path, &[], Some(&body)).unwrap();
		assert_eq!(request.fully_read.as_deref(), Some("$a"));
		assert_eq!(request.read_receipt.as_deref(), Some("$b"));
		assert_eq!(request.private_read_receipt.as_deref(), Some("$c"));
	}
}

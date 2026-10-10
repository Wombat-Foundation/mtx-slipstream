pub use crate::client_api::message_events::get_message_events;

pub mod send_message_event {
	pub mod v3 {
		use crate::{
			MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedTransactionId,
			events::MessageLikeEventType, sswire::Raw,
		};

		// The request body is the event content as given, not an object with a
		// `body` key. (`delay` is a query parameter handled by the router.)
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_type: MessageLikeEventType,
			pub txn_id: OwnedTransactionId,
			pub ts: Option<MilliSecondsSinceUnixEpoch>,
			pub body: Raw<crate::events::AnyMessageLikeEventContent>,
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
				"/_matrix/client/v3/rooms/{roomId}/send/{eventType}/{txnId}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_type),
					crate::endpoint::path_param(&self.txn_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [("ts", crate::endpoint::enc(&self.ts))])
			}
			fn body(&self) -> Option<crate::json::Value> {
				Some(crate::endpoint::enc(&self.body))
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					event_type: input.path()?,
					txn_id: input.path()?,
					ts: input.query("ts")?,
					body: crate::codec::Deserialize::from_json(
						body.ok_or_else(|| crate::codec::DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub event_id: OwnedEventId,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"event_id",
					crate::endpoint::enc(&self.event_id),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					event_id: input.body("event_id")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::send_message_event::v3::Request;
	use crate::{endpoint::EndpointRequest, json::Value};

	fn path() -> [String; 3] {
		["!room:example.org".into(), "m.reaction".into(), "1".into()]
	}

	#[test]
	fn send_message_event_takes_the_whole_body_as_content() {
		// A reaction has no `body` key; the content must still come through.
		let content = Value::parse(
			r#"{"m.relates_to":{"rel_type":"m.annotation","event_id":"$e","key":"x"}}"#,
		)
		.unwrap();
		let request = Request::from_parts(&path(), &[], Some(&content)).unwrap();
		assert!(request.body.0.contains("m.relates_to"));
		assert!(request.ts.is_none());
	}

	#[test]
	fn send_message_event_round_trips_content_with_a_body_key() {
		let content = Value::parse(r#"{"msgtype":"m.text","body":"hello"}"#).unwrap();
		let request = Request::from_parts(&path(), &[], Some(&content)).unwrap();
		assert!(request.body.0.contains("msgtype") && request.body.0.contains("hello"));
		assert_eq!(request.body(), Some(content));
	}
}

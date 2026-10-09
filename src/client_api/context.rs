pub mod get_context {
	pub mod v3 {
		use crate::{OwnedEventId, OwnedRoomId, UInt, filter::RoomEventFilter, sswire::Raw};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub limit: UInt,
			pub filter: Option<RoomEventFilter>,
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
				"/_matrix/client/v3/rooms/{roomId}/context/{eventId}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("limit", crate::endpoint::enc(&self.limit)),
					("filter", crate::endpoint::enc(&self.filter)),
				])
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
					room_id: input.path()?,
					event_id: input.path()?,
					limit: input.query_or("limit", 10)?,
					filter: input.query("filter")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub start: Option<String>,
			pub end: Option<String>,
			pub events_before: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub event: Option<Raw<crate::events::AnyTimelineEvent>>,
			pub events_after: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub state: Vec<Raw<crate::events::AnyStateEvent>>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("start", crate::endpoint::enc(&self.start)),
					("end", crate::endpoint::enc(&self.end)),
					("events_before", crate::endpoint::enc(&self.events_before)),
					("event", crate::endpoint::enc(&self.event)),
					("events_after", crate::endpoint::enc(&self.events_after)),
					("state", crate::endpoint::enc(&self.state)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let _input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					start: _input.body("start")?,
					end: _input.body("end")?,
					events_before: _input.body("events_before")?,
					event: _input.body("event")?,
					events_after: _input.body("events_after")?,
					state: _input.body("state")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::get_context::v3::Request;
	use crate::endpoint::EndpointRequest;

	#[test]
	fn context_limit_defaults_to_ten() {
		let path = ["!room:example.org".to_owned(), "$event".to_owned()];
		let request = Request::from_parts(&path, &[], None).unwrap();
		assert_eq!(request.limit, 10);
		assert!(request.filter.is_none());
	}
}

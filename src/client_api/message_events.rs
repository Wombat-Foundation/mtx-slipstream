pub mod get_message_events {
	pub mod v3 {
		use crate::{OwnedRoomId, UInt, api::Direction, filter::RoomEventFilter, sswire::Raw};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub from: Option<String>,
			pub to: Option<String>,
			pub dir: Direction,
			pub limit: UInt,
			pub filter: RoomEventFilter,
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
				"/_matrix/client/v3/rooms/{roomId}/messages",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("from", crate::endpoint::enc(&self.from)),
					("to", crate::endpoint::enc(&self.to)),
					("dir", crate::endpoint::enc(&self.dir)),
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
					from: input.query("from")?,
					to: input.query("to")?,
					dir: input.query("dir")?,
					limit: input.query_or("limit", 10)?,
					filter: input.query_or("filter", RoomEventFilter::default())?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub start: String,
			pub end: Option<String>,
			pub chunk: Vec<Raw<crate::events::AnyTimelineEvent>>,
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
					("chunk", crate::endpoint::enc(&self.chunk)),
					("state", crate::endpoint::enc(&self.state)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					start: input.body("start")?,
					end: input.body("end")?,
					chunk: input.body("chunk")?,
					state: input.body("state")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::get_message_events::v3::Request;
	use crate::{api::Direction, endpoint::EndpointRequest};

	fn parse(query: &[(&str, &str)]) -> Request {
		let path = ["!room:example.org".to_owned()];
		let query: Vec<_> =
			query.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect();
		Request::from_parts(&path, &query, None).unwrap()
	}

	#[test]
	fn messages_query_requires_direction() {
		assert!(Request::from_parts(&["!room:example.org".to_owned()], &[], None).is_err());
	}

	#[test]
	fn messages_query_explicit_values_win() {
		let request = parse(&[("limit", "3"), ("dir", "f"), ("from", "t1")]);
		assert_eq!(request.limit, 3);
		assert_eq!(request.dir, Direction::Forward);
		assert_eq!(request.from.as_deref(), Some("t1"));
	}
}

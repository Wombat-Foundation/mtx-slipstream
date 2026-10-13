pub mod get_relating_events {
	pub mod v1 {
		use crate::{OwnedEventId, OwnedRoomId, UInt, api::Direction, sswire::Raw};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub from: Option<String>,
			pub to: Option<String>,
			pub limit: Option<UInt>,
			pub dir: Direction,
			pub recurse: bool,
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
				"/_matrix/client/v1/rooms/{roomId}/relations/{eventId}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("from", crate::endpoint::enc(&self.from)),
					("to", crate::endpoint::enc(&self.to)),
					("limit", crate::endpoint::enc(&self.limit)),
					("dir", crate::endpoint::enc(&self.dir)),
					("recurse", crate::endpoint::enc(&self.recurse)),
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
					from: input.query("from")?,
					to: input.query("to")?,
					limit: input.query("limit")?,
					dir: input.query_or("dir", Direction::Backward)?,
					recurse: input.query_or("recurse", false)?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub chunk: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub next_batch: Option<String>,
			pub prev_batch: Option<String>,
			pub recursion_depth: Option<UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("chunk", crate::endpoint::enc(&self.chunk)),
					("next_batch", crate::endpoint::enc(&self.next_batch)),
					("prev_batch", crate::endpoint::enc(&self.prev_batch)),
					("recursion_depth", crate::endpoint::enc(&self.recursion_depth)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					chunk: input.body("chunk")?,
					next_batch: input.body("next_batch")?,
					prev_batch: input.body("prev_batch")?,
					recursion_depth: input.body("recursion_depth")?,
				})
			}
		}
	}
}
pub mod get_relating_events_with_rel_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt, api::Direction, events::relation::RelationType,
			sswire::Raw,
		};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub rel_type: RelationType,
			pub from: Option<String>,
			pub to: Option<String>,
			pub limit: UInt,
			pub dir: Direction,
			pub recurse: Option<bool>,
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
				"/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_id),
					crate::endpoint::path_param(&self.rel_type),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("from", crate::endpoint::enc(&self.from)),
					("to", crate::endpoint::enc(&self.to)),
					("limit", crate::endpoint::enc(&self.limit)),
					("dir", crate::endpoint::enc(&self.dir)),
					("recurse", crate::endpoint::enc(&self.recurse)),
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
					rel_type: input.path()?,
					from: input.query("from")?,
					to: input.query("to")?,
					limit: input.query_or("limit", 10)?,
					dir: input.query_or("dir", Direction::Backward)?,
					recurse: input.query("recurse")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub chunk: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub next_batch: Option<String>,
			pub prev_batch: Option<String>,
			pub recursion_depth: Option<UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("chunk", crate::endpoint::enc(&self.chunk)),
					("next_batch", crate::endpoint::enc(&self.next_batch)),
					("prev_batch", crate::endpoint::enc(&self.prev_batch)),
					("recursion_depth", crate::endpoint::enc(&self.recursion_depth)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					chunk: input.body("chunk")?,
					next_batch: input.body("next_batch")?,
					prev_batch: input.body("prev_batch")?,
					recursion_depth: input.body("recursion_depth")?,
				})
			}
		}
	}
}
pub mod get_relating_events_with_rel_type_and_event_type {
	pub mod v1 {
		use crate::{
			OwnedEventId, OwnedRoomId, UInt,
			api::Direction,
			events::{TimelineEventType, relation::RelationType},
			sswire::Raw,
		};
		pub struct Request {
			pub room_id: OwnedRoomId,
			pub event_id: OwnedEventId,
			pub rel_type: RelationType,
			pub event_type: TimelineEventType,
			pub from: Option<String>,
			pub to: Option<String>,
			pub limit: UInt,
			pub dir: Direction,
			pub recurse: Option<bool>,
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
				"/_matrix/client/v1/rooms/{roomId}/relations/{eventId}/{relType}/{eventType}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.event_id),
					crate::endpoint::path_param(&self.rel_type),
					crate::endpoint::path_param(&self.event_type),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [
					("from", crate::endpoint::enc(&self.from)),
					("to", crate::endpoint::enc(&self.to)),
					("limit", crate::endpoint::enc(&self.limit)),
					("dir", crate::endpoint::enc(&self.dir)),
					("recurse", crate::endpoint::enc(&self.recurse)),
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
					rel_type: input.path()?,
					event_type: input.path()?,
					from: input.query("from")?,
					to: input.query("to")?,
					limit: input.query_or("limit", 10)?,
					dir: input.query_or("dir", Direction::Backward)?,
					recurse: input.query("recurse")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub chunk: Vec<Raw<crate::events::AnyTimelineEvent>>,
			pub next_batch: Option<String>,
			pub prev_batch: Option<String>,
			pub recursion_depth: Option<UInt>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("chunk", crate::endpoint::enc(&self.chunk)),
					("next_batch", crate::endpoint::enc(&self.next_batch)),
					("prev_batch", crate::endpoint::enc(&self.prev_batch)),
					("recursion_depth", crate::endpoint::enc(&self.recursion_depth)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					chunk: input.body("chunk")?,
					next_batch: input.body("next_batch")?,
					prev_batch: input.body("prev_batch")?,
					recursion_depth: input.body("recursion_depth")?,
				})
			}
		}
	}
}

pub mod event_relationships {
	pub mod unstable {
		use crate::{OwnedEventId, OwnedRoomId, sswire::Raw};

		pub struct Request {
			pub event_id: OwnedEventId,
			pub room_id: Option<OwnedRoomId>,
			pub max_depth: Option<i64>,
			pub max_breadth: Option<i64>,
			pub limit: Option<i64>,
			pub depth_first: Option<bool>,
			pub recent_first: Option<bool>,
			pub include_parent: Option<bool>,
			pub include_children: Option<bool>,
			pub direction: Option<String>,
			pub batch: Option<String>,
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
				"/_matrix/client/unstable/event_relationships",
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
					&mut [
						("event_id", crate::endpoint::enc(&self.event_id)),
						("room_id", crate::endpoint::enc(&self.room_id)),
						("max_depth", crate::endpoint::enc(&self.max_depth)),
						("max_breadth", crate::endpoint::enc(&self.max_breadth)),
						("limit", crate::endpoint::enc(&self.limit)),
						("depth_first", crate::endpoint::enc(&self.depth_first)),
						("recent_first", crate::endpoint::enc(&self.recent_first)),
						("include_parent", crate::endpoint::enc(&self.include_parent)),
						("include_children", crate::endpoint::enc(&self.include_children)),
						("direction", crate::endpoint::enc(&self.direction)),
						("batch", crate::endpoint::enc(&self.batch)),
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
					event_id: input.body("event_id")?,
					room_id: input.body("room_id")?,
					max_depth: input.body("max_depth")?,
					max_breadth: input.body("max_breadth")?,
					limit: input.body("limit")?,
					depth_first: input.body("depth_first")?,
					recent_first: input.body("recent_first")?,
					include_parent: input.body("include_parent")?,
					include_children: input.body("include_children")?,
					direction: input.body("direction")?,
					batch: input.body("batch")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub events: Vec<Raw<crate::json::Value>>,
			pub next_batch: Option<String>,
			pub limited: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("events", crate::endpoint::enc(&self.events)),
					("next_batch", crate::endpoint::enc(&self.next_batch)),
					("limited", crate::endpoint::enc(&self.limited)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					events: input.body("events")?,
					next_batch: input.body("next_batch")?,
					limited: input.body("limited")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{api::Direction, endpoint::EndpointRequest};

	fn path(extra: &[&str]) -> Vec<String> {
		["!room:example.org", "$event"].iter().chain(extra).map(|s| (*s).to_owned()).collect()
	}

	#[test]
	fn relations_query_defaults() {
		let request =
			super::get_relating_events::v1::Request::from_parts(&path(&[]), &[], None).unwrap();
		assert_eq!(request.dir, Direction::Backward);
		assert!(!request.recurse);
		assert!(request.limit.is_none());

		let request = super::get_relating_events_with_rel_type::v1::Request::from_parts(
			&path(&["m.annotation"]),
			&[],
			None,
		)
		.unwrap();
		assert_eq!(request.dir, Direction::Backward);
		assert_eq!(request.limit, 10);
	}
}

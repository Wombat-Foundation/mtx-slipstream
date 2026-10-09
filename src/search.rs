//! Server-side event search.

pub mod search_events {
	pub mod v3 {
		use alloc::{collections::BTreeMap, string::String, vec::Vec};

		use crate::{
			OwnedMxcUri, OwnedRoomId, OwnedUserId, UInt,
			codec::{DeError, Deserialize, Serialize},
			endpoint::{Input, object_from},
			federation_api::RawPdu,
			filter::RoomEventFilter,
			json::Value,
			sswire::Raw,
		};

		/// How much surrounding context to return with each hit.
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub struct EventContext {
			pub before_limit: UInt,
			pub after_limit: UInt,
			pub include_profile: bool,
		}

		impl Default for EventContext {
			fn default() -> Self {
				Self {
					before_limit: 5,
					after_limit: 5,
					include_profile: false,
				}
			}
		}

		impl Serialize for EventContext {
			fn to_json(&self) -> Value {
				Value::Object(object_from(alloc::vec![
					("before_limit", self.before_limit.to_json()),
					("after_limit", self.after_limit.to_json()),
					("include_profile", self.include_profile.to_json()),
				]))
			}
		}

		impl Deserialize for EventContext {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				let input = Input::new(&[], &[], Some(value));
				let default = Self::default();
				Ok(Self {
					before_limit: input
						.body::<Option<UInt>>("before_limit")?
						.unwrap_or(default.before_limit),
					after_limit: input
						.body::<Option<UInt>>("after_limit")?
						.unwrap_or(default.after_limit),
					include_profile: input.body_or_default("include_profile")?,
				})
			}
		}

		/// What to search for in one category.
		#[derive(Debug, Default)]
		pub struct Criteria {
			pub search_term: String,
			pub event_context: EventContext,
			pub filter: RoomEventFilter,
			pub include_state: Option<bool>,
		}

		impl crate::codec::Serialize for Criteria {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(search_term), crate::endpoint::enc(&self.search_term)),
					(stringify!(event_context), crate::endpoint::enc(&self.event_context)),
					(stringify!(filter), crate::endpoint::enc(&self.filter)),
					(stringify!(include_state), crate::endpoint::enc(&self.include_state)),
				])
			}
		}
		impl crate::codec::Deserialize for Criteria {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(Criteria)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					search_term: input.body(stringify!(search_term))?,
					event_context: input.body_or_default(stringify!(event_context))?,
					filter: input.body_or_default(stringify!(filter))?,
					include_state: input.body_or_default(stringify!(include_state))?,
				})
			}
		}

		#[derive(Debug, Default)]
		pub struct Categories {
			pub room_events: Option<Criteria>,
		}

		impl crate::codec::Serialize for Categories {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					stringify!(room_events),
					crate::endpoint::enc(&self.room_events),
				)])
			}
		}
		impl crate::codec::Deserialize for Categories {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(Categories)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					room_events: input.body_or_default(stringify!(room_events))?,
				})
			}
		}

		/// A user's profile, as returned alongside context events.
		#[derive(Debug, Default)]
		pub struct UserProfile {
			pub displayname: Option<String>,
			pub avatar_url: Option<OwnedMxcUri>,
		}

		impl crate::codec::Serialize for UserProfile {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(displayname), crate::endpoint::enc(&self.displayname)),
					(stringify!(avatar_url), crate::endpoint::enc(&self.avatar_url)),
				])
			}
		}
		impl crate::codec::Deserialize for UserProfile {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(UserProfile)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					displayname: input.body_or_default(stringify!(displayname))?,
					avatar_url: input.body_or_default(stringify!(avatar_url))?,
				})
			}
		}

		/// Events around a hit.
		#[derive(Debug, Default)]
		pub struct EventContextResult {
			pub end: Option<String>,
			pub events_after: Vec<RawPdu>,
			pub events_before: Vec<RawPdu>,
			pub profile_info: BTreeMap<OwnedUserId, UserProfile>,
			pub start: Option<String>,
		}

		impl crate::codec::Serialize for EventContextResult {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(end), crate::endpoint::enc(&self.end)),
					(stringify!(events_after), crate::endpoint::enc(&self.events_after)),
					(stringify!(events_before), crate::endpoint::enc(&self.events_before)),
					(stringify!(profile_info), crate::endpoint::enc(&self.profile_info)),
					(stringify!(start), crate::endpoint::enc(&self.start)),
				])
			}
		}
		impl crate::codec::Deserialize for EventContextResult {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(EventContextResult)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					end: input.body_or_default(stringify!(end))?,
					events_after: input.body_or_default(stringify!(events_after))?,
					events_before: input.body_or_default(stringify!(events_before))?,
					profile_info: input.body_or_default(stringify!(profile_info))?,
					start: input.body_or_default(stringify!(start))?,
				})
			}
		}

		#[derive(Debug, Default)]
		pub struct SearchResult {
			pub context: EventContextResult,
			pub rank: Option<f64>,
			pub result: Option<RawPdu>,
		}

		impl crate::codec::Serialize for SearchResult {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(context), crate::endpoint::enc(&self.context)),
					(stringify!(rank), crate::endpoint::enc(&self.rank)),
					(stringify!(result), crate::endpoint::enc(&self.result)),
				])
			}
		}
		impl crate::codec::Deserialize for SearchResult {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(SearchResult)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					context: input.body_or_default(stringify!(context))?,
					rank: input.body_or_default(stringify!(rank))?,
					result: input.body_or_default(stringify!(result))?,
				})
			}
		}

		/// The results for the `room_events` category.
		#[derive(Debug, Default)]
		pub struct ResultRoomEvents {
			pub count: Option<UInt>,
			/// Result groupings; not produced by this server.
			pub groups: BTreeMap<String, Value>,
			pub highlights: Vec<String>,
			pub next_batch: Option<String>,
			pub results: Vec<SearchResult>,
			pub state: BTreeMap<OwnedRoomId, Vec<Raw<crate::events::AnyStateEvent>>>,
		}

		impl crate::codec::Serialize for ResultRoomEvents {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					(stringify!(count), crate::endpoint::enc(&self.count)),
					(stringify!(groups), crate::endpoint::enc(&self.groups)),
					(stringify!(highlights), crate::endpoint::enc(&self.highlights)),
					(stringify!(next_batch), crate::endpoint::enc(&self.next_batch)),
					(stringify!(results), crate::endpoint::enc(&self.results)),
					(stringify!(state), crate::endpoint::enc(&self.state)),
				])
			}
		}
		impl crate::codec::Deserialize for ResultRoomEvents {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(ResultRoomEvents)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					count: input.body_or_default(stringify!(count))?,
					groups: input.body_or_default(stringify!(groups))?,
					highlights: input.body_or_default(stringify!(highlights))?,
					next_batch: input.body_or_default(stringify!(next_batch))?,
					results: input.body_or_default(stringify!(results))?,
					state: input.body_or_default(stringify!(state))?,
				})
			}
		}

		#[derive(Debug, Default)]
		pub struct ResultCategories {
			pub room_events: ResultRoomEvents,
		}

		impl crate::codec::Serialize for ResultCategories {
			fn to_json(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					stringify!(room_events),
					crate::endpoint::enc(&self.room_events),
				)])
			}
		}
		impl crate::codec::Deserialize for ResultCategories {
			fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				// A struct is a JSON object; anything else is malformed, not "all defaults".
				if value.as_object().is_none() {
					return Err(crate::codec::DeError::expected(stringify!(ResultCategories)));
				}
				let input = crate::endpoint::Input::body_only(value);
				Ok(Self {
					room_events: input.body_or_default(stringify!(room_events))?,
				})
			}
		}

		pub struct Request {
			pub next_batch: Option<String>,
			pub search_categories: Categories,
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
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/search");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"next_batch",
					crate::endpoint::enc(&self.next_batch),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("search_categories", crate::endpoint::enc(&self.search_categories))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					next_batch: input.query("next_batch")?,
					search_categories: input.body("search_categories")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub search_categories: ResultCategories,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"search_categories",
					crate::endpoint::enc(&self.search_categories),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					search_categories: input.body("search_categories")?,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::search_events::v3::*;
	use crate::codec::from_str;

	#[test]
	fn context_defaults_to_five_each_side() {
		let criteria: Criteria = from_str(r#"{"search_term":"hello"}"#).unwrap();
		assert_eq!(criteria.event_context.before_limit, 5);
		assert_eq!(criteria.event_context.after_limit, 5);
		assert_eq!(criteria.include_state, None);
	}
}

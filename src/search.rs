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
			impl_codec_struct,
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

		impl_codec_struct!(Criteria { search_term: String } default {
			event_context: EventContext,
			filter: RoomEventFilter,
			include_state: Option<bool>,
		});

		#[derive(Debug, Default)]
		pub struct Categories {
			pub room_events: Option<Criteria>,
		}

		impl_codec_struct!(Categories {} default { room_events: Option<Criteria> });

		/// A user's profile, as returned alongside context events.
		#[derive(Debug, Default)]
		pub struct UserProfile {
			pub displayname: Option<String>,
			pub avatar_url: Option<OwnedMxcUri>,
		}

		impl_codec_struct!(UserProfile {} default {
			displayname: Option<String>,
			avatar_url: Option<OwnedMxcUri>,
		});

		/// Events around a hit.
		#[derive(Debug, Default)]
		pub struct EventContextResult {
			pub end: Option<String>,
			pub events_after: Vec<RawPdu>,
			pub events_before: Vec<RawPdu>,
			pub profile_info: BTreeMap<OwnedUserId, UserProfile>,
			pub start: Option<String>,
		}

		impl_codec_struct!(EventContextResult {} default {
			end: Option<String>,
			events_after: Vec<RawPdu>,
			events_before: Vec<RawPdu>,
			profile_info: BTreeMap<OwnedUserId, UserProfile>,
			start: Option<String>,
		});

		#[derive(Debug, Default)]
		pub struct SearchResult {
			pub context: EventContextResult,
			pub rank: Option<f64>,
			pub result: Option<RawPdu>,
		}

		impl_codec_struct!(SearchResult {} default {
			context: EventContextResult,
			rank: Option<f64>,
			result: Option<RawPdu>,
		});

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

		impl_codec_struct!(ResultRoomEvents {} default {
			count: Option<UInt>,
			groups: BTreeMap<String, Value>,
			highlights: Vec<String>,
			next_batch: Option<String>,
			results: Vec<SearchResult>,
			state: BTreeMap<OwnedRoomId, Vec<Raw<crate::events::AnyStateEvent>>>,
		});

		#[derive(Debug, Default)]
		pub struct ResultCategories {
			pub room_events: ResultRoomEvents,
		}

		impl_codec_struct!(ResultCategories {} default { room_events: ResultRoomEvents });

		crate::endpoint! {
			method: "POST", path: "/_matrix/client/v3/search",
			request {
				path {}
				query { next_batch: Option<String> }
				body { search_categories: Categories }
			}
			response { search_categories: ResultCategories }
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

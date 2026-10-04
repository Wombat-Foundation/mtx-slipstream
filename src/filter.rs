//! Sync and event filters.

use alloc::{string::String, vec::Vec};

use crate::{
	OwnedRoomId, OwnedUserId, UInt,
	codec::{DeError, Deserialize, Serialize},
	endpoint::{Input, object_from},
	impl_codec_enum, impl_codec_struct,
	json::Value,
};

/// Whether membership events are loaded lazily.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LazyLoadOptions {
	#[default]
	Disabled,
	Enabled {
		include_redundant_members: bool,
	},
}

impl LazyLoadOptions {
	#[must_use]
	pub fn is_disabled(&self) -> bool {
		matches!(self, Self::Disabled)
	}

	fn fields(self) -> (bool, bool) {
		match self {
			Self::Disabled => (false, false),
			Self::Enabled {
				include_redundant_members,
			} => (true, include_redundant_members),
		}
	}
}

/// Filters events by whether their content has a `url`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UrlFilter {
	EventsWithUrl,
	EventsWithoutUrl,
}

/// A filter on timeline, state or account-data events of a room.
#[derive(Clone, Debug, Default)]
pub struct RoomEventFilter {
	pub limit: Option<UInt>,
	pub types: Option<Vec<String>>,
	pub not_types: Vec<String>,
	pub rooms: Option<Vec<OwnedRoomId>>,
	pub not_rooms: Vec<OwnedRoomId>,
	pub senders: Option<Vec<OwnedUserId>>,
	pub not_senders: Vec<OwnedUserId>,
	pub url_filter: Option<UrlFilter>,
	pub lazy_load_options: LazyLoadOptions,
	pub unread_thread_notifications: bool,
}

impl Serialize for RoomEventFilter {
	fn to_json(&self) -> Value {
		let (lazy, redundant) = self.lazy_load_options.fields();
		let contains_url = self.url_filter.map(|f| f == UrlFilter::EventsWithUrl);
		Value::Object(object_from(alloc::vec![
			("limit", self.limit.to_json()),
			("types", self.types.to_json()),
			("not_types", self.not_types.to_json()),
			("rooms", self.rooms.to_json()),
			("not_rooms", self.not_rooms.to_json()),
			("senders", self.senders.to_json()),
			("not_senders", self.not_senders.to_json()),
			("contains_url", contains_url.to_json()),
			("lazy_load_members", lazy.to_json()),
			("include_redundant_members", redundant.to_json()),
			("unread_thread_notifications", self.unread_thread_notifications.to_json()),
		]))
	}
}

impl Deserialize for RoomEventFilter {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		let contains_url: Option<bool> = input.body("contains_url")?;
		let lazy: bool = input.body_or_default("lazy_load_members")?;
		Ok(Self {
			limit: input.body("limit")?,
			types: input.body("types")?,
			not_types: input.body_or_default("not_types")?,
			rooms: input.body("rooms")?,
			not_rooms: input.body_or_default("not_rooms")?,
			senders: input.body("senders")?,
			not_senders: input.body_or_default("not_senders")?,
			url_filter: contains_url.map(|with| {
				if with {
					UrlFilter::EventsWithUrl
				} else {
					UrlFilter::EventsWithoutUrl
				}
			}),
			lazy_load_options: if lazy {
				LazyLoadOptions::Enabled {
					include_redundant_members: input
						.body_or_default("include_redundant_members")?,
				}
			} else {
				LazyLoadOptions::Disabled
			},
			unread_thread_notifications: input.body_or_default("unread_thread_notifications")?,
		})
	}
}

/// A filter on non-room events such as presence.
#[derive(Clone, Debug, Default)]
pub struct Filter {
	pub limit: Option<UInt>,
	pub types: Option<Vec<String>>,
	pub not_types: Vec<String>,
	pub senders: Option<Vec<OwnedUserId>>,
	pub not_senders: Vec<OwnedUserId>,
}

impl_codec_struct!(Filter {} default {
	limit: Option<UInt>,
	types: Option<Vec<String>>,
	not_types: Vec<String>,
	senders: Option<Vec<OwnedUserId>>,
	not_senders: Vec<OwnedUserId>,
});

/// Filters applying to events in rooms.
#[derive(Clone, Debug, Default)]
pub struct RoomFilter {
	pub not_rooms: Vec<OwnedRoomId>,
	pub rooms: Option<Vec<OwnedRoomId>>,
	pub ephemeral: RoomEventFilter,
	pub include_leave: bool,
	pub state: RoomEventFilter,
	pub timeline: RoomEventFilter,
	pub account_data: RoomEventFilter,
}

impl_codec_struct!(RoomFilter {} default {
	not_rooms: Vec<OwnedRoomId>,
	rooms: Option<Vec<OwnedRoomId>>,
	ephemeral: RoomEventFilter,
	include_leave: bool,
	state: RoomEventFilter,
	timeline: RoomEventFilter,
	account_data: RoomEventFilter,
});

/// The format events are returned in.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EventFormat {
	#[default]
	Client,
	Federation,
}

impl_codec_enum!(EventFormat { Client => "client", Federation => "federation" });

/// A full sync filter.
#[derive(Clone, Debug, Default)]
pub struct FilterDefinition {
	pub event_fields: Option<Vec<String>>,
	pub event_format: EventFormat,
	pub presence: Filter,
	pub account_data: Filter,
	pub room: RoomFilter,
}

impl_codec_struct!(FilterDefinition {} default {
	event_fields: Option<Vec<String>>,
	event_format: EventFormat,
	presence: Filter,
	account_data: Filter,
	room: RoomFilter,
});

pub mod get_filter {
	pub mod v3 {
		use crate::{OwnedUserId, filter::FilterDefinition};

		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/user/{user_id}/filter/{filter_id}",
			request {
				path { user_id: OwnedUserId, filter_id: alloc::string::String }
				query {}
				body {}
			}
		}
		crate::endpoint_response_flat!(filter: FilterDefinition);

		impl Response {
			#[must_use]
			pub fn new(filter: FilterDefinition) -> Self {
				Self {
					filter,
				}
			}
		}
	}
}

pub mod create_filter {
	pub mod v3 {
		use crate::{OwnedUserId, filter::FilterDefinition};

		crate::endpoint_request_raw! {
			method: "POST", path: "/_matrix/client/v3/user/{user_id}/filter",
			request {
				path { user_id: OwnedUserId }
				query {}
				raw_body { filter: FilterDefinition }
			}
		}
		crate::endpoint_response! { response { filter_id: alloc::string::String } }

		impl Response {
			#[must_use]
			pub fn new(filter_id: alloc::string::String) -> Self {
				Self {
					filter_id,
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::from_str;

	#[test]
	fn lazy_load_and_url_filter_round_trip() {
		let filter: RoomEventFilter =
			from_str(r#"{"lazy_load_members":true,"contains_url":false,"limit":5}"#).unwrap();
		assert_eq!(
			filter.lazy_load_options,
			LazyLoadOptions::Enabled {
				include_redundant_members: false
			}
		);
		assert_eq!(filter.url_filter, Some(UrlFilter::EventsWithoutUrl));
		let back = RoomEventFilter::from_json(&filter.to_json()).unwrap();
		assert_eq!(back.limit, Some(5));
	}

	#[test]
	fn empty_definition_is_default() {
		let def: FilterDefinition = from_str("{}").unwrap();
		assert!(def.room.timeline.lazy_load_options.is_disabled());
		assert!(!def.room.include_leave);
	}
}

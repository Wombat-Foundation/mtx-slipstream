//! `/sync` (v3) and sliding sync (v4 requests, v5) request and response types.

use alloc::{string::String, vec::Vec};
use core::time::Duration;

use crate::{
	UInt,
	codec::{DeError, Deserialize, Serialize},
	js_option::JsOption,
	json::Value,
};

/// Whether a value counts as empty when deciding to omit a field.
pub trait SyncEmpty {
	fn sync_empty(&self) -> bool;
}

/// Writes `value` under `key` unless it is empty.
pub(crate) fn put_skip<T: Serialize + SyncEmpty>(
	object: &mut crate::json::Object,
	key: &str,
	value: &T,
) {
	if !value.sync_empty() {
		object.insert(key.into(), value.to_json());
	}
}

/// Writes `value` under `key` unconditionally.
pub(crate) fn put<T: Serialize>(object: &mut crate::json::Object, key: &str, value: &T) {
	object.insert(key.into(), value.to_json());
}

/// Reads `key`, or the type's default when it is absent.
pub(crate) fn get<T: Deserialize + Default>(
	object: &crate::json::Object,
	key: &str,
) -> Result<T, DeError> {
	match object.get(key) {
		Some(found) => T::from_json(found),
		None => Ok(T::default()),
	}
}

impl<T> SyncEmpty for Vec<T> {
	fn sync_empty(&self) -> bool {
		self.is_empty()
	}
}
impl<K, V> SyncEmpty for alloc::collections::BTreeMap<K, V> {
	fn sync_empty(&self) -> bool {
		self.is_empty()
	}
}
impl<T> SyncEmpty for Option<T> {
	fn sync_empty(&self) -> bool {
		self.is_none()
	}
}
impl<T> SyncEmpty for JsOption<T> {
	fn sync_empty(&self) -> bool {
		self.is_undefined()
	}
}
impl SyncEmpty for bool {
	fn sync_empty(&self) -> bool {
		!*self
	}
}
impl SyncEmpty for String {
	fn sync_empty(&self) -> bool {
		self.is_empty()
	}
}
impl SyncEmpty for crate::OwnedUserId {
	fn sync_empty(&self) -> bool {
		self.as_str().is_empty()
	}
}
impl SyncEmpty for u64 {
	fn sync_empty(&self) -> bool {
		*self == 0
	}
}

/// Declares a struct together with its Slipstream JSON codec.
impl Serialize for Duration {
	fn to_json(&self) -> Value {
		u64::try_from(self.as_millis()).unwrap_or(u64::MAX).to_json()
	}
}
impl Deserialize for Duration {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		u64::from_json(value).map(Self::from_millis)
	}
}

/// Unread counts for a room or thread.
#[derive(Clone, Debug, Default)]
pub struct UnreadNotificationsCount {
	pub highlight_count: Option<UInt>,
	pub notification_count: Option<UInt>,
}
impl crate::sync_events::SyncEmpty for UnreadNotificationsCount {
	fn sync_empty(&self) -> bool {
		crate::sync_events::SyncEmpty::sync_empty(&self.highlight_count)
			&& crate::sync_events::SyncEmpty::sync_empty(&self.notification_count)
	}
}
impl crate::codec::Serialize for UnreadNotificationsCount {
	fn to_json(&self) -> crate::json::Value {
		let mut object = crate::json::Object::new();
		if !crate::sync_events::SyncEmpty::sync_empty(&self.highlight_count) {
			object.insert(
				"highlight_count".into(),
				crate::codec::Serialize::to_json(&self.highlight_count),
			);
		}
		if !crate::sync_events::SyncEmpty::sync_empty(&self.notification_count) {
			object.insert(
				"notification_count".into(),
				crate::codec::Serialize::to_json(&self.notification_count),
			);
		}
		crate::json::Value::Object(object)
	}
}
impl crate::codec::Deserialize for UnreadNotificationsCount {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		let object =
			value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
		Ok(Self {
			highlight_count: crate::sync_events::get(object, "highlight_count")?,
			notification_count: crate::sync_events::get(object, "notification_count")?,
		})
	}
}
impl UnreadNotificationsCount {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
	#[must_use]
	pub fn is_empty(&self) -> bool {
		crate::sync_events::SyncEmpty::sync_empty(self)
	}
}

/// Users whose device lists changed or who left shared rooms.
#[derive(Clone, Debug, Default)]
pub struct DeviceLists {
	pub changed: Vec<crate::OwnedUserId>,
	pub left: Vec<crate::OwnedUserId>,
}
impl crate::sync_events::SyncEmpty for DeviceLists {
	fn sync_empty(&self) -> bool {
		crate::sync_events::SyncEmpty::sync_empty(&self.changed)
			&& crate::sync_events::SyncEmpty::sync_empty(&self.left)
	}
}
impl crate::codec::Serialize for DeviceLists {
	fn to_json(&self) -> crate::json::Value {
		let mut object = crate::json::Object::new();
		crate::sync_events::put_skip(&mut object, "changed", &self.changed);
		crate::sync_events::put_skip(&mut object, "left", &self.left);
		crate::json::Value::Object(object)
	}
}
impl crate::codec::Deserialize for DeviceLists {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		let object =
			value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
		Ok(Self {
			changed: crate::sync_events::get(object, "changed")?,
			left: crate::sync_events::get(object, "left")?,
		})
	}
}
impl DeviceLists {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
	#[must_use]
	pub fn is_empty(&self) -> bool {
		crate::sync_events::SyncEmpty::sync_empty(self)
	}
}

/// Sticky list filters for simplified sliding sync, kept alongside a
/// connection's other sticky parameters. `is_invited` is accepted as an alias
/// of `is_invite`.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct CompatListFilters {
	pub is_dm: Option<bool>,
	pub is_encrypted: Option<bool>,
	pub is_invite: Option<bool>,
	pub room_types: Vec<crate::directory::RoomTypeFilter>,
	pub not_room_types: Vec<crate::directory::RoomTypeFilter>,
	pub tags: Vec<String>,
	pub not_tags: Vec<String>,
	pub spaces: Vec<crate::OwnedRoomId>,
}

impl Serialize for CompatListFilters {
	fn to_json(&self) -> Value {
		let mut object = crate::json::Object::new();
		crate::sync_events::put_skip(&mut object, "is_dm", &self.is_dm);
		if !crate::sync_events::SyncEmpty::sync_empty(&self.is_encrypted) {
			object.insert(
				"is_encrypted".into(),
				crate::codec::Serialize::to_json(&self.is_encrypted),
			);
		}
		crate::sync_events::put_skip(&mut object, "is_invite", &self.is_invite);
		if !crate::sync_events::SyncEmpty::sync_empty(&self.room_types) {
			object
				.insert("room_types".into(), crate::codec::Serialize::to_json(&self.room_types));
		}
		if !crate::sync_events::SyncEmpty::sync_empty(&self.not_room_types) {
			object.insert(
				"not_room_types".into(),
				crate::codec::Serialize::to_json(&self.not_room_types),
			);
		}
		crate::sync_events::put_skip(&mut object, "tags", &self.tags);
		crate::sync_events::put_skip(&mut object, "not_tags", &self.not_tags);
		crate::sync_events::put_skip(&mut object, "spaces", &self.spaces);
		Value::Object(object)
	}
}

impl Deserialize for CompatListFilters {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("filters object"))?;
		let invite = object.get("is_invite").or_else(|| object.get("is_invited"));
		Ok(Self {
			is_dm: crate::sync_events::get(object, "is_dm")?,
			is_encrypted: crate::sync_events::get(object, "is_encrypted")?,
			is_invite: match invite {
				Some(found) => Deserialize::from_json(found)?,
				None => None,
			},
			room_types: crate::sync_events::get(object, "room_types")?,
			not_room_types: crate::sync_events::get(object, "not_room_types")?,
			tags: crate::sync_events::get(object, "tags")?,
			not_tags: crate::sync_events::get(object, "not_tags")?,
			spaces: crate::sync_events::get(object, "spaces")?,
		})
	}
}

pub mod v3 {
	use alloc::{collections::BTreeMap, string::String, vec::Vec};
	use core::time::Duration;

	use super::{DeviceLists, UnreadNotificationsCount};
	use crate::{
		OwnedEventId, OwnedRoomId, OwnedUserId, UInt,
		codec::{DeError, Deserialize, Serialize},
		endpoint::{EndpointRequest, EndpointResponse, Input, Metadata},
		events::{
			AnyGlobalAccountDataEvent, AnyRoomAccountDataEvent, AnyStrippedStateEvent,
			AnySyncEphemeralRoomEvent, AnySyncStateEvent, AnySyncTimelineEvent, AnyToDeviceEvent,
			presence::{PresenceEvent, PresenceState},
		},
		filter::FilterDefinition,
		json::Value,
		key_id::OneTimeKeyAlgorithm,
		sswire::Raw,
	};

	/// A filter given inline or by ID.
	#[derive(Debug)]
	pub enum Filter {
		FilterDefinition(alloc::boxed::Box<FilterDefinition>),
		FilterId(String),
	}

	impl From<FilterDefinition> for Filter {
		fn from(definition: FilterDefinition) -> Self {
			Self::FilterDefinition(alloc::boxed::Box::new(definition))
		}
	}
	impl From<String> for Filter {
		fn from(id: String) -> Self {
			Self::FilterId(id)
		}
	}

	impl Serialize for Filter {
		fn to_json(&self) -> Value {
			match self {
				Self::FilterId(id) => Value::String(id.clone()),
				Self::FilterDefinition(definition) => {
					Value::String(crate::codec::to_string(&**definition))
				}
			}
		}
	}
	impl Deserialize for Filter {
		fn from_json(value: &Value) -> Result<Self, DeError> {
			let text = value.as_str().ok_or_else(|| DeError::expected("filter string"))?;
			if text.starts_with('{') {
				crate::codec::from_str(text)
					.map(|definition| Self::FilterDefinition(alloc::boxed::Box::new(definition)))
			} else {
				Ok(Self::FilterId(text.into()))
			}
		}
	}

	#[derive(Debug)]
	pub struct Request {
		pub filter: Option<Filter>,
		pub since: Option<String>,
		pub full_state: bool,
		pub set_presence: PresenceState,
		pub timeout: Option<Duration>,
	}

	impl Default for Request {
		fn default() -> Self {
			Self {
				filter: None,
				since: None,
				full_state: false,
				set_presence: PresenceState::Online,
				timeout: None,
			}
		}
	}

	impl Request {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
	}

	const _: crate::endpoint::Metadata =
		crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/sync");
	impl EndpointRequest for Request {
		type Response = Response;

		const METADATA: Metadata = Metadata::new("GET", "/_matrix/client/v3/sync");

		fn path_args(&self) -> Vec<String> {
			Vec::new()
		}

		fn query(&self) -> Vec<(String, String)> {
			let set_presence =
				(self.set_presence != PresenceState::Online).then(|| self.set_presence.to_json());
			crate::endpoint::query_pairs(alloc::vec![
				("filter", self.filter.to_json()),
				("since", self.since.to_json()),
				("full_state", self.full_state.then_some(true).to_json()),
				("set_presence", set_presence.unwrap_or_default()),
				("timeout", self.timeout.to_json()),
			])
		}

		fn body(&self) -> Option<Value> {
			None
		}

		fn from_parts(
			path: &[String],
			query: &[(String, String)],
			body: Option<&Value>,
		) -> Result<Self, DeError> {
			let input = Input::new(path, query, body);
			let request = Self {
				filter: input.query("filter")?,
				since: input.query("since")?,
				full_state: input.query("full_state")?,
				set_presence: input
					.query::<Option<PresenceState>>("set_presence")?
					.unwrap_or(PresenceState::Online),
				timeout: input.query("timeout")?,
			};
			input.finish()?;
			Ok(request)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Rooms {
		pub leave: BTreeMap<OwnedRoomId, LeftRoom>,
		pub join: BTreeMap<OwnedRoomId, JoinedRoom>,
		pub invite: BTreeMap<OwnedRoomId, InvitedRoom>,
		pub knock: BTreeMap<OwnedRoomId, KnockedRoom>,
	}
	impl crate::sync_events::SyncEmpty for Rooms {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.leave)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.join)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.invite)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.knock)
		}
	}
	impl crate::codec::Serialize for Rooms {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "leave", &self.leave);
			crate::sync_events::put_skip(&mut object, "join", &self.join);
			crate::sync_events::put_skip(&mut object, "invite", &self.invite);
			crate::sync_events::put_skip(&mut object, "knock", &self.knock);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Rooms {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				leave: crate::sync_events::get(object, "leave")?,
				join: crate::sync_events::get(object, "join")?,
				invite: crate::sync_events::get(object, "invite")?,
				knock: crate::sync_events::get(object, "knock")?,
			})
		}
	}
	impl Rooms {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct LeftRoom {
		pub timeline: Timeline,
		pub state: State,
		pub account_data: RoomAccountData,
	}
	impl crate::sync_events::SyncEmpty for LeftRoom {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.timeline)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.state)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.account_data)
		}
	}
	impl crate::codec::Serialize for LeftRoom {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put(&mut object, "timeline", &self.timeline);
			crate::sync_events::put_skip(&mut object, "state", &self.state);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.account_data) {
				object.insert(
					"account_data".into(),
					crate::codec::Serialize::to_json(&self.account_data),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for LeftRoom {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				timeline: crate::sync_events::get(object, "timeline")?,
				state: crate::sync_events::get(object, "state")?,
				account_data: crate::sync_events::get(object, "account_data")?,
			})
		}
	}
	impl LeftRoom {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct JoinedRoom {
		pub summary: RoomSummary,
		pub unread_notifications: UnreadNotificationsCount,
		pub unread_thread_notifications: BTreeMap<OwnedEventId, UnreadNotificationsCount>,
		pub timeline: Timeline,
		pub state: State,
		pub account_data: RoomAccountData,
		pub ephemeral: Ephemeral,
	}
	impl crate::sync_events::SyncEmpty for JoinedRoom {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.summary)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.unread_notifications)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.unread_thread_notifications)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.state)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.account_data)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.ephemeral)
		}
	}
	impl crate::codec::Serialize for JoinedRoom {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "summary", &self.summary);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.unread_notifications) {
				object.insert(
					"unread_notifications".into(),
					crate::codec::Serialize::to_json(&self.unread_notifications),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.unread_thread_notifications) {
				object.insert(
					"unread_thread_notifications".into(),
					crate::codec::Serialize::to_json(&self.unread_thread_notifications),
				);
			}
			crate::sync_events::put(&mut object, "timeline", &self.timeline);
			crate::sync_events::put_skip(&mut object, "state", &self.state);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.account_data) {
				object.insert(
					"account_data".into(),
					crate::codec::Serialize::to_json(&self.account_data),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.ephemeral) {
				object.insert(
					"ephemeral".into(),
					crate::codec::Serialize::to_json(&self.ephemeral),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for JoinedRoom {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				summary: crate::sync_events::get(object, "summary")?,
				unread_notifications: crate::sync_events::get(object, "unread_notifications")?,
				unread_thread_notifications: crate::sync_events::get(
					object,
					"unread_thread_notifications",
				)?,
				timeline: crate::sync_events::get(object, "timeline")?,
				state: crate::sync_events::get(object, "state")?,
				account_data: crate::sync_events::get(object, "account_data")?,
				ephemeral: crate::sync_events::get(object, "ephemeral")?,
			})
		}
	}
	impl JoinedRoom {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct KnockedRoom {
		pub knock_state: KnockState,
	}
	impl crate::sync_events::SyncEmpty for KnockedRoom {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.knock_state)
		}
	}
	impl crate::codec::Serialize for KnockedRoom {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.knock_state) {
				object.insert(
					"knock_state".into(),
					crate::codec::Serialize::to_json(&self.knock_state),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for KnockedRoom {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				knock_state: crate::sync_events::get(object, "knock_state")?,
			})
		}
	}
	impl KnockedRoom {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	impl From<KnockState> for KnockedRoom {
		fn from(knock_state: KnockState) -> Self {
			Self {
				knock_state,
			}
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct KnockState {
		pub events: Vec<Raw<AnyStrippedStateEvent>>,
	}
	impl crate::sync_events::SyncEmpty for KnockState {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for KnockState {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for KnockState {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl KnockState {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Timeline {
		pub limited: bool,
		pub prev_batch: Option<String>,
		pub events: Vec<Raw<AnySyncTimelineEvent>>,
	}
	impl crate::sync_events::SyncEmpty for Timeline {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.limited)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.prev_batch)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for Timeline {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "limited", &self.limited);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.prev_batch) {
				object.insert(
					"prev_batch".into(),
					crate::codec::Serialize::to_json(&self.prev_batch),
				);
			}
			crate::sync_events::put(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Timeline {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				limited: crate::sync_events::get(object, "limited")?,
				prev_batch: crate::sync_events::get(object, "prev_batch")?,
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl Timeline {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct State {
		pub events: Vec<Raw<AnySyncStateEvent>>,
	}
	impl crate::sync_events::SyncEmpty for State {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for State {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for State {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl State {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	impl State {
		#[must_use]
		pub fn with_events(events: Vec<Raw<AnySyncStateEvent>>) -> Self {
			Self {
				events,
			}
		}
	}

	impl From<Vec<Raw<AnySyncStateEvent>>> for State {
		fn from(events: Vec<Raw<AnySyncStateEvent>>) -> Self {
			Self::with_events(events)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct GlobalAccountData {
		pub events: Vec<Raw<AnyGlobalAccountDataEvent>>,
	}
	impl crate::sync_events::SyncEmpty for GlobalAccountData {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for GlobalAccountData {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for GlobalAccountData {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl GlobalAccountData {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct RoomAccountData {
		pub events: Vec<Raw<AnyRoomAccountDataEvent>>,
	}
	impl crate::sync_events::SyncEmpty for RoomAccountData {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for RoomAccountData {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for RoomAccountData {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl RoomAccountData {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Ephemeral {
		pub events: Vec<Raw<AnySyncEphemeralRoomEvent>>,
	}
	impl crate::sync_events::SyncEmpty for Ephemeral {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for Ephemeral {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Ephemeral {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl Ephemeral {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct RoomSummary {
		pub heroes: Vec<OwnedUserId>,
		pub joined_member_count: Option<UInt>,
		pub invited_member_count: Option<UInt>,
	}
	impl crate::sync_events::SyncEmpty for RoomSummary {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.heroes)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.joined_member_count)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.invited_member_count)
		}
	}
	impl crate::codec::Serialize for RoomSummary {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "m.heroes", &self.heroes);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.joined_member_count) {
				object.insert(
					"m.joined_member_count".into(),
					crate::codec::Serialize::to_json(&self.joined_member_count),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.invited_member_count) {
				object.insert(
					"m.invited_member_count".into(),
					crate::codec::Serialize::to_json(&self.invited_member_count),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for RoomSummary {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				heroes: crate::sync_events::get(object, "m.heroes")?,
				joined_member_count: crate::sync_events::get(object, "m.joined_member_count")?,
				invited_member_count: crate::sync_events::get(object, "m.invited_member_count")?,
			})
		}
	}
	impl RoomSummary {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct InvitedRoom {
		pub invite_state: InviteState,
	}
	impl crate::sync_events::SyncEmpty for InvitedRoom {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.invite_state)
		}
	}
	impl crate::codec::Serialize for InvitedRoom {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.invite_state) {
				object.insert(
					"invite_state".into(),
					crate::codec::Serialize::to_json(&self.invite_state),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for InvitedRoom {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				invite_state: crate::sync_events::get(object, "invite_state")?,
			})
		}
	}
	impl InvitedRoom {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	impl From<InviteState> for InvitedRoom {
		fn from(invite_state: InviteState) -> Self {
			Self {
				invite_state,
			}
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct InviteState {
		pub events: Vec<Raw<AnyStrippedStateEvent>>,
	}
	impl crate::sync_events::SyncEmpty for InviteState {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for InviteState {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for InviteState {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl InviteState {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	impl From<Vec<Raw<AnyStrippedStateEvent>>> for InviteState {
		fn from(events: Vec<Raw<AnyStrippedStateEvent>>) -> Self {
			Self {
				events,
			}
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Presence {
		pub events: Vec<Raw<PresenceEvent>>,
	}
	impl crate::sync_events::SyncEmpty for Presence {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for Presence {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Presence {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl Presence {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct ToDevice {
		pub events: Vec<Raw<AnyToDeviceEvent>>,
	}
	impl crate::sync_events::SyncEmpty for ToDevice {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.events)
		}
	}
	impl crate::codec::Serialize for ToDevice {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "events", &self.events);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for ToDevice {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				events: crate::sync_events::get(object, "events")?,
			})
		}
	}
	impl ToDevice {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Response {
		pub next_batch: String,
		pub rooms: Rooms,
		pub presence: Presence,
		pub account_data: GlobalAccountData,
		pub to_device: ToDevice,
		pub device_lists: DeviceLists,
		pub device_one_time_keys_count: BTreeMap<OneTimeKeyAlgorithm, UInt>,
		pub device_unused_fallback_key_types: Option<Vec<OneTimeKeyAlgorithm>>,
	}
	impl crate::sync_events::SyncEmpty for Response {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.next_batch)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.presence)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.account_data)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.to_device)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.device_lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.device_one_time_keys_count)
				&& crate::sync_events::SyncEmpty::sync_empty(
					&self.device_unused_fallback_key_types,
				)
		}
	}
	impl crate::codec::Serialize for Response {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			object
				.insert("next_batch".into(), crate::codec::Serialize::to_json(&self.next_batch));
			crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.presence) {
				object
					.insert("presence".into(), crate::codec::Serialize::to_json(&self.presence));
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.account_data) {
				object.insert(
					"account_data".into(),
					crate::codec::Serialize::to_json(&self.account_data),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.to_device) {
				object.insert(
					"to_device".into(),
					crate::codec::Serialize::to_json(&self.to_device),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.device_lists) {
				object.insert(
					"device_lists".into(),
					crate::codec::Serialize::to_json(&self.device_lists),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.device_one_time_keys_count) {
				object.insert(
					"device_one_time_keys_count".into(),
					crate::codec::Serialize::to_json(&self.device_one_time_keys_count),
				);
			}
			object.insert(
				"device_unused_fallback_key_types".into(),
				crate::codec::Serialize::to_json(&self.device_unused_fallback_key_types),
			);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Response {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				next_batch: crate::sync_events::get(object, "next_batch")?,
				rooms: crate::sync_events::get(object, "rooms")?,
				presence: crate::sync_events::get(object, "presence")?,
				account_data: crate::sync_events::get(object, "account_data")?,
				to_device: crate::sync_events::get(object, "to_device")?,
				device_lists: crate::sync_events::get(object, "device_lists")?,
				device_one_time_keys_count: crate::sync_events::get(
					object,
					"device_one_time_keys_count",
				)?,
				device_unused_fallback_key_types: crate::sync_events::get(
					object,
					"device_unused_fallback_key_types",
				)?,
			})
		}
	}

	impl Response {
		#[must_use]
		pub fn new(next_batch: String) -> Self {
			Self {
				next_batch,
				..Self::default()
			}
		}
	}

	impl EndpointResponse for Response {
		fn to_body(&self) -> Value {
			self.to_json()
		}

		fn from_body(body: &Value) -> Result<Self, DeError> {
			Self::from_json(body)
		}
	}
}

pub mod v5 {
	use alloc::{collections::BTreeMap, string::String, vec::Vec};
	use core::time::Duration;

	use super::{DeviceLists, UnreadNotificationsCount};
	use crate::{
		OwnedRoomId,
		codec::{DeError, Deserialize, Serialize},
		endpoint::{EndpointRequest, EndpointResponse, Input, Metadata},
		json::{Object, Value},
	};

	pub mod request {
		use alloc::{string::String, vec::Vec};

		use crate::{
			OwnedRoomId, UInt,
			codec::{DeError, Deserialize, Serialize},
			directory::RoomTypeFilter,
			events::StateEventType,
			json::Value,
		};

		#[derive(Clone, Debug, Default)]
		pub struct List {
			pub ranges: Vec<(UInt, UInt)>,
			pub room_details: RoomDetails,
			pub include_heroes: Option<bool>,
			pub filters: Option<ListFilters>,
		}
		impl crate::sync_events::SyncEmpty for List {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.ranges)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.room_details)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.filters)
			}
		}
		impl crate::codec::Serialize for List {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				crate::sync_events::put(&mut object, "ranges", &self.ranges);
				if let crate::json::Value::Object(inner) =
					crate::codec::Serialize::to_json(&self.room_details)
				{
					object.extend(inner);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes) {
					object.insert(
						"include_heroes".into(),
						crate::codec::Serialize::to_json(&self.include_heroes),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.filters) {
					object.insert(
						"filters".into(),
						crate::codec::Serialize::to_json(&self.filters),
					);
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for List {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					ranges: crate::sync_events::get(object, "ranges")?,
					room_details: crate::codec::Deserialize::from_json(value)?,
					include_heroes: crate::sync_events::get(object, "include_heroes")?,
					filters: crate::sync_events::get(object, "filters")?,
				})
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct ListFilters {
			pub is_invite: Option<bool>,
			pub not_room_types: Vec<RoomTypeFilter>,
		}
		impl crate::sync_events::SyncEmpty for ListFilters {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.is_invite)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.not_room_types)
			}
		}
		impl crate::codec::Serialize for ListFilters {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.is_invite) {
					object.insert(
						"is_invite".into(),
						crate::codec::Serialize::to_json(&self.is_invite),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.not_room_types) {
					object.insert(
						"not_room_types".into(),
						crate::codec::Serialize::to_json(&self.not_room_types),
					);
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for ListFilters {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					is_invite: crate::sync_events::get(object, "is_invite")?,
					not_room_types: crate::sync_events::get(object, "not_room_types")?,
				})
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct RoomSubscription {
			pub required_state: Vec<(StateEventType, String)>,
			pub timeline_limit: UInt,
			pub include_heroes: Option<bool>,
		}
		impl crate::sync_events::SyncEmpty for RoomSubscription {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.required_state)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes)
			}
		}
		impl crate::codec::Serialize for RoomSubscription {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.required_state) {
					object.insert(
						"required_state".into(),
						crate::codec::Serialize::to_json(&self.required_state),
					);
				}
				object.insert(
					"timeline_limit".into(),
					crate::codec::Serialize::to_json(&self.timeline_limit),
				);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes) {
					object.insert(
						"include_heroes".into(),
						crate::codec::Serialize::to_json(&self.include_heroes),
					);
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for RoomSubscription {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					required_state: crate::sync_events::get(object, "required_state")?,
					timeline_limit: crate::sync_events::get(object, "timeline_limit")?,
					include_heroes: crate::sync_events::get(object, "include_heroes")?,
				})
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct RoomDetails {
			pub required_state: Vec<(StateEventType, String)>,
			pub timeline_limit: UInt,
		}
		impl crate::sync_events::SyncEmpty for RoomDetails {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.required_state)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit)
			}
		}
		impl crate::codec::Serialize for RoomDetails {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.required_state) {
					object.insert(
						"required_state".into(),
						crate::codec::Serialize::to_json(&self.required_state),
					);
				}
				object.insert(
					"timeline_limit".into(),
					crate::codec::Serialize::to_json(&self.timeline_limit),
				);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for RoomDetails {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					required_state: crate::sync_events::get(object, "required_state")?,
					timeline_limit: crate::sync_events::get(object, "timeline_limit")?,
				})
			}
		}

		#[derive(PartialEq, Clone, Debug, Default)]
		pub struct Extensions {
			pub to_device: ToDevice,
			pub e2ee: E2EE,
			pub account_data: AccountData,
			pub receipts: Receipts,
			pub typing: Typing,
		}
		impl crate::sync_events::SyncEmpty for Extensions {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.to_device)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.e2ee)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.account_data)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.receipts)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.typing)
			}
		}
		impl crate::codec::Serialize for Extensions {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.to_device) {
					object.insert(
						"to_device".into(),
						crate::codec::Serialize::to_json(&self.to_device),
					);
				}
				crate::sync_events::put_skip(&mut object, "e2ee", &self.e2ee);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.account_data) {
					object.insert(
						"account_data".into(),
						crate::codec::Serialize::to_json(&self.account_data),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.receipts) {
					object.insert(
						"receipts".into(),
						crate::codec::Serialize::to_json(&self.receipts),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.typing) {
					object
						.insert("typing".into(), crate::codec::Serialize::to_json(&self.typing));
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Extensions {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					to_device: crate::sync_events::get(object, "to_device")?,
					e2ee: crate::sync_events::get(object, "e2ee")?,
					account_data: crate::sync_events::get(object, "account_data")?,
					receipts: crate::sync_events::get(object, "receipts")?,
					typing: crate::sync_events::get(object, "typing")?,
				})
			}
		}
		impl Extensions {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(PartialEq, Clone, Debug, Default)]
		pub struct ToDevice {
			pub enabled: Option<bool>,
			pub limit: Option<UInt>,
			pub since: Option<String>,
			pub lists: Option<Vec<String>>,
			pub rooms: Option<Vec<OwnedRoomId>>,
		}
		impl crate::sync_events::SyncEmpty for ToDevice {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.limit)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.since)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for ToDevice {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.enabled) {
					object.insert(
						"enabled".into(),
						crate::codec::Serialize::to_json(&self.enabled),
					);
				}
				crate::sync_events::put_skip(&mut object, "limit", &self.limit);
				crate::sync_events::put_skip(&mut object, "since", &self.since);
				crate::sync_events::put_skip(&mut object, "lists", &self.lists);
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for ToDevice {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					enabled: crate::sync_events::get(object, "enabled")?,
					limit: crate::sync_events::get(object, "limit")?,
					since: crate::sync_events::get(object, "since")?,
					lists: crate::sync_events::get(object, "lists")?,
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl ToDevice {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(PartialEq, Clone, Debug, Default)]
		pub struct E2EE {
			pub enabled: Option<bool>,
		}
		impl crate::sync_events::SyncEmpty for E2EE {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
			}
		}
		impl crate::codec::Serialize for E2EE {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.enabled) {
					object.insert(
						"enabled".into(),
						crate::codec::Serialize::to_json(&self.enabled),
					);
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for E2EE {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					enabled: crate::sync_events::get(object, "enabled")?,
				})
			}
		}
		impl E2EE {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(PartialEq, Clone, Debug, Default)]
		pub struct AccountData {
			pub enabled: Option<bool>,
			pub lists: Option<Vec<String>>,
			pub rooms: Option<Vec<OwnedRoomId>>,
		}
		impl crate::sync_events::SyncEmpty for AccountData {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for AccountData {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.enabled) {
					object.insert(
						"enabled".into(),
						crate::codec::Serialize::to_json(&self.enabled),
					);
				}
				crate::sync_events::put_skip(&mut object, "lists", &self.lists);
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for AccountData {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					enabled: crate::sync_events::get(object, "enabled")?,
					lists: crate::sync_events::get(object, "lists")?,
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl AccountData {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(PartialEq, Clone, Debug, Default)]
		pub struct Receipts {
			pub enabled: Option<bool>,
			pub lists: Option<Vec<String>>,
			pub rooms: Option<Vec<ReceiptsRoom>>,
		}
		impl crate::sync_events::SyncEmpty for Receipts {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for Receipts {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.enabled) {
					object.insert(
						"enabled".into(),
						crate::codec::Serialize::to_json(&self.enabled),
					);
				}
				crate::sync_events::put_skip(&mut object, "lists", &self.lists);
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Receipts {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					enabled: crate::sync_events::get(object, "enabled")?,
					lists: crate::sync_events::get(object, "lists")?,
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl Receipts {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		/// Rooms a receipts extension covers: every subscribed room (`*`) or one room.
		#[derive(Clone, Debug, Eq, PartialEq)]
		pub enum ReceiptsRoom {
			AllSubscribed,
			Room(OwnedRoomId),
		}

		impl Serialize for ReceiptsRoom {
			fn to_json(&self) -> Value {
				match self {
					Self::AllSubscribed => Value::String("*".into()),
					Self::Room(room) => room.to_json(),
				}
			}
		}
		impl Deserialize for ReceiptsRoom {
			fn from_json(value: &Value) -> Result<Self, DeError> {
				match value.as_str() {
					Some("*") => Ok(Self::AllSubscribed),
					Some(other) => Ok(Self::Room(
						OwnedRoomId::parse(other).map_err(|_| DeError::expected("room ID"))?,
					)),
					None => Err(DeError::expected("room ID or `*`")),
				}
			}
		}

		#[derive(PartialEq, Clone, Debug, Default)]
		pub struct Typing {
			pub enabled: Option<bool>,
			pub lists: Option<Vec<String>>,
			pub rooms: Option<Vec<OwnedRoomId>>,
		}
		impl crate::sync_events::SyncEmpty for Typing {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for Typing {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.enabled) {
					object.insert(
						"enabled".into(),
						crate::codec::Serialize::to_json(&self.enabled),
					);
				}
				crate::sync_events::put_skip(&mut object, "lists", &self.lists);
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Typing {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					enabled: crate::sync_events::get(object, "enabled")?,
					lists: crate::sync_events::get(object, "lists")?,
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl Typing {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Request {
		pub pos: Option<String>,
		pub conn_id: Option<String>,
		pub txn_id: Option<String>,
		pub timeout: Option<Duration>,
		pub lists: BTreeMap<String, request::List>,
		pub room_subscriptions: BTreeMap<OwnedRoomId, request::RoomSubscription>,
		pub extensions: request::Extensions,
	}
	impl crate::sync_events::SyncEmpty for Request {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.pos)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.conn_id)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.txn_id)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.timeout)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.room_subscriptions)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.extensions)
		}
	}
	impl crate::codec::Serialize for Request {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "pos", &self.pos);
			crate::sync_events::put_skip(&mut object, "conn_id", &self.conn_id);
			crate::sync_events::put_skip(&mut object, "txn_id", &self.txn_id);
			crate::sync_events::put_skip(&mut object, "timeout", &self.timeout);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.room_subscriptions) {
				object.insert(
					"room_subscriptions".into(),
					crate::codec::Serialize::to_json(&self.room_subscriptions),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.extensions) {
				object.insert(
					"extensions".into(),
					crate::codec::Serialize::to_json(&self.extensions),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Request {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				pos: crate::sync_events::get(object, "pos")?,
				conn_id: crate::sync_events::get(object, "conn_id")?,
				txn_id: crate::sync_events::get(object, "txn_id")?,
				timeout: crate::sync_events::get(object, "timeout")?,
				lists: crate::sync_events::get(object, "lists")?,
				room_subscriptions: crate::sync_events::get(object, "room_subscriptions")?,
				extensions: crate::sync_events::get(object, "extensions")?,
			})
		}
	}
	impl Request {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	const _: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
		"POST",
		"/_matrix/client/unstable/org.matrix.simplified_msc3575/sync",
	);
	impl EndpointRequest for Request {
		type Response = Response;

		const METADATA: Metadata =
			Metadata::new("POST", "/_matrix/client/unstable/org.matrix.simplified_msc3575/sync");

		fn path_args(&self) -> Vec<String> {
			Vec::new()
		}

		fn query(&self) -> Vec<(String, String)> {
			crate::endpoint::query_pairs(alloc::vec![
				("pos", self.pos.to_json()),
				("timeout", self.timeout.to_json()),
			])
		}

		fn body(&self) -> Option<Value> {
			let mut body = self.to_json();
			if let Value::Object(object) = &mut body {
				object.remove("pos");
				object.remove("timeout");
			}
			Some(body)
		}

		fn from_parts(
			path: &[String],
			query: &[(String, String)],
			body: Option<&Value>,
		) -> Result<Self, DeError> {
			let input = Input::new(path, query, body);
			let empty = Value::Object(Object::new());
			let mut request = Self::from_json(body.unwrap_or(&empty))?;
			request.pos = input.query("pos")?;
			request.timeout = input.query("timeout")?;
			input.finish()?;
			Ok(request)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Response {
		pub txn_id: Option<String>,
		pub pos: String,
		pub lists: BTreeMap<String, response::List>,
		pub rooms: BTreeMap<OwnedRoomId, response::Room>,
		pub extensions: response::Extensions,
	}
	impl crate::sync_events::SyncEmpty for Response {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.txn_id)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.pos)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.extensions)
		}
	}
	impl crate::codec::Serialize for Response {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "txn_id", &self.txn_id);
			crate::sync_events::put(&mut object, "pos", &self.pos);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.extensions) {
				object.insert(
					"extensions".into(),
					crate::codec::Serialize::to_json(&self.extensions),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Response {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				txn_id: crate::sync_events::get(object, "txn_id")?,
				pos: crate::sync_events::get(object, "pos")?,
				lists: crate::sync_events::get(object, "lists")?,
				rooms: crate::sync_events::get(object, "rooms")?,
				extensions: crate::sync_events::get(object, "extensions")?,
			})
		}
	}

	impl Response {
		#[must_use]
		pub fn new(pos: String) -> Self {
			Self {
				pos,
				..Self::default()
			}
		}
	}

	impl EndpointResponse for Response {
		fn to_body(&self) -> Value {
			self.to_json()
		}

		fn from_body(body: &Value) -> Result<Self, DeError> {
			Self::from_json(body)
		}
	}

	pub mod response {
		use alloc::{collections::BTreeMap, string::String, vec::Vec};

		use super::{DeviceLists, UnreadNotificationsCount};
		pub use crate::events::{SyncReceiptEvent, SyncTypingEvent};
		use crate::{
			OwnedMxcUri, OwnedRoomId, OwnedUserId, UInt,
			events::{
				AnyGlobalAccountDataEvent, AnyRoomAccountDataEvent, AnyStrippedStateEvent,
				AnySyncStateEvent, AnySyncTimelineEvent, AnyToDeviceEvent,
			},
			js_option::JsOption,
			key_id::OneTimeKeyAlgorithm,
			sswire::Raw,
		};

		#[derive(Clone, Debug, Default)]
		pub struct List {
			pub count: UInt,
		}
		impl crate::sync_events::SyncEmpty for List {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.count)
			}
		}
		impl crate::codec::Serialize for List {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				crate::sync_events::put(&mut object, "count", &self.count);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for List {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					count: crate::sync_events::get(object, "count")?,
				})
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct Room {
			pub name: Option<String>,
			pub avatar: JsOption<OwnedMxcUri>,
			pub initial: Option<bool>,
			pub is_dm: Option<bool>,
			pub invite_state: Option<Vec<Raw<AnyStrippedStateEvent>>>,
			pub unread_notifications: UnreadNotificationsCount,
			pub timeline: Vec<Raw<AnySyncTimelineEvent>>,
			pub required_state: Vec<Raw<AnySyncStateEvent>>,
			pub prev_batch: Option<String>,
			pub limited: bool,
			pub joined_count: Option<UInt>,
			pub invited_count: Option<UInt>,
			pub num_live: Option<UInt>,
			pub bump_stamp: Option<UInt>,
			pub heroes: Option<Vec<Hero>>,
		}
		impl crate::sync_events::SyncEmpty for Room {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.name)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.avatar)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.initial)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.is_dm)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.invite_state)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.unread_notifications)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.required_state)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.prev_batch)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.limited)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.joined_count)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.invited_count)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.num_live)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.bump_stamp)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.heroes)
			}
		}
		impl crate::codec::Serialize for Room {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				crate::sync_events::put_skip(&mut object, "name", &self.name);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.avatar) {
					object
						.insert("avatar".into(), crate::codec::Serialize::to_json(&self.avatar));
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.initial) {
					object.insert(
						"initial".into(),
						crate::codec::Serialize::to_json(&self.initial),
					);
				}
				crate::sync_events::put_skip(&mut object, "is_dm", &self.is_dm);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.invite_state) {
					object.insert(
						"invite_state".into(),
						crate::codec::Serialize::to_json(&self.invite_state),
					);
				}
				if let crate::json::Value::Object(inner) =
					crate::codec::Serialize::to_json(&self.unread_notifications)
				{
					object.extend(inner);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.timeline) {
					object.insert(
						"timeline".into(),
						crate::codec::Serialize::to_json(&self.timeline),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.required_state) {
					object.insert(
						"required_state".into(),
						crate::codec::Serialize::to_json(&self.required_state),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.prev_batch) {
					object.insert(
						"prev_batch".into(),
						crate::codec::Serialize::to_json(&self.prev_batch),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.limited) {
					object.insert(
						"limited".into(),
						crate::codec::Serialize::to_json(&self.limited),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.joined_count) {
					object.insert(
						"joined_count".into(),
						crate::codec::Serialize::to_json(&self.joined_count),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.invited_count) {
					object.insert(
						"invited_count".into(),
						crate::codec::Serialize::to_json(&self.invited_count),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.num_live) {
					object.insert(
						"num_live".into(),
						crate::codec::Serialize::to_json(&self.num_live),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.bump_stamp) {
					object.insert(
						"bump_stamp".into(),
						crate::codec::Serialize::to_json(&self.bump_stamp),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.heroes) {
					object
						.insert("heroes".into(), crate::codec::Serialize::to_json(&self.heroes));
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Room {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					name: crate::sync_events::get(object, "name")?,
					avatar: crate::sync_events::get(object, "avatar")?,
					initial: crate::sync_events::get(object, "initial")?,
					is_dm: crate::sync_events::get(object, "is_dm")?,
					invite_state: crate::sync_events::get(object, "invite_state")?,
					unread_notifications: crate::codec::Deserialize::from_json(value)?,
					timeline: crate::sync_events::get(object, "timeline")?,
					required_state: crate::sync_events::get(object, "required_state")?,
					prev_batch: crate::sync_events::get(object, "prev_batch")?,
					limited: crate::sync_events::get(object, "limited")?,
					joined_count: crate::sync_events::get(object, "joined_count")?,
					invited_count: crate::sync_events::get(object, "invited_count")?,
					num_live: crate::sync_events::get(object, "num_live")?,
					bump_stamp: crate::sync_events::get(object, "bump_stamp")?,
					heroes: crate::sync_events::get(object, "heroes")?,
				})
			}
		}
		impl Room {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct Hero {
			pub user_id: OwnedUserId,
			pub name: Option<String>,
			pub avatar: Option<OwnedMxcUri>,
		}
		impl crate::sync_events::SyncEmpty for Hero {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.user_id)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.name)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.avatar)
			}
		}
		impl crate::codec::Serialize for Hero {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				crate::sync_events::put(&mut object, "user_id", &self.user_id);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.name) {
					object.insert(
						"displayname".into(),
						crate::codec::Serialize::to_json(&self.name),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.avatar) {
					object.insert(
						"avatar_url".into(),
						crate::codec::Serialize::to_json(&self.avatar),
					);
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Hero {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					user_id: crate::sync_events::get(object, "user_id")?,
					name: crate::sync_events::get(object, "displayname")?,
					avatar: crate::sync_events::get(object, "avatar_url")?,
				})
			}
		}

		impl Hero {
			#[must_use]
			pub fn new(user_id: OwnedUserId) -> Self {
				Self {
					user_id,
					name: None,
					avatar: None,
				}
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct Extensions {
			pub to_device: Option<ToDevice>,
			pub e2ee: E2EE,
			pub account_data: AccountData,
			pub receipts: Receipts,
			pub typing: Typing,
		}
		impl crate::sync_events::SyncEmpty for Extensions {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.to_device)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.e2ee)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.account_data)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.receipts)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.typing)
			}
		}
		impl crate::codec::Serialize for Extensions {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.to_device) {
					object.insert(
						"to_device".into(),
						crate::codec::Serialize::to_json(&self.to_device),
					);
				}
				crate::sync_events::put_skip(&mut object, "e2ee", &self.e2ee);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.account_data) {
					object.insert(
						"account_data".into(),
						crate::codec::Serialize::to_json(&self.account_data),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.receipts) {
					object.insert(
						"receipts".into(),
						crate::codec::Serialize::to_json(&self.receipts),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.typing) {
					object
						.insert("typing".into(), crate::codec::Serialize::to_json(&self.typing));
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Extensions {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					to_device: crate::sync_events::get(object, "to_device")?,
					e2ee: crate::sync_events::get(object, "e2ee")?,
					account_data: crate::sync_events::get(object, "account_data")?,
					receipts: crate::sync_events::get(object, "receipts")?,
					typing: crate::sync_events::get(object, "typing")?,
				})
			}
		}
		impl Extensions {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct ToDevice {
			pub next_batch: String,
			pub events: Vec<Raw<AnyToDeviceEvent>>,
		}
		impl crate::sync_events::SyncEmpty for ToDevice {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.next_batch)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.events)
			}
		}
		impl crate::codec::Serialize for ToDevice {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				object.insert(
					"next_batch".into(),
					crate::codec::Serialize::to_json(&self.next_batch),
				);
				if !crate::sync_events::SyncEmpty::sync_empty(&self.events) {
					object
						.insert("events".into(), crate::codec::Serialize::to_json(&self.events));
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for ToDevice {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					next_batch: crate::sync_events::get(object, "next_batch")?,
					events: crate::sync_events::get(object, "events")?,
				})
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct E2EE {
			pub device_lists: DeviceLists,
			pub device_one_time_keys_count: BTreeMap<OneTimeKeyAlgorithm, UInt>,
			pub device_unused_fallback_key_types: Option<Vec<OneTimeKeyAlgorithm>>,
		}
		impl crate::sync_events::SyncEmpty for E2EE {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.device_lists)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.device_one_time_keys_count)
					&& crate::sync_events::SyncEmpty::sync_empty(
						&self.device_unused_fallback_key_types,
					)
			}
		}
		impl crate::codec::Serialize for E2EE {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.device_lists) {
					object.insert(
						"device_lists".into(),
						crate::codec::Serialize::to_json(&self.device_lists),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(&self.device_one_time_keys_count) {
					object.insert(
						"device_one_time_keys_count".into(),
						crate::codec::Serialize::to_json(&self.device_one_time_keys_count),
					);
				}
				if !crate::sync_events::SyncEmpty::sync_empty(
					&self.device_unused_fallback_key_types,
				) {
					object.insert(
						"device_unused_fallback_key_types".into(),
						crate::codec::Serialize::to_json(&self.device_unused_fallback_key_types),
					);
				}
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for E2EE {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					device_lists: crate::sync_events::get(object, "device_lists")?,
					device_one_time_keys_count: crate::sync_events::get(
						object,
						"device_one_time_keys_count",
					)?,
					device_unused_fallback_key_types: crate::sync_events::get(
						object,
						"device_unused_fallback_key_types",
					)?,
				})
			}
		}
		impl E2EE {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct AccountData {
			pub global: Vec<Raw<AnyGlobalAccountDataEvent>>,
			pub rooms: BTreeMap<OwnedRoomId, Vec<Raw<AnyRoomAccountDataEvent>>>,
		}
		impl crate::sync_events::SyncEmpty for AccountData {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.global)
					&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for AccountData {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				if !crate::sync_events::SyncEmpty::sync_empty(&self.global) {
					object
						.insert("global".into(), crate::codec::Serialize::to_json(&self.global));
				}
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for AccountData {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					global: crate::sync_events::get(object, "global")?,
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl AccountData {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct Receipts {
			pub rooms: BTreeMap<OwnedRoomId, Raw<SyncReceiptEvent>>,
		}
		impl crate::sync_events::SyncEmpty for Receipts {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for Receipts {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Receipts {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl Receipts {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}

		#[derive(Clone, Debug, Default)]
		pub struct Typing {
			pub rooms: BTreeMap<OwnedRoomId, Raw<SyncTypingEvent>>,
		}
		impl crate::sync_events::SyncEmpty for Typing {
			fn sync_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
			}
		}
		impl crate::codec::Serialize for Typing {
			fn to_json(&self) -> crate::json::Value {
				let mut object = crate::json::Object::new();
				crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
				crate::json::Value::Object(object)
			}
		}
		impl crate::codec::Deserialize for Typing {
			fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let object =
					value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
				Ok(Self {
					rooms: crate::sync_events::get(object, "rooms")?,
				})
			}
		}
		impl Typing {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}
			#[must_use]
			pub fn is_empty(&self) -> bool {
				crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}
	}
}

pub mod v4 {
	use alloc::{collections::BTreeMap, string::String, vec::Vec};
	use core::time::Duration;

	use super::v5::request::ReceiptsRoom;
	use crate::{
		OwnedRoomId, UInt,
		codec::{DeError, Deserialize, Serialize},
		directory::RoomTypeFilter,
		endpoint::{EndpointRequest, EndpointResponse, Input, Metadata},
		events::{StateEventType, TimelineEventType},
		json::{Object, Value},
	};

	#[derive(Clone, Debug, Default)]
	pub struct SyncRequestListFilters {
		pub is_dm: Option<bool>,
		pub spaces: Vec<String>,
		pub is_encrypted: Option<bool>,
		pub is_invite: Option<bool>,
		pub is_tombstoned: Option<bool>,
		pub room_types: Vec<RoomTypeFilter>,
		pub not_room_types: Vec<RoomTypeFilter>,
		pub room_name_like: Option<String>,
		pub tags: Vec<String>,
		pub not_tags: Vec<String>,
	}
	impl crate::sync_events::SyncEmpty for SyncRequestListFilters {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.is_dm)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.spaces)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.is_encrypted)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.is_invite)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.is_tombstoned)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.room_types)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.not_room_types)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.room_name_like)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.tags)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.not_tags)
		}
	}
	impl crate::codec::Serialize for SyncRequestListFilters {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "is_dm", &self.is_dm);
			crate::sync_events::put_skip(&mut object, "spaces", &self.spaces);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.is_encrypted) {
				object.insert(
					"is_encrypted".into(),
					crate::codec::Serialize::to_json(&self.is_encrypted),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.is_invite) {
				object.insert(
					"is_invite".into(),
					crate::codec::Serialize::to_json(&self.is_invite),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.is_tombstoned) {
				object.insert(
					"is_tombstoned".into(),
					crate::codec::Serialize::to_json(&self.is_tombstoned),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.room_types) {
				object.insert(
					"room_types".into(),
					crate::codec::Serialize::to_json(&self.room_types),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.not_room_types) {
				object.insert(
					"not_room_types".into(),
					crate::codec::Serialize::to_json(&self.not_room_types),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.room_name_like) {
				object.insert(
					"room_name_like".into(),
					crate::codec::Serialize::to_json(&self.room_name_like),
				);
			}
			crate::sync_events::put_skip(&mut object, "tags", &self.tags);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.not_tags) {
				object
					.insert("not_tags".into(), crate::codec::Serialize::to_json(&self.not_tags));
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for SyncRequestListFilters {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				is_dm: crate::sync_events::get(object, "is_dm")?,
				spaces: crate::sync_events::get(object, "spaces")?,
				is_encrypted: crate::sync_events::get(object, "is_encrypted")?,
				is_invite: crate::sync_events::get(object, "is_invite")?,
				is_tombstoned: crate::sync_events::get(object, "is_tombstoned")?,
				room_types: crate::sync_events::get(object, "room_types")?,
				not_room_types: crate::sync_events::get(object, "not_room_types")?,
				room_name_like: crate::sync_events::get(object, "room_name_like")?,
				tags: crate::sync_events::get(object, "tags")?,
				not_tags: crate::sync_events::get(object, "not_tags")?,
			})
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct SyncRequestList {
		pub slow_get_all_rooms: bool,
		pub ranges: Vec<(UInt, UInt)>,
		pub sort: Vec<String>,
		pub room_details: RoomDetailsConfig,
		pub include_old_rooms: Option<IncludeOldRooms>,
		pub include_heroes: Option<bool>,
		pub filters: Option<SyncRequestListFilters>,
		pub bump_event_types: Vec<TimelineEventType>,
	}
	impl crate::sync_events::SyncEmpty for SyncRequestList {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.slow_get_all_rooms)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.ranges)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.sort)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.room_details)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.include_old_rooms)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.filters)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.bump_event_types)
		}
	}
	impl crate::codec::Serialize for SyncRequestList {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.slow_get_all_rooms) {
				object.insert(
					"slow_get_all_rooms".into(),
					crate::codec::Serialize::to_json(&self.slow_get_all_rooms),
				);
			}
			crate::sync_events::put(&mut object, "ranges", &self.ranges);
			crate::sync_events::put_skip(&mut object, "sort", &self.sort);
			if let crate::json::Value::Object(inner) =
				crate::codec::Serialize::to_json(&self.room_details)
			{
				object.extend(inner);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.include_old_rooms) {
				object.insert(
					"include_old_rooms".into(),
					crate::codec::Serialize::to_json(&self.include_old_rooms),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes) {
				object.insert(
					"include_heroes".into(),
					crate::codec::Serialize::to_json(&self.include_heroes),
				);
			}
			crate::sync_events::put_skip(&mut object, "filters", &self.filters);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.bump_event_types) {
				object.insert(
					"bump_event_types".into(),
					crate::codec::Serialize::to_json(&self.bump_event_types),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for SyncRequestList {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				slow_get_all_rooms: crate::sync_events::get(object, "slow_get_all_rooms")?,
				ranges: crate::sync_events::get(object, "ranges")?,
				sort: crate::sync_events::get(object, "sort")?,
				room_details: crate::codec::Deserialize::from_json(value)?,
				include_old_rooms: crate::sync_events::get(object, "include_old_rooms")?,
				include_heroes: crate::sync_events::get(object, "include_heroes")?,
				filters: crate::sync_events::get(object, "filters")?,
				bump_event_types: crate::sync_events::get(object, "bump_event_types")?,
			})
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct RoomDetailsConfig {
		pub required_state: Vec<(StateEventType, String)>,
		pub timeline_limit: Option<UInt>,
	}
	impl crate::sync_events::SyncEmpty for RoomDetailsConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.required_state)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit)
		}
	}
	impl crate::codec::Serialize for RoomDetailsConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.required_state) {
				object.insert(
					"required_state".into(),
					crate::codec::Serialize::to_json(&self.required_state),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit) {
				object.insert(
					"timeline_limit".into(),
					crate::codec::Serialize::to_json(&self.timeline_limit),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for RoomDetailsConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				required_state: crate::sync_events::get(object, "required_state")?,
				timeline_limit: crate::sync_events::get(object, "timeline_limit")?,
			})
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct IncludeOldRooms {
		pub required_state: Vec<(StateEventType, String)>,
		pub timeline_limit: Option<UInt>,
	}
	impl crate::sync_events::SyncEmpty for IncludeOldRooms {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.required_state)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit)
		}
	}
	impl crate::codec::Serialize for IncludeOldRooms {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.required_state) {
				object.insert(
					"required_state".into(),
					crate::codec::Serialize::to_json(&self.required_state),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit) {
				object.insert(
					"timeline_limit".into(),
					crate::codec::Serialize::to_json(&self.timeline_limit),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for IncludeOldRooms {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				required_state: crate::sync_events::get(object, "required_state")?,
				timeline_limit: crate::sync_events::get(object, "timeline_limit")?,
			})
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct RoomSubscription {
		pub required_state: Vec<(StateEventType, String)>,
		pub timeline_limit: Option<UInt>,
		pub include_heroes: Option<bool>,
	}
	impl crate::sync_events::SyncEmpty for RoomSubscription {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.required_state)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes)
		}
	}
	impl crate::codec::Serialize for RoomSubscription {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.required_state) {
				object.insert(
					"required_state".into(),
					crate::codec::Serialize::to_json(&self.required_state),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.timeline_limit) {
				object.insert(
					"timeline_limit".into(),
					crate::codec::Serialize::to_json(&self.timeline_limit),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.include_heroes) {
				object.insert(
					"include_heroes".into(),
					crate::codec::Serialize::to_json(&self.include_heroes),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for RoomSubscription {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				required_state: crate::sync_events::get(object, "required_state")?,
				timeline_limit: crate::sync_events::get(object, "timeline_limit")?,
				include_heroes: crate::sync_events::get(object, "include_heroes")?,
			})
		}
	}

	#[derive(PartialEq, Clone, Debug, Default)]
	pub struct ToDeviceConfig {
		pub enabled: Option<bool>,
		pub limit: Option<UInt>,
		pub since: Option<String>,
		pub lists: Option<Vec<String>>,
		pub rooms: Option<Vec<OwnedRoomId>>,
	}
	impl crate::sync_events::SyncEmpty for ToDeviceConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.limit)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.since)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
		}
	}
	impl crate::codec::Serialize for ToDeviceConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "enabled", &self.enabled);
			crate::sync_events::put_skip(&mut object, "limit", &self.limit);
			crate::sync_events::put_skip(&mut object, "since", &self.since);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for ToDeviceConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				enabled: crate::sync_events::get(object, "enabled")?,
				limit: crate::sync_events::get(object, "limit")?,
				since: crate::sync_events::get(object, "since")?,
				lists: crate::sync_events::get(object, "lists")?,
				rooms: crate::sync_events::get(object, "rooms")?,
			})
		}
	}

	#[derive(PartialEq, Clone, Debug, Default)]
	pub struct E2EEConfig {
		pub enabled: Option<bool>,
	}
	impl crate::sync_events::SyncEmpty for E2EEConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
		}
	}
	impl crate::codec::Serialize for E2EEConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "enabled", &self.enabled);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for E2EEConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				enabled: crate::sync_events::get(object, "enabled")?,
			})
		}
	}

	#[derive(PartialEq, Clone, Debug, Default)]
	pub struct AccountDataConfig {
		pub enabled: Option<bool>,
		pub lists: Option<Vec<String>>,
		pub rooms: Option<Vec<OwnedRoomId>>,
	}
	impl crate::sync_events::SyncEmpty for AccountDataConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
		}
	}
	impl crate::codec::Serialize for AccountDataConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "enabled", &self.enabled);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for AccountDataConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				enabled: crate::sync_events::get(object, "enabled")?,
				lists: crate::sync_events::get(object, "lists")?,
				rooms: crate::sync_events::get(object, "rooms")?,
			})
		}
	}

	#[derive(PartialEq, Clone, Debug, Default)]
	pub struct ReceiptsConfig {
		pub enabled: Option<bool>,
		pub lists: Option<Vec<String>>,
		pub rooms: Option<Vec<ReceiptsRoom>>,
	}
	impl crate::sync_events::SyncEmpty for ReceiptsConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
		}
	}
	impl crate::codec::Serialize for ReceiptsConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "enabled", &self.enabled);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for ReceiptsConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				enabled: crate::sync_events::get(object, "enabled")?,
				lists: crate::sync_events::get(object, "lists")?,
				rooms: crate::sync_events::get(object, "rooms")?,
			})
		}
	}

	#[derive(PartialEq, Clone, Debug, Default)]
	pub struct TypingConfig {
		pub enabled: Option<bool>,
		pub lists: Option<Vec<String>>,
		pub rooms: Option<Vec<OwnedRoomId>>,
	}
	impl crate::sync_events::SyncEmpty for TypingConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.enabled)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.rooms)
		}
	}
	impl crate::codec::Serialize for TypingConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "enabled", &self.enabled);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			crate::sync_events::put_skip(&mut object, "rooms", &self.rooms);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for TypingConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				enabled: crate::sync_events::get(object, "enabled")?,
				lists: crate::sync_events::get(object, "lists")?,
				rooms: crate::sync_events::get(object, "rooms")?,
			})
		}
	}

	#[derive(PartialEq, Clone, Debug, Default)]
	pub struct ExtensionsConfig {
		pub to_device: ToDeviceConfig,
		pub e2ee: E2EEConfig,
		pub account_data: AccountDataConfig,
		pub receipts: ReceiptsConfig,
		pub typing: TypingConfig,
	}
	impl crate::sync_events::SyncEmpty for ExtensionsConfig {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.to_device)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.e2ee)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.account_data)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.receipts)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.typing)
		}
	}
	impl crate::codec::Serialize for ExtensionsConfig {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			if !crate::sync_events::SyncEmpty::sync_empty(&self.to_device) {
				object.insert(
					"to_device".into(),
					crate::codec::Serialize::to_json(&self.to_device),
				);
			}
			crate::sync_events::put_skip(&mut object, "e2ee", &self.e2ee);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.account_data) {
				object.insert(
					"account_data".into(),
					crate::codec::Serialize::to_json(&self.account_data),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.receipts) {
				object
					.insert("receipts".into(), crate::codec::Serialize::to_json(&self.receipts));
			}
			crate::sync_events::put_skip(&mut object, "typing", &self.typing);
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for ExtensionsConfig {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				to_device: crate::sync_events::get(object, "to_device")?,
				e2ee: crate::sync_events::get(object, "e2ee")?,
				account_data: crate::sync_events::get(object, "account_data")?,
				receipts: crate::sync_events::get(object, "receipts")?,
				typing: crate::sync_events::get(object, "typing")?,
			})
		}
	}
	impl SyncRequestListFilters {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl SyncRequestList {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl RoomDetailsConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl IncludeOldRooms {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl RoomSubscription {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl ToDeviceConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl E2EEConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl AccountDataConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl ReceiptsConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl TypingConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}
	impl ExtensionsConfig {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Request {
		pub pos: Option<String>,
		pub delta_token: Option<String>,
		pub conn_id: Option<String>,
		pub txn_id: Option<String>,
		pub timeout: Option<Duration>,
		pub lists: BTreeMap<String, SyncRequestList>,
		pub room_subscriptions: BTreeMap<OwnedRoomId, RoomSubscription>,
		pub unsubscribe_rooms: Vec<OwnedRoomId>,
		pub extensions: ExtensionsConfig,
	}
	impl crate::sync_events::SyncEmpty for Request {
		fn sync_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(&self.pos)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.delta_token)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.conn_id)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.txn_id)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.timeout)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.lists)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.room_subscriptions)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.unsubscribe_rooms)
				&& crate::sync_events::SyncEmpty::sync_empty(&self.extensions)
		}
	}
	impl crate::codec::Serialize for Request {
		fn to_json(&self) -> crate::json::Value {
			let mut object = crate::json::Object::new();
			crate::sync_events::put_skip(&mut object, "pos", &self.pos);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.delta_token) {
				object.insert(
					"delta_token".into(),
					crate::codec::Serialize::to_json(&self.delta_token),
				);
			}
			crate::sync_events::put_skip(&mut object, "conn_id", &self.conn_id);
			crate::sync_events::put_skip(&mut object, "txn_id", &self.txn_id);
			crate::sync_events::put_skip(&mut object, "timeout", &self.timeout);
			crate::sync_events::put_skip(&mut object, "lists", &self.lists);
			if !crate::sync_events::SyncEmpty::sync_empty(&self.room_subscriptions) {
				object.insert(
					"room_subscriptions".into(),
					crate::codec::Serialize::to_json(&self.room_subscriptions),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.unsubscribe_rooms) {
				object.insert(
					"unsubscribe_rooms".into(),
					crate::codec::Serialize::to_json(&self.unsubscribe_rooms),
				);
			}
			if !crate::sync_events::SyncEmpty::sync_empty(&self.extensions) {
				object.insert(
					"extensions".into(),
					crate::codec::Serialize::to_json(&self.extensions),
				);
			}
			crate::json::Value::Object(object)
		}
	}
	impl crate::codec::Deserialize for Request {
		fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
			let object =
				value.as_object().ok_or_else(|| crate::codec::DeError::expected("object"))?;
			Ok(Self {
				pos: crate::sync_events::get(object, "pos")?,
				delta_token: crate::sync_events::get(object, "delta_token")?,
				conn_id: crate::sync_events::get(object, "conn_id")?,
				txn_id: crate::sync_events::get(object, "txn_id")?,
				timeout: crate::sync_events::get(object, "timeout")?,
				lists: crate::sync_events::get(object, "lists")?,
				room_subscriptions: crate::sync_events::get(object, "room_subscriptions")?,
				unsubscribe_rooms: crate::sync_events::get(object, "unsubscribe_rooms")?,
				extensions: crate::sync_events::get(object, "extensions")?,
			})
		}
	}
	impl Request {
		#[must_use]
		pub fn new() -> Self {
			Self::default()
		}
		#[must_use]
		pub fn is_empty(&self) -> bool {
			crate::sync_events::SyncEmpty::sync_empty(self)
		}
	}

	const _: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
		"POST",
		"/_matrix/client/unstable/org.matrix.msc3575/sync",
	);
	impl EndpointRequest for Request {
		type Response = Response;

		const METADATA: Metadata =
			Metadata::new("POST", "/_matrix/client/unstable/org.matrix.msc3575/sync");

		fn path_args(&self) -> Vec<String> {
			Vec::new()
		}

		fn query(&self) -> Vec<(String, String)> {
			crate::endpoint::query_pairs(alloc::vec![
				("pos", self.pos.to_json()),
				("timeout", self.timeout.to_json()),
			])
		}

		fn body(&self) -> Option<Value> {
			let mut body = self.to_json();
			if let Value::Object(object) = &mut body {
				object.remove("pos");
				object.remove("timeout");
			}
			Some(body)
		}

		fn from_parts(
			path: &[String],
			query: &[(String, String)],
			body: Option<&Value>,
		) -> Result<Self, DeError> {
			let input = Input::new(path, query, body);
			let empty = Value::Object(Object::new());
			let mut request = Self::from_json(body.unwrap_or(&empty))?;
			request.pos = input.query("pos")?;
			request.timeout = input.query("timeout")?;
			input.finish()?;
			Ok(request)
		}
	}

	/// Sliding-sync v4 responses are not produced by this server.
	#[derive(Debug, Default)]
	pub struct Response {
		pub pos: String,
	}

	impl EndpointResponse for Response {
		fn to_body(&self) -> Value {
			let mut object = Object::new();
			object.insert("pos".into(), Value::String(self.pos.clone()));
			Value::Object(object)
		}

		fn from_body(body: &Value) -> Result<Self, DeError> {
			Ok(Self {
				pos: body.get("pos").and_then(Value::as_str).unwrap_or_default().into(),
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::ToString;
	use core::time::Duration;

	use super::*;
	use crate::{
		codec::{from_str, to_string},
		endpoint::EndpointRequest,
		events::presence::PresenceState,
	};

	#[test]
	fn timeline_always_has_events_and_skips_default_limited() {
		let mut timeline = v3::Timeline::new();
		assert_eq!(to_string(&timeline), r#"{"events":[]}"#);
		timeline.limited = true;
		assert_eq!(to_string(&timeline), r#"{"events":[],"limited":true}"#);
		assert!(from_str::<v3::Timeline>(r#"{"events":[]}"#).unwrap().is_empty());
	}

	#[test]
	fn v3_response_omits_empty_sections() {
		let response = v3::Response::new("s1".to_string());
		let text = to_string(&response);
		assert_eq!(text, r#"{"device_unused_fallback_key_types":null,"next_batch":"s1"}"#);
		let mut joined = v3::JoinedRoom::new();
		joined.unread_notifications.notification_count = Some(3);
		assert!(!joined.is_empty());
		assert!(to_string(&joined).contains("\"notification_count\":3"));
	}

	#[test]
	fn v3_request_query_round_trips() {
		let request = v3::Request {
			filter: Some(v3::Filter::FilterId("f1".into())),
			since: Some("s72".into()),
			full_state: true,
			set_presence: PresenceState::Offline,
			timeout: Some(Duration::from_secs(30)),
		};
		let query = request.query();
		for expected in ["filter", "since", "full_state", "set_presence", "timeout"] {
			assert!(query.iter().any(|(key, _)| key == expected), "{expected}");
		}
		let parsed = v3::Request::from_parts(&[], &query, None).unwrap();
		assert_eq!(parsed.since.as_deref(), Some("s72"));
		assert!(parsed.full_state);
		assert_eq!(parsed.set_presence, PresenceState::Offline);
		assert_eq!(parsed.timeout, Some(Duration::from_secs(30)));

		let bare = v3::Request::from_parts(&[], &[], None).unwrap();
		assert_eq!(bare.set_presence, PresenceState::Online);
		assert!(!bare.full_state && bare.timeout.is_none() && bare.filter.is_none());
	}

	#[test]
	fn v5_request_flattens_room_details() {
		let body = from_str::<Value>(
			r#"{"conn_id":"c","lists":{"all":{"ranges":[[0,19]],"timeline_limit":5,"required_state":[["m.room.name",""]]}},"extensions":{"e2ee":{"enabled":true}}}"#,
		)
		.unwrap();
		let query =
			[("pos".to_string(), "7".to_string()), ("timeout".to_string(), "100".to_string())];
		let request = v5::Request::from_parts(&[], &query, Some(&body)).unwrap();
		assert_eq!(request.pos.as_deref(), Some("7"));
		assert_eq!(request.timeout, Some(Duration::from_millis(100)));
		let list = &request.lists["all"];
		assert_eq!(list.ranges, [(0, 19)]);
		assert_eq!(list.room_details.timeline_limit, 5);
		assert_eq!(list.room_details.required_state.len(), 1);
		assert_eq!(request.extensions.e2ee.enabled, Some(true));
		let sent = request.body().unwrap();
		assert!(sent.get("pos").is_none() && sent.get("timeout").is_none());
		assert!(sent.get("lists").is_some());
	}

	#[test]
	fn v5_room_distinguishes_null_avatar_from_absent() {
		let mut room = v5::response::Room::new();
		assert!(!to_string(&room).contains("avatar"));
		room.avatar = JsOption::Null;
		assert!(to_string(&room).contains("\"avatar\":null"));
		room.avatar = JsOption::Some(crate::OwnedMxcUri::parse("mxc://x/y").unwrap());
		room.unread_notifications.highlight_count = Some(1);
		let text = to_string(&room);
		assert!(
			text.contains("\"avatar\":\"mxc://x/y\"") && text.contains("\"highlight_count\":1")
		);
	}

	#[test]
	fn v4_extensions_config_defaults_and_compares() {
		let config = from_str::<v4::ExtensionsConfig>(r#"{"e2ee":{"enabled":true}}"#).unwrap();
		assert_eq!(config.e2ee.enabled, Some(true));
		assert!(config.account_data.is_empty() && !config.is_empty());
		assert_eq!(config, from_str::<v4::ExtensionsConfig>(&to_string(&config)).unwrap());
	}
}

#[cfg(test)]
mod compat_tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn compat_filters_accept_is_invited_alias_and_skip_empty() {
		let parsed =
			from_str::<CompatListFilters>(r#"{"is_invited":true,"tags":["a"]}"#).unwrap();
		assert_eq!(parsed.is_invite, Some(true));
		assert_eq!(to_string(&parsed), r#"{"is_invite":true,"tags":["a"]}"#);
		assert_eq!(to_string(&CompatListFilters::default()), "{}");
	}
}

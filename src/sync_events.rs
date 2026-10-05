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

macro_rules! sync_put {
	($o:ident, $v:expr, ($key:literal, skip)) => {
		if !$crate::sync_events::SyncEmpty::sync_empty($v) {
			$o.insert($key.into(), $crate::codec::Serialize::to_json($v));
		}
	};
	($o:ident, $v:expr, ($key:literal, keep)) => {
		$o.insert($key.into(), $crate::codec::Serialize::to_json($v));
	};
	($o:ident, $v:expr, (flatten)) => {
		if let $crate::json::Value::Object(inner) = $crate::codec::Serialize::to_json($v) {
			$o.extend(inner);
		}
	};
}

macro_rules! sync_get {
	($o:ident, $v:ident, ($key:literal, $mode:ident)) => {
		match $o.get($key) {
			Some(found) => $crate::codec::Deserialize::from_json(found)?,
			None => Default::default(),
		}
	};
	($o:ident, $v:ident, (flatten)) => {
		$crate::codec::Deserialize::from_json($v)?
	};
}

/// Declares a struct together with its JSON codec.
///
/// Each field names its key and whether it is omitted when empty (`skip`) or
/// always written (`keep`); `flatten` merges a nested struct into the parent.
/// Absent fields decode to their `Default`.
#[macro_export]
macro_rules! sync_struct {
	(
		$(#[$meta:meta])*
		pub struct $name:ident {
			$($(#[$fmeta:meta])* pub $field:ident : $ty:ty = $spec:tt),* $(,)?
		}
	) => {
		$(#[$meta])*
		#[derive(Clone, Debug, Default)]
		pub struct $name {
			$($(#[$fmeta])* pub $field: $ty),*
		}

		impl $crate::sync_events::SyncEmpty for $name {
			fn sync_empty(&self) -> bool {
				true $(&& $crate::sync_events::SyncEmpty::sync_empty(&self.$field))*
			}
		}

		impl $crate::codec::Serialize for $name {
			fn to_json(&self) -> $crate::json::Value {
				let mut object = $crate::json::Object::new();
				$($crate::sync_struct!(@put object, &self.$field, $spec);)*
				$crate::json::Value::Object(object)
			}
		}

		impl $crate::codec::Deserialize for $name {
			fn from_json(
				value: &$crate::json::Value,
			) -> Result<Self, $crate::codec::DeError> {
				let object = value
					.as_object()
					.ok_or_else(|| $crate::codec::DeError::expected("object"))?;
				Ok(Self {
					$($field: $crate::sync_struct!(@get object, value, $spec)),*
				})
			}
		}
	};
	(@put $object:ident, $value:expr, ($key:literal, skip)) => {
		if !$crate::sync_events::SyncEmpty::sync_empty($value) {
			$object.insert($key.into(), $crate::codec::Serialize::to_json($value));
		}
	};
	(@put $object:ident, $value:expr, ($key:literal, keep)) => {
		$object.insert($key.into(), $crate::codec::Serialize::to_json($value));
	};
	(@put $object:ident, $value:expr, (flatten)) => {
		if let $crate::json::Value::Object(inner) = $crate::codec::Serialize::to_json($value) {
			$object.extend(inner);
		}
	};
	(@get $object:ident, $value:ident, ($key:literal, $mode:ident)) => {
		match $object.get($key) {
			Some(found) => $crate::codec::Deserialize::from_json(found)?,
			None => Default::default(),
		}
	};
	(@get $object:ident, $value:ident, (flatten)) => {
		$crate::codec::Deserialize::from_json($value)?
	};
}

/// Declares a struct together with its Slipstream JSON codec.
pub use crate::sync_struct as codec_struct;

/// Adds `new` and `is_empty` to a struct made by `sync_struct!`.
macro_rules! sync_basics {
	($($name:ident),* $(,)?) => {$(
		impl $name {
			#[must_use]
			pub fn new() -> Self {
				Self::default()
			}

			#[must_use]
			pub fn is_empty(&self) -> bool {
				$crate::sync_events::SyncEmpty::sync_empty(self)
			}
		}
	)*};
}

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

sync_struct! {
	/// Unread counts for a room or thread.
	pub struct UnreadNotificationsCount {
		pub highlight_count: Option<UInt> = ("highlight_count", skip),
		pub notification_count: Option<UInt> = ("notification_count", skip),
	}
}
sync_basics!(UnreadNotificationsCount);

sync_struct! {
	/// Users whose device lists changed or who left shared rooms.
	pub struct DeviceLists {
		pub changed: Vec<crate::OwnedUserId> = ("changed", skip),
		pub left: Vec<crate::OwnedUserId> = ("left", skip),
	}
}
sync_basics!(DeviceLists);

/// Sticky list filters for simplified sliding sync, kept alongside a
/// connection's other sticky parameters. `is_invited` is accepted as an alias
/// of `is_invite`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
		sync_put!(object, &self.is_dm, ("is_dm", skip));
		sync_put!(object, &self.is_encrypted, ("is_encrypted", skip));
		sync_put!(object, &self.is_invite, ("is_invite", skip));
		sync_put!(object, &self.room_types, ("room_types", skip));
		sync_put!(object, &self.not_room_types, ("not_room_types", skip));
		sync_put!(object, &self.tags, ("tags", skip));
		sync_put!(object, &self.not_tags, ("not_tags", skip));
		sync_put!(object, &self.spaces, ("spaces", skip));
		Value::Object(object)
	}
}

impl Deserialize for CompatListFilters {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("filters object"))?;
		let invite = object.get("is_invite").or_else(|| object.get("is_invited"));
		Ok(Self {
			is_dm: sync_get!(object, value, ("is_dm", skip)),
			is_encrypted: sync_get!(object, value, ("is_encrypted", skip)),
			is_invite: match invite {
				Some(found) => Deserialize::from_json(found)?,
				None => None,
			},
			room_types: sync_get!(object, value, ("room_types", skip)),
			not_room_types: sync_get!(object, value, ("not_room_types", skip)),
			tags: sync_get!(object, value, ("tags", skip)),
			not_tags: sync_get!(object, value, ("not_tags", skip)),
			spaces: sync_get!(object, value, ("spaces", skip)),
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
		serde::Raw,
	};

	/// A filter given inline or by ID.
	#[derive(Clone, Debug)]
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

	#[derive(Clone, Debug)]
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

	sync_struct! {
		pub struct Rooms {
			pub leave: BTreeMap<OwnedRoomId, LeftRoom> = ("leave", skip),
			pub join: BTreeMap<OwnedRoomId, JoinedRoom> = ("join", skip),
			pub invite: BTreeMap<OwnedRoomId, InvitedRoom> = ("invite", skip),
			pub knock: BTreeMap<OwnedRoomId, KnockedRoom> = ("knock", skip),
		}
	}
	sync_basics!(Rooms);

	sync_struct! {
		pub struct LeftRoom {
			pub timeline: Timeline = ("timeline", keep),
			pub state: State = ("state", skip),
			pub account_data: RoomAccountData = ("account_data", skip),
		}
	}
	sync_basics!(LeftRoom);

	sync_struct! {
		pub struct JoinedRoom {
			pub summary: RoomSummary = ("summary", skip),
			pub unread_notifications: UnreadNotificationsCount = ("unread_notifications", skip),
			pub unread_thread_notifications:
				BTreeMap<OwnedEventId, UnreadNotificationsCount> =
				("unread_thread_notifications", skip),
			pub timeline: Timeline = ("timeline", keep),
			pub state: State = ("state", skip),
			pub account_data: RoomAccountData = ("account_data", skip),
			pub ephemeral: Ephemeral = ("ephemeral", skip),
		}
	}
	sync_basics!(JoinedRoom);

	sync_struct! {
		pub struct KnockedRoom {
			pub knock_state: KnockState = ("knock_state", skip),
		}
	}
	sync_basics!(KnockedRoom);

	impl From<KnockState> for KnockedRoom {
		fn from(knock_state: KnockState) -> Self {
			Self {
				knock_state,
			}
		}
	}

	sync_struct! {
		pub struct KnockState {
			pub events: Vec<Raw<AnyStrippedStateEvent>> = ("events", skip),
		}
	}
	sync_basics!(KnockState);

	sync_struct! {
		pub struct Timeline {
			pub limited: bool = ("limited", skip),
			pub prev_batch: Option<String> = ("prev_batch", skip),
			pub events: Vec<Raw<AnySyncTimelineEvent>> = ("events", keep),
		}
	}
	sync_basics!(Timeline);

	sync_struct! {
		pub struct State {
			pub events: Vec<Raw<AnySyncStateEvent>> = ("events", skip),
		}
	}
	sync_basics!(State);

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

	sync_struct! {
		pub struct GlobalAccountData {
			pub events: Vec<Raw<AnyGlobalAccountDataEvent>> = ("events", skip),
		}
	}
	sync_basics!(GlobalAccountData);

	sync_struct! {
		pub struct RoomAccountData {
			pub events: Vec<Raw<AnyRoomAccountDataEvent>> = ("events", skip),
		}
	}
	sync_basics!(RoomAccountData);

	sync_struct! {
		pub struct Ephemeral {
			pub events: Vec<Raw<AnySyncEphemeralRoomEvent>> = ("events", skip),
		}
	}
	sync_basics!(Ephemeral);

	sync_struct! {
		pub struct RoomSummary {
			pub heroes: Vec<OwnedUserId> = ("m.heroes", skip),
			pub joined_member_count: Option<UInt> = ("m.joined_member_count", skip),
			pub invited_member_count: Option<UInt> = ("m.invited_member_count", skip),
		}
	}
	sync_basics!(RoomSummary);

	sync_struct! {
		pub struct InvitedRoom {
			pub invite_state: InviteState = ("invite_state", skip),
		}
	}
	sync_basics!(InvitedRoom);

	impl From<InviteState> for InvitedRoom {
		fn from(invite_state: InviteState) -> Self {
			Self {
				invite_state,
			}
		}
	}

	sync_struct! {
		pub struct InviteState {
			pub events: Vec<Raw<AnyStrippedStateEvent>> = ("events", skip),
		}
	}
	sync_basics!(InviteState);

	impl From<Vec<Raw<AnyStrippedStateEvent>>> for InviteState {
		fn from(events: Vec<Raw<AnyStrippedStateEvent>>) -> Self {
			Self {
				events,
			}
		}
	}

	sync_struct! {
		pub struct Presence {
			pub events: Vec<Raw<PresenceEvent>> = ("events", skip),
		}
	}
	sync_basics!(Presence);

	sync_struct! {
		pub struct ToDevice {
			pub events: Vec<Raw<AnyToDeviceEvent>> = ("events", skip),
		}
	}
	sync_basics!(ToDevice);

	sync_struct! {
		pub struct Response {
			pub next_batch: String = ("next_batch", keep),
			pub rooms: Rooms = ("rooms", skip),
			pub presence: Presence = ("presence", skip),
			pub account_data: GlobalAccountData = ("account_data", skip),
			pub to_device: ToDevice = ("to_device", skip),
			pub device_lists: DeviceLists = ("device_lists", skip),
			pub device_one_time_keys_count:
				BTreeMap<OneTimeKeyAlgorithm, UInt> = ("device_one_time_keys_count", skip),
			pub device_unused_fallback_key_types:
				Option<Vec<OneTimeKeyAlgorithm>> = ("device_unused_fallback_key_types", keep),
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

		sync_struct! {
			pub struct List {
				pub ranges: Vec<(UInt, UInt)> = ("ranges", keep),
				pub room_details: RoomDetails = (flatten),
				pub include_heroes: Option<bool> = ("include_heroes", skip),
				pub filters: Option<ListFilters> = ("filters", skip),
			}
		}

		sync_struct! {
			pub struct ListFilters {
				pub is_invite: Option<bool> = ("is_invite", skip),
				pub not_room_types: Vec<RoomTypeFilter> = ("not_room_types", skip),
			}
		}

		sync_struct! {
			pub struct RoomSubscription {
				pub required_state: Vec<(StateEventType, String)> = ("required_state", skip),
				pub timeline_limit: UInt = ("timeline_limit", keep),
				pub include_heroes: Option<bool> = ("include_heroes", skip),
			}
		}

		sync_struct! {
			pub struct RoomDetails {
				pub required_state: Vec<(StateEventType, String)> = ("required_state", skip),
				pub timeline_limit: UInt = ("timeline_limit", keep),
			}
		}

		sync_struct! {
			#[derive(PartialEq)]
			pub struct Extensions {
				pub to_device: ToDevice = ("to_device", skip),
				pub e2ee: E2EE = ("e2ee", skip),
				pub account_data: AccountData = ("account_data", skip),
				pub receipts: Receipts = ("receipts", skip),
				pub typing: Typing = ("typing", skip),
			}
		}
		sync_basics!(Extensions);

		sync_struct! {
			#[derive(PartialEq)]
			pub struct ToDevice {
				pub enabled: Option<bool> = ("enabled", skip),
				pub limit: Option<UInt> = ("limit", skip),
				pub since: Option<String> = ("since", skip),
				pub lists: Option<Vec<String>> = ("lists", skip),
				pub rooms: Option<Vec<OwnedRoomId>> = ("rooms", skip),
			}
		}
		sync_basics!(ToDevice);

		sync_struct! {
			#[derive(PartialEq)]
			pub struct E2EE {
				pub enabled: Option<bool> = ("enabled", skip),
			}
		}
		sync_basics!(E2EE);

		sync_struct! {
			#[derive(PartialEq)]
			pub struct AccountData {
				pub enabled: Option<bool> = ("enabled", skip),
				pub lists: Option<Vec<String>> = ("lists", skip),
				pub rooms: Option<Vec<OwnedRoomId>> = ("rooms", skip),
			}
		}
		sync_basics!(AccountData);

		sync_struct! {
			#[derive(PartialEq)]
			pub struct Receipts {
				pub enabled: Option<bool> = ("enabled", skip),
				pub lists: Option<Vec<String>> = ("lists", skip),
				pub rooms: Option<Vec<ReceiptsRoom>> = ("rooms", skip),
			}
		}
		sync_basics!(Receipts);

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
					Some(other) => Ok(Self::Room(OwnedRoomId::from(other))),
					None => Err(DeError::expected("room ID or `*`")),
				}
			}
		}

		sync_struct! {
			#[derive(PartialEq)]
			pub struct Typing {
				pub enabled: Option<bool> = ("enabled", skip),
				pub lists: Option<Vec<String>> = ("lists", skip),
				pub rooms: Option<Vec<OwnedRoomId>> = ("rooms", skip),
			}
		}
		sync_basics!(Typing);
	}

	sync_struct! {
		pub struct Request {
			pub pos: Option<String> = ("pos", skip),
			pub conn_id: Option<String> = ("conn_id", skip),
			pub txn_id: Option<String> = ("txn_id", skip),
			pub timeout: Option<Duration> = ("timeout", skip),
			pub lists: BTreeMap<String, request::List> = ("lists", skip),
			pub room_subscriptions:
				BTreeMap<OwnedRoomId, request::RoomSubscription> = ("room_subscriptions", skip),
			pub extensions: request::Extensions = ("extensions", skip),
		}
	}
	sync_basics!(Request);

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

	sync_struct! {
		pub struct Response {
			pub txn_id: Option<String> = ("txn_id", skip),
			pub pos: String = ("pos", keep),
			pub lists: BTreeMap<String, response::List> = ("lists", skip),
			pub rooms: BTreeMap<OwnedRoomId, response::Room> = ("rooms", skip),
			pub extensions: response::Extensions = ("extensions", skip),
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
			serde::Raw,
		};

		sync_struct! {
			pub struct List {
				pub count: UInt = ("count", keep),
			}
		}

		sync_struct! {
			pub struct Room {
				pub name: Option<String> = ("name", skip),
				pub avatar: JsOption<OwnedMxcUri> = ("avatar", skip),
				pub initial: Option<bool> = ("initial", skip),
				pub is_dm: Option<bool> = ("is_dm", skip),
				pub invite_state: Option<Vec<Raw<AnyStrippedStateEvent>>> = ("invite_state", skip),
				pub unread_notifications: UnreadNotificationsCount = (flatten),
				pub timeline: Vec<Raw<AnySyncTimelineEvent>> = ("timeline", skip),
				pub required_state: Vec<Raw<AnySyncStateEvent>> = ("required_state", skip),
				pub prev_batch: Option<String> = ("prev_batch", skip),
				pub limited: bool = ("limited", skip),
				pub joined_count: Option<UInt> = ("joined_count", skip),
				pub invited_count: Option<UInt> = ("invited_count", skip),
				pub num_live: Option<UInt> = ("num_live", skip),
				pub bump_stamp: Option<UInt> = ("bump_stamp", skip),
				pub heroes: Option<Vec<Hero>> = ("heroes", skip),
			}
		}
		sync_basics!(Room);

		sync_struct! {
			pub struct Hero {
				pub user_id: OwnedUserId = ("user_id", keep),
				pub name: Option<String> = ("displayname", skip),
				pub avatar: Option<OwnedMxcUri> = ("avatar_url", skip),
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

		sync_struct! {
			pub struct Extensions {
				pub to_device: Option<ToDevice> = ("to_device", skip),
				pub e2ee: E2EE = ("e2ee", skip),
				pub account_data: AccountData = ("account_data", skip),
				pub receipts: Receipts = ("receipts", skip),
				pub typing: Typing = ("typing", skip),
			}
		}
		sync_basics!(Extensions);

		sync_struct! {
			pub struct ToDevice {
				pub next_batch: String = ("next_batch", keep),
				pub events: Vec<Raw<AnyToDeviceEvent>> = ("events", skip),
			}
		}

		sync_struct! {
			pub struct E2EE {
				pub device_lists: DeviceLists = ("device_lists", skip),
				pub device_one_time_keys_count:
					BTreeMap<OneTimeKeyAlgorithm, UInt> = ("device_one_time_keys_count", skip),
				pub device_unused_fallback_key_types:
					Option<Vec<OneTimeKeyAlgorithm>> = ("device_unused_fallback_key_types", skip),
			}
		}
		sync_basics!(E2EE);

		sync_struct! {
			pub struct AccountData {
				pub global: Vec<Raw<AnyGlobalAccountDataEvent>> = ("global", skip),
				pub rooms:
					BTreeMap<OwnedRoomId, Vec<Raw<AnyRoomAccountDataEvent>>> = ("rooms", skip),
			}
		}
		sync_basics!(AccountData);

		sync_struct! {
			pub struct Receipts {
				pub rooms: BTreeMap<OwnedRoomId, Raw<SyncReceiptEvent>> = ("rooms", skip),
			}
		}
		sync_basics!(Receipts);

		sync_struct! {
			pub struct Typing {
				pub rooms: BTreeMap<OwnedRoomId, Raw<SyncTypingEvent>> = ("rooms", skip),
			}
		}
		sync_basics!(Typing);
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

	sync_struct! {
		pub struct SyncRequestListFilters {
			pub is_dm: Option<bool> = ("is_dm", skip),
			pub spaces: Vec<String> = ("spaces", skip),
			pub is_encrypted: Option<bool> = ("is_encrypted", skip),
			pub is_invite: Option<bool> = ("is_invite", skip),
			pub is_tombstoned: Option<bool> = ("is_tombstoned", skip),
			pub room_types: Vec<RoomTypeFilter> = ("room_types", skip),
			pub not_room_types: Vec<RoomTypeFilter> = ("not_room_types", skip),
			pub room_name_like: Option<String> = ("room_name_like", skip),
			pub tags: Vec<String> = ("tags", skip),
			pub not_tags: Vec<String> = ("not_tags", skip),
		}
	}

	sync_struct! {
		pub struct SyncRequestList {
			pub slow_get_all_rooms: bool = ("slow_get_all_rooms", skip),
			pub ranges: Vec<(UInt, UInt)> = ("ranges", keep),
			pub sort: Vec<String> = ("sort", skip),
			pub room_details: RoomDetailsConfig = (flatten),
			pub include_old_rooms: Option<IncludeOldRooms> = ("include_old_rooms", skip),
			pub include_heroes: Option<bool> = ("include_heroes", skip),
			pub filters: Option<SyncRequestListFilters> = ("filters", skip),
			pub bump_event_types: Vec<TimelineEventType> = ("bump_event_types", skip),
		}
	}

	sync_struct! {
		pub struct RoomDetailsConfig {
			pub required_state: Vec<(StateEventType, String)> = ("required_state", skip),
			pub timeline_limit: Option<UInt> = ("timeline_limit", skip),
		}
	}

	sync_struct! {
		pub struct IncludeOldRooms {
			pub required_state: Vec<(StateEventType, String)> = ("required_state", skip),
			pub timeline_limit: Option<UInt> = ("timeline_limit", skip),
		}
	}

	sync_struct! {
		pub struct RoomSubscription {
			pub required_state: Vec<(StateEventType, String)> = ("required_state", skip),
			pub timeline_limit: Option<UInt> = ("timeline_limit", skip),
			pub include_heroes: Option<bool> = ("include_heroes", skip),
		}
	}

	sync_struct! {
		#[derive(PartialEq)]
		pub struct ToDeviceConfig {
			pub enabled: Option<bool> = ("enabled", skip),
			pub limit: Option<UInt> = ("limit", skip),
			pub since: Option<String> = ("since", skip),
			pub lists: Option<Vec<String>> = ("lists", skip),
			pub rooms: Option<Vec<OwnedRoomId>> = ("rooms", skip),
		}
	}

	sync_struct! {
		#[derive(PartialEq)]
		pub struct E2EEConfig {
			pub enabled: Option<bool> = ("enabled", skip),
		}
	}

	sync_struct! {
		#[derive(PartialEq)]
		pub struct AccountDataConfig {
			pub enabled: Option<bool> = ("enabled", skip),
			pub lists: Option<Vec<String>> = ("lists", skip),
			pub rooms: Option<Vec<OwnedRoomId>> = ("rooms", skip),
		}
	}

	sync_struct! {
		#[derive(PartialEq)]
		pub struct ReceiptsConfig {
			pub enabled: Option<bool> = ("enabled", skip),
			pub lists: Option<Vec<String>> = ("lists", skip),
			pub rooms: Option<Vec<ReceiptsRoom>> = ("rooms", skip),
		}
	}

	sync_struct! {
		#[derive(PartialEq)]
		pub struct TypingConfig {
			pub enabled: Option<bool> = ("enabled", skip),
			pub lists: Option<Vec<String>> = ("lists", skip),
			pub rooms: Option<Vec<OwnedRoomId>> = ("rooms", skip),
		}
	}

	sync_struct! {
		#[derive(PartialEq)]
		pub struct ExtensionsConfig {
			pub to_device: ToDeviceConfig = ("to_device", skip),
			pub e2ee: E2EEConfig = ("e2ee", skip),
			pub account_data: AccountDataConfig = ("account_data", skip),
			pub receipts: ReceiptsConfig = ("receipts", skip),
			pub typing: TypingConfig = ("typing", skip),
		}
	}
	sync_basics!(
		SyncRequestListFilters,
		SyncRequestList,
		RoomDetailsConfig,
		IncludeOldRooms,
		RoomSubscription,
		ToDeviceConfig,
		E2EEConfig,
		AccountDataConfig,
		ReceiptsConfig,
		TypingConfig,
		ExtensionsConfig,
	);

	sync_struct! {
		pub struct Request {
			pub pos: Option<String> = ("pos", skip),
			pub delta_token: Option<String> = ("delta_token", skip),
			pub conn_id: Option<String> = ("conn_id", skip),
			pub txn_id: Option<String> = ("txn_id", skip),
			pub timeout: Option<Duration> = ("timeout", skip),
			pub lists: BTreeMap<String, SyncRequestList> = ("lists", skip),
			pub room_subscriptions:
				BTreeMap<OwnedRoomId, RoomSubscription> = ("room_subscriptions", skip),
			pub unsubscribe_rooms: Vec<OwnedRoomId> = ("unsubscribe_rooms", skip),
			pub extensions: ExtensionsConfig = ("extensions", skip),
		}
	}
	sync_basics!(Request);

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
	#[derive(Clone, Debug, Default)]
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
		room.avatar = JsOption::Some(crate::OwnedMxcUri::from("mxc://x/y"));
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
		assert_eq!(config, config.clone());
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

//! Push rules: storage, server defaults and evaluation.

use alloc::{
	collections::BTreeMap,
	string::{String, ToString},
	vec,
	vec::Vec,
};

use crate::{
	Int, OwnedRoomId, OwnedUserId, RoomVersionId, UInt,
	codec::{DeError, Deserialize, Serialize},
	endpoint::object_from,
	events::AnySyncTimelineEvent,
	json::{Object, Value},
	power_levels::NotificationPowerLevels,
	push::{Action, Tweak},
	sswire::Raw,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RuleScope {
	Global,
}
impl crate::codec::Serialize for RuleScope {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Global => "global",
		}))
	}
}
impl crate::codec::Deserialize for RuleScope {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("global") => Ok(Self::Global),
			_ => Err(crate::codec::DeError::expected(stringify!(RuleScope))),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RuleKind {
	Override,
	Underride,
	PostContent,
	Sender,
	Room,
	Content,
}
impl crate::codec::Serialize for RuleKind {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::Override => "override",
			Self::Underride => "underride",
			Self::PostContent => "postcontent",
			Self::Sender => "sender",
			Self::Room => "room",
			Self::Content => "content",
		}))
	}
}
impl crate::codec::Deserialize for RuleKind {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("override") => Ok(Self::Override),
			Some("underride") => Ok(Self::Underride),
			Some("postcontent") => Ok(Self::PostContent),
			Some("sender") => Ok(Self::Sender),
			Some("room") => Ok(Self::Room),
			Some("content") => Ok(Self::Content),
			_ => Err(crate::codec::DeError::expected(stringify!(RuleKind))),
		}
	}
}

impl RuleKind {
	#[must_use]
	pub fn as_str(self) -> &'static str {
		match self {
			Self::Override => "override",
			Self::Underride => "underride",
			Self::Sender => "sender",
			Self::Room => "room",
			Self::Content => "content",
			Self::PostContent => "postcontent",
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredefinedOverrideRuleId {
	Master,
	SuppressNotices,
	InviteForMe,
	MemberEvent,
	IsUserMention,
	ContainsDisplayName,
	IsRoomMention,
	RoomNotif,
	Tombstone,
	Reaction,
	ServerAcl,
	SuppressEdits,
	PollResponse,
	StablePollResponse,
}
impl PredefinedOverrideRuleId {
	#[must_use]
	pub fn as_str(self) -> &'static str {
		match self {
			Self::Master => ".m.rule.master",
			Self::SuppressNotices => ".m.rule.suppress_notices",
			Self::InviteForMe => ".m.rule.invite_for_me",
			Self::MemberEvent => ".m.rule.member_event",
			Self::IsUserMention => ".m.rule.is_user_mention",
			Self::ContainsDisplayName => ".m.rule.contains_display_name",
			Self::IsRoomMention => ".m.rule.is_room_mention",
			Self::RoomNotif => ".m.rule.roomnotif",
			Self::Tombstone => ".m.rule.tombstone",
			Self::Reaction => ".m.rule.reaction",
			Self::ServerAcl => ".m.rule.room.server_acl",
			Self::SuppressEdits => ".m.rule.suppress_edits",
			Self::PollResponse => ".org.matrix.msc3930.rule.poll_response",
			Self::StablePollResponse => ".m.rule.poll_response",
		}
	}
}
impl AsRef<str> for PredefinedOverrideRuleId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredefinedContentRuleId {
	ContainsUserName,
}
impl PredefinedContentRuleId {
	#[must_use]
	pub fn as_str(self) -> &'static str {
		match self {
			Self::ContainsUserName => ".m.rule.contains_user_name",
		}
	}
}
impl AsRef<str> for PredefinedContentRuleId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredefinedUnderrideRuleId {
	Call,
	EncryptedRoomOneToOne,
	RoomOneToOne,
	Message,
	Encrypted,
	PollStartOneToOne,
	PollStart,
	PollEndOneToOne,
	PollEnd,
	StablePollStartOneToOne,
	StablePollStart,
	StablePollEndOneToOne,
	StablePollEnd,
}
impl PredefinedUnderrideRuleId {
	#[must_use]
	pub fn as_str(self) -> &'static str {
		match self {
			Self::Call => ".m.rule.call",
			Self::EncryptedRoomOneToOne => ".m.rule.encrypted_room_one_to_one",
			Self::RoomOneToOne => ".m.rule.room_one_to_one",
			Self::Message => ".m.rule.message",
			Self::Encrypted => ".m.rule.encrypted",
			Self::PollStartOneToOne => ".org.matrix.msc3930.rule.poll_start_one_to_one",
			Self::PollStart => ".org.matrix.msc3930.rule.poll_start",
			Self::PollEndOneToOne => ".org.matrix.msc3930.rule.poll_end_one_to_one",
			Self::PollEnd => ".org.matrix.msc3930.rule.poll_end",
			Self::StablePollStartOneToOne => ".m.rule.poll_start_one_to_one",
			Self::StablePollStart => ".m.rule.poll_start",
			Self::StablePollEndOneToOne => ".m.rule.poll_end_one_to_one",
			Self::StablePollEnd => ".m.rule.poll_end",
		}
	}
}
impl AsRef<str> for PredefinedUnderrideRuleId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredefinedPostContentRuleId {
	UnsubscribedThread,
	SubscribedThread,
}
impl PredefinedPostContentRuleId {
	#[must_use]
	pub fn as_str(self) -> &'static str {
		match self {
			Self::UnsubscribedThread => ".io.element.msc4306.rule.unsubscribed_thread",
			Self::SubscribedThread => ".io.element.msc4306.rule.subscribed_thread",
		}
	}
}
impl AsRef<str> for PredefinedPostContentRuleId {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// A condition that must hold for a rule to apply.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PushCondition {
	EventMatch {
		key: String,
		pattern: String,
	},
	ContainsDisplayName,
	RoomMemberCount {
		is: String,
	},
	SenderNotificationPermission {
		key: String,
	},
	EventPropertyIs {
		key: String,
		value: Value,
	},
	EventPropertyContains {
		key: String,
		value: Value,
	},
	ThreadSubscription {
		subscribed: bool,
	},
	Unknown(Value),
}

fn text_field(object: &Object, name: &str) -> Result<String, DeError> {
	object
		.get(name)
		.and_then(Value::as_str)
		.map(String::from)
		.ok_or_else(|| DeError(alloc::format!("expected string field `{name}`")))
}

impl Serialize for PushCondition {
	fn to_json(&self) -> Value {
		let string = |s: &String| Value::String(s.clone());
		let fields = match self {
			Self::EventMatch {
				key,
				pattern,
			} => vec![
				("kind", Value::String("event_match".into())),
				("key", string(key)),
				("pattern", string(pattern)),
			],
			Self::ContainsDisplayName => {
				vec![("kind", Value::String("contains_display_name".into()))]
			}
			Self::RoomMemberCount {
				is,
			} => vec![("kind", Value::String("room_member_count".into())), ("is", string(is))],
			Self::SenderNotificationPermission {
				key,
			} => vec![
				("kind", Value::String("sender_notification_permission".into())),
				("key", string(key)),
			],
			Self::EventPropertyIs {
				key,
				value,
			} => vec![
				("kind", Value::String("event_property_is".into())),
				("key", string(key)),
				("value", value.clone()),
			],
			Self::EventPropertyContains {
				key,
				value,
			} => vec![
				("kind", Value::String("event_property_contains".into())),
				("key", string(key)),
				("value", value.clone()),
			],
			Self::ThreadSubscription {
				subscribed,
			} => vec![
				("kind", Value::String("io.element.msc4306.thread_subscription".into())),
				("subscribed", Value::Bool(*subscribed)),
			],
			Self::Unknown(value) => return value.clone(),
		};
		Value::Object(object_from(fields))
	}
}

impl Deserialize for PushCondition {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("condition object"))?;
		let property = |name: &str| object.get(name).cloned().unwrap_or_default();
		Ok(match text_field(object, "kind")?.as_str() {
			"event_match" => Self::EventMatch {
				key: text_field(object, "key")?,
				pattern: text_field(object, "pattern")?,
			},
			"contains_display_name" => Self::ContainsDisplayName,
			"room_member_count" => Self::RoomMemberCount {
				is: text_field(object, "is")?,
			},
			"sender_notification_permission" => Self::SenderNotificationPermission {
				key: text_field(object, "key")?,
			},
			"event_property_is" => Self::EventPropertyIs {
				key: text_field(object, "key")?,
				value: property("value"),
			},
			"event_property_contains" => Self::EventPropertyContains {
				key: text_field(object, "key")?,
				value: property("value"),
			},
			"io.element.msc4306.thread_subscription" => Self::ThreadSubscription {
				subscribed: object
					.get("subscribed")
					.and_then(Value::as_bool)
					.ok_or_else(|| DeError::expected("subscribed"))?,
			},
			_ => Self::Unknown(value.clone()),
		})
	}
}

/// Rule with a list of conditions (override and underride rules).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConditionalPushRule {
	pub actions: Vec<Action>,
	pub default: bool,
	pub enabled: bool,
	pub rule_id: String,
	pub conditions: Vec<PushCondition>,
}

/// Rule matching a glob against the message body (content rules).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternedPushRule {
	pub actions: Vec<Action>,
	pub default: bool,
	pub enabled: bool,
	pub rule_id: String,
	pub pattern: String,
}

/// Rule keyed by room or sender ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimplePushRule<T> {
	pub actions: Vec<Action>,
	pub default: bool,
	pub enabled: bool,
	pub rule_id: T,
}

fn rule_base(
	actions: &[Action],
	default: bool,
	enabled: bool,
	rule_id: &str,
	extra: Vec<(&str, Value)>,
) -> Value {
	let mut fields = vec![
		("actions", actions.to_vec().to_json()),
		("default", Value::Bool(default)),
		("enabled", Value::Bool(enabled)),
		("rule_id", Value::String(rule_id.into())),
	];
	fields.extend(extra);
	Value::Object(object_from(fields))
}

fn rule_actions(object: &Object) -> Result<Vec<Action>, DeError> {
	object.get("actions").map_or_else(|| Ok(Vec::new()), Vec::<Action>::from_json)
}

fn rule_flags(object: &Object) -> (bool, bool) {
	(
		object.get("default").and_then(Value::as_bool).unwrap_or(false),
		object.get("enabled").and_then(Value::as_bool).unwrap_or(true),
	)
}

impl Serialize for ConditionalPushRule {
	fn to_json(&self) -> Value {
		rule_base(
			&self.actions,
			self.default,
			self.enabled,
			&self.rule_id,
			vec![("conditions", self.conditions.to_json())],
		)
	}
}
impl Deserialize for ConditionalPushRule {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("rule object"))?;
		let (default, enabled) = rule_flags(object);
		Ok(Self {
			actions: rule_actions(object)?,
			default,
			enabled,
			rule_id: text_field(object, "rule_id")?,
			conditions: object
				.get("conditions")
				.map_or_else(|| Ok(Vec::new()), Vec::<PushCondition>::from_json)?,
		})
	}
}

impl Serialize for PatternedPushRule {
	fn to_json(&self) -> Value {
		rule_base(
			&self.actions,
			self.default,
			self.enabled,
			&self.rule_id,
			vec![("pattern", Value::String(self.pattern.clone()))],
		)
	}
}
impl Deserialize for PatternedPushRule {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("rule object"))?;
		let (default, enabled) = rule_flags(object);
		Ok(Self {
			actions: rule_actions(object)?,
			default,
			enabled,
			rule_id: text_field(object, "rule_id")?,
			pattern: text_field(object, "pattern")?,
		})
	}
}

impl<T: Serialize> Serialize for SimplePushRule<T> {
	fn to_json(&self) -> Value {
		rule_base(
			&self.actions,
			self.default,
			self.enabled,
			self.rule_id.to_json().as_str().unwrap_or_default(),
			Vec::new(),
		)
	}
}
impl<T: Deserialize> Deserialize for SimplePushRule<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("rule object"))?;
		let (default, enabled) = rule_flags(object);
		Ok(Self {
			actions: rule_actions(object)?,
			default,
			enabled,
			rule_id: T::from_json(
				object.get("rule_id").ok_or_else(|| DeError::expected("rule_id"))?,
			)?,
		})
	}
}

/// An owned rule of any kind.
#[derive(Debug, Eq, PartialEq)]
pub enum AnyPushRule {
	Override(ConditionalPushRule),
	Content(PatternedPushRule),
	Room(SimplePushRule<OwnedRoomId>),
	Sender(SimplePushRule<OwnedUserId>),
	Underride(ConditionalPushRule),
	PostContent(ConditionalPushRule),
}

/// A borrowed rule of any kind.
#[derive(Clone, Copy, Debug)]
pub enum AnyPushRuleRef<'a> {
	Override(&'a ConditionalPushRule),
	Content(&'a PatternedPushRule),
	Room(&'a SimplePushRule<OwnedRoomId>),
	Sender(&'a SimplePushRule<OwnedUserId>),
	Underride(&'a ConditionalPushRule),
	PostContent(&'a ConditionalPushRule),
}

impl AnyPushRuleRef<'_> {
	#[must_use]
	pub fn actions(&self) -> &[Action] {
		match self {
			Self::Override(r) | Self::Underride(r) | Self::PostContent(r) => &r.actions,
			Self::Content(r) => &r.actions,
			Self::Room(r) => &r.actions,
			Self::Sender(r) => &r.actions,
		}
	}

	#[must_use]
	pub fn enabled(self) -> bool {
		match self {
			Self::Override(r) | Self::Underride(r) | Self::PostContent(r) => r.enabled,
			Self::Content(r) => r.enabled,
			Self::Room(r) => r.enabled,
			Self::Sender(r) => r.enabled,
		}
	}

	#[must_use]
	pub fn rule_id(&self) -> &str {
		match self {
			Self::Override(r) | Self::Underride(r) | Self::PostContent(r) => &r.rule_id,
			Self::Content(r) => &r.rule_id,
			Self::Room(r) => r.rule_id.as_str(),
			Self::Sender(r) => r.rule_id.as_str(),
		}
	}
}

impl From<AnyPushRuleRef<'_>> for AnyPushRule {
	fn from(rule: AnyPushRuleRef<'_>) -> Self {
		match rule {
			AnyPushRuleRef::Override(r) => Self::Override(r.clone()),
			AnyPushRuleRef::Content(r) => Self::Content(r.clone()),
			AnyPushRuleRef::Room(r) => Self::Room(r.clone()),
			AnyPushRuleRef::Sender(r) => Self::Sender(r.clone()),
			AnyPushRuleRef::Underride(r) => Self::Underride(r.clone()),
			AnyPushRuleRef::PostContent(r) => Self::PostContent(r.clone()),
		}
	}
}

impl Serialize for AnyPushRule {
	fn to_json(&self) -> Value {
		match self {
			Self::Override(r) | Self::Underride(r) | Self::PostContent(r) => r.to_json(),
			Self::Content(r) => r.to_json(),
			Self::Room(r) => r.to_json(),
			Self::Sender(r) => r.to_json(),
		}
	}
}

/// The rule shape returned by `GET /pushrules/global/{kind}/{ruleId}`.
pub type PushRule = AnyPushRule;

/// A rule to be created by `PUT /pushrules/global/{kind}/{ruleId}`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NewPushRule {
	Override(NewConditionalPushRule),
	Content(NewPatternedPushRule),
	Room(NewSimplePushRule<OwnedRoomId>),
	Sender(NewSimplePushRule<OwnedUserId>),
	Underride(NewConditionalPushRule),
	PostContent(NewConditionalPushRule),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewConditionalPushRule {
	pub rule_id: String,
	pub conditions: Vec<PushCondition>,
	pub actions: Vec<Action>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewPatternedPushRule {
	pub rule_id: String,
	pub pattern: String,
	pub actions: Vec<Action>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewSimplePushRule<T> {
	pub rule_id: T,
	pub actions: Vec<Action>,
}

impl NewPushRule {
	#[must_use]
	pub fn kind(&self) -> RuleKind {
		match self {
			Self::Override(_) => RuleKind::Override,
			Self::Content(_) => RuleKind::Content,
			Self::Room(_) => RuleKind::Room,
			Self::Sender(_) => RuleKind::Sender,
			Self::Underride(_) => RuleKind::Underride,
			Self::PostContent(_) => RuleKind::PostContent,
		}
	}

	#[must_use]
	pub fn rule_id(&self) -> &str {
		match self {
			Self::Override(r) | Self::Underride(r) | Self::PostContent(r) => &r.rule_id,
			Self::Content(r) => &r.rule_id,
			Self::Room(r) => r.rule_id.as_str(),
			Self::Sender(r) => r.rule_id.as_str(),
		}
	}

	/// The request-body fields: everything except the ID, which is in the path.
	#[must_use]
	pub fn body_json(&self) -> Value {
		let (actions, extra) = match self {
			Self::Override(r) | Self::Underride(r) | Self::PostContent(r) => {
				(&r.actions, vec![("conditions", r.conditions.to_json())])
			}
			Self::Content(r) => (&r.actions, vec![("pattern", Value::String(r.pattern.clone()))]),
			Self::Room(r) => (&r.actions, Vec::new()),
			Self::Sender(r) => (&r.actions, Vec::new()),
		};
		let mut fields = vec![("actions", actions.to_json())];
		fields.extend(extra);
		Value::Object(object_from(fields))
	}

	/// Parses a request body for the given kind and ID.
	///
	/// # Errors
	///
	/// Returns an error if the body does not match the kind.
	pub fn from_body(kind: RuleKind, rule_id: String, body: &Value) -> Result<Self, DeError> {
		let object = body.as_object().ok_or_else(|| DeError::expected("rule object"))?;
		let actions = rule_actions(object)?;
		let conditions = || {
			object
				.get("conditions")
				.map_or_else(|| Ok(Vec::new()), Vec::<PushCondition>::from_json)
		};
		Ok(match kind {
			RuleKind::Override => Self::Override(NewConditionalPushRule {
				rule_id,
				conditions: conditions()?,
				actions,
			}),
			RuleKind::Underride => Self::Underride(NewConditionalPushRule {
				rule_id,
				conditions: conditions()?,
				actions,
			}),
			RuleKind::PostContent => Self::PostContent(NewConditionalPushRule {
				rule_id,
				conditions: conditions()?,
				actions,
			}),
			RuleKind::Content => Self::Content(NewPatternedPushRule {
				rule_id,
				pattern: text_field(object, "pattern")?,
				actions,
			}),
			RuleKind::Room => Self::Room(NewSimplePushRule {
				rule_id: OwnedRoomId::parse(rule_id)
					.map_err(|_| DeError::expected("room rule ID"))?,
				actions,
			}),
			RuleKind::Sender => Self::Sender(NewSimplePushRule {
				rule_id: OwnedUserId::parse(rule_id)
					.map_err(|_| DeError::expected("user rule ID"))?,
				actions,
			}),
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsertPushRuleError {
	ServerDefaultRuleId,
	InvalidRuleId,
	RelativeToServerDefaultRule,
	UnknownRuleId,
	BeforeHigherThanAfter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemovePushRuleError {
	ServerDefault,
	NotFound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleNotFoundError;

impl core::fmt::Display for InsertPushRuleError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str("invalid push rule insertion")
	}
}
impl core::error::Error for InsertPushRuleError {}
impl core::fmt::Display for RemovePushRuleError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str("cannot remove push rule")
	}
}
impl core::error::Error for RemovePushRuleError {}
impl core::fmt::Display for RuleNotFoundError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str("push rule not found")
	}
}
impl core::error::Error for RuleNotFoundError {}

/// A user's push rules, ordered by priority within each kind.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ruleset {
	pub override_: RuleList<ConditionalPushRule>,
	pub content: RuleList<PatternedPushRule>,
	pub room: RuleList<SimplePushRule<OwnedRoomId>>,
	pub sender: RuleList<SimplePushRule<OwnedUserId>>,
	pub underride: RuleList<ConditionalPushRule>,
	pub postcontent: RuleList<ConditionalPushRule>,
}

/// A rule with an ID unique within its list.
pub trait HasRuleId {
	fn rule_id(&self) -> &str;
}
impl HasRuleId for ConditionalPushRule {
	fn rule_id(&self) -> &str {
		&self.rule_id
	}
}
impl HasRuleId for PatternedPushRule {
	fn rule_id(&self) -> &str {
		&self.rule_id
	}
}
impl<T: AsRef<str>> HasRuleId for SimplePushRule<T> {
	fn rule_id(&self) -> &str {
		self.rule_id.as_ref()
	}
}

/// An ordered list of rules keyed by rule ID.
///
/// Dereferences to the underlying `Vec` for iteration and in-place edits; the
/// by-ID `get`, `insert` and `shift_remove` mirror an insertion-ordered set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleList<T>(Vec<T>);

impl<T> Default for RuleList<T> {
	fn default() -> Self {
		Self(Vec::new())
	}
}
impl<T> From<Vec<T>> for RuleList<T> {
	fn from(rules: Vec<T>) -> Self {
		Self(rules)
	}
}
impl<T> core::ops::Deref for RuleList<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Vec<T> {
		&self.0
	}
}
impl<T> core::ops::DerefMut for RuleList<T> {
	fn deref_mut(&mut self) -> &mut Vec<T> {
		&mut self.0
	}
}
impl<'a, T> IntoIterator for &'a RuleList<T> {
	type IntoIter = core::slice::Iter<'a, T>;
	type Item = &'a T;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}
impl<T> IntoIterator for RuleList<T> {
	type IntoIter = alloc::vec::IntoIter<T>;
	type Item = T;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}
impl<T> FromIterator<T> for RuleList<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self(iter.into_iter().collect())
	}
}

impl<T: HasRuleId> RuleList<T> {
	#[must_use]
	pub fn get(&self, rule_id: &str) -> Option<&T> {
		self.0.iter().find(|rule| rule.rule_id() == rule_id)
	}

	/// Adds a rule at the end, replacing any rule with the same ID in place.
	/// Returns whether the ID was new.
	pub fn insert(&mut self, rule: T) -> bool {
		if let Some(slot) = self.0.iter_mut().find(|r| r.rule_id() == rule.rule_id()) {
			*slot = rule;
			false
		} else {
			self.0.push(rule);
			true
		}
	}

	/// Removes the rule with this ID, keeping the order of the rest.
	pub fn shift_remove(&mut self, rule_id: &str) -> Option<T> {
		let index = self.0.iter().position(|r| r.rule_id() == rule_id)?;
		Some(self.0.remove(index))
	}
}

/// Power-level data needed to evaluate `sender_notification_permission`.
#[derive(Debug, Default)]
pub struct PushConditionPowerLevelsCtx {
	pub users: BTreeMap<OwnedUserId, Int>,
	pub users_default: Int,
	pub notifications: NotificationPowerLevels,
}

/// Room context for evaluating push conditions.
#[derive(Debug)]
pub struct PushConditionRoomCtx {
	pub room_id: OwnedRoomId,
	pub member_count: UInt,
	pub user_id: OwnedUserId,
	pub user_display_name: String,
	pub power_levels: Option<PushConditionPowerLevelsCtx>,
	pub room_version: Option<RoomVersionId>,
	/// Precomputed subscription state for the event's thread root.
	/// `None` means the event is not a valid thread event or the state is unknown.
	pub thread_subscription: Option<bool>,
}

fn action_list(tweaks: &[Tweak], notify: bool) -> Vec<Action> {
	let mut actions = Vec::new();
	if notify {
		actions.push(Action::Notify);
	}
	actions.extend(tweaks.iter().cloned().map(Action::SetTweak));
	actions
}

fn event_match(key: &str, pattern: &str) -> PushCondition {
	PushCondition::EventMatch {
		key: key.into(),
		pattern: pattern.into(),
	}
}

fn default_conditional(
	id: &str,
	enabled: bool,
	conditions: Vec<PushCondition>,
	actions: Vec<Action>,
) -> ConditionalPushRule {
	ConditionalPushRule {
		actions,
		default: true,
		enabled,
		rule_id: id.into(),
		conditions,
	}
}

/// MSC3930: matches an event whose `type` is exactly `event_type`.
fn type_is(event_type: &str) -> PushCondition {
	PushCondition::EventPropertyIs {
		key: "type".into(),
		value: Value::String(event_type.into()),
	}
}

fn member_count_is(is: &str) -> PushCondition {
	PushCondition::RoomMemberCount {
		is: is.into(),
	}
}

impl Ruleset {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// The server-default rules for `user_id`.
	#[must_use]
	pub fn server_default(user_id: impl AsRef<str>) -> Self {
		let user_id = user_id.as_ref();
		Self {
			override_: default_override_rules(user_id).into(),
			underride: default_underride_rules().into(),
			postcontent: default_postcontent_rules().into(),
			..Self::default()
		}
	}
	/// Finds a rule by kind and ID.
	#[must_use]
	pub fn get(&self, kind: RuleKind, rule_id: impl AsRef<str>) -> Option<AnyPushRuleRef<'_>> {
		let id = rule_id.as_ref();
		match kind {
			RuleKind::Override => {
				self.override_.iter().find(|r| r.rule_id == id).map(AnyPushRuleRef::Override)
			}
			RuleKind::Underride => {
				self.underride.iter().find(|r| r.rule_id == id).map(AnyPushRuleRef::Underride)
			}
			RuleKind::PostContent => {
				self.postcontent.iter().find(|r| r.rule_id == id).map(AnyPushRuleRef::PostContent)
			}
			RuleKind::Content => {
				self.content.iter().find(|r| r.rule_id == id).map(AnyPushRuleRef::Content)
			}
			RuleKind::Room => {
				self.room.iter().find(|r| r.rule_id == id).map(AnyPushRuleRef::Room)
			}
			RuleKind::Sender => {
				self.sender.iter().find(|r| r.rule_id == id).map(AnyPushRuleRef::Sender)
			}
		}
	}

	/// Inserts or replaces a user rule, placing it relative to another rule.
	///
	/// With neither `after` nor `before` the rule becomes the highest priority
	/// rule of its kind.
	///
	/// # Errors
	///
	/// Returns an error if the rule ID is reserved or invalid, or if the
	/// anchor rules are unknown, reserved, or in the wrong order.
	pub fn insert(
		&mut self,
		rule: NewPushRule,
		after: Option<&str>,
		before: Option<&str>,
	) -> Result<(), InsertPushRuleError> {
		let kind = rule.kind();
		let id = rule.rule_id().to_string();
		if id.starts_with('.') {
			return Err(InsertPushRuleError::ServerDefaultRuleId);
		}
		if id.is_empty() || id.contains(['/', '\\']) {
			return Err(InsertPushRuleError::InvalidRuleId);
		}
		if after.is_some_and(|a| a.starts_with('.')) || before.is_some_and(|b| b.starts_with('.'))
		{
			return Err(InsertPushRuleError::RelativeToServerDefaultRule);
		}

		let ids = self.ids(kind);
		let position = |anchor: &str| ids.iter().position(|id| id == anchor);
		let after_pos = after.map(|a| position(a).ok_or(InsertPushRuleError::UnknownRuleId));
		let before_pos = before.map(|b| position(b).ok_or(InsertPushRuleError::UnknownRuleId));
		let (after_pos, before_pos) = (after_pos.transpose()?, before_pos.transpose()?);
		if matches!((after_pos, before_pos), (Some(a), Some(b)) if b > a) {
			return Err(InsertPushRuleError::BeforeHigherThanAfter);
		}
		let own = position(&id);

		let mut target = match (after_pos, before_pos) {
			(Some(a), _) => a.saturating_add(1),
			(None, Some(b)) => b,
			(None, None) => 0,
		};
		if own.is_some_and(|own| own < target) {
			target = target.saturating_sub(1);
		}

		self.remove_unchecked(kind, &id);
		self.place(rule, target);
		Ok(())
	}

	/// Removes a user rule.
	///
	/// # Errors
	///
	/// Returns an error if the rule is a server default or does not exist.
	pub fn remove(
		&mut self,
		kind: RuleKind,
		rule_id: impl AsRef<str>,
	) -> Result<(), RemovePushRuleError> {
		let id = rule_id.as_ref();
		match self.get(kind, id) {
			None => Err(RemovePushRuleError::NotFound),
			Some(rule) if is_default(rule) => Err(RemovePushRuleError::ServerDefault),
			Some(_) => {
				self.remove_unchecked(kind, id);
				Ok(())
			}
		}
	}

	/// Replaces a rule's actions.
	///
	/// # Errors
	///
	/// Returns an error if the rule does not exist.
	pub fn set_actions(
		&mut self,
		kind: RuleKind,
		rule_id: impl AsRef<str>,
		actions: Vec<Action>,
	) -> Result<(), RuleNotFoundError> {
		let id = rule_id.as_ref();
		let slot = match kind {
			RuleKind::Override => {
				self.override_.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.actions)
			}
			RuleKind::Underride => {
				self.underride.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.actions)
			}
			RuleKind::PostContent => {
				self.postcontent.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.actions)
			}
			RuleKind::Content => {
				self.content.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.actions)
			}
			RuleKind::Room => {
				self.room.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.actions)
			}
			RuleKind::Sender => {
				self.sender.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.actions)
			}
		};
		*slot.ok_or(RuleNotFoundError)? = actions;
		Ok(())
	}

	/// Enables or disables a rule.
	///
	/// # Errors
	///
	/// Returns an error if the rule does not exist.
	pub fn set_enabled(
		&mut self,
		kind: RuleKind,
		rule_id: impl AsRef<str>,
		enabled: bool,
	) -> Result<(), RuleNotFoundError> {
		let id = rule_id.as_ref();
		let slot = match kind {
			RuleKind::Override => {
				self.override_.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.enabled)
			}
			RuleKind::Underride => {
				self.underride.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.enabled)
			}
			RuleKind::PostContent => {
				self.postcontent.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.enabled)
			}
			RuleKind::Content => {
				self.content.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.enabled)
			}
			RuleKind::Room => {
				self.room.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.enabled)
			}
			RuleKind::Sender => {
				self.sender.iter_mut().find(|r| r.rule_id == id).map(|r| &mut r.enabled)
			}
		};
		*slot.ok_or(RuleNotFoundError)? = enabled;
		Ok(())
	}

	/// Brings the default rules up to date with `defaults`, keeping the
	/// user's enabled flag and actions, adding missing rules and dropping
	/// default rules that no longer exist.
	pub fn update_with_server_default(&mut self, defaults: Self) {
		merge_conditional(&mut self.override_.0, defaults.override_.0);
		merge_conditional(&mut self.underride.0, defaults.underride.0);
		merge_conditional(&mut self.postcontent.0, defaults.postcontent.0);
		self.content.retain(|r| !r.default);
		self.room.retain(|r| !r.default);
		self.sender.retain(|r| !r.default);
		self.postcontent.retain(|r| !r.default);
	}

	/// The actions of the first enabled rule matching `event`, or none.
	#[must_use]
	pub fn get_actions(
		&self,
		event: &Raw<AnySyncTimelineEvent>,
		ctx: &PushConditionRoomCtx,
	) -> &[Action] {
		let Ok(value) = event.json() else {
			return &[];
		};
		let matcher = Matcher {
			event: &value,
			ctx,
		};
		for rule in self.override_.iter().filter(|r| r.enabled) {
			if rule.conditions.iter().all(|c| matcher.condition(c)) {
				return &rule.actions;
			}
		}
		for rule in self.content.iter().filter(|r| r.enabled) {
			if matcher.body_matches(&rule.pattern) {
				return &rule.actions;
			}
		}
		for rule in self.postcontent.iter().filter(|r| r.enabled) {
			if rule.conditions.iter().all(|c| matcher.condition(c)) {
				return &rule.actions;
			}
		}
		for rule in self.room.iter().filter(|r| r.enabled) {
			if rule.rule_id == ctx.room_id.as_str() {
				return &rule.actions;
			}
		}
		for rule in self.sender.iter().filter(|r| r.enabled) {
			if matcher.string("sender") == Some(rule.rule_id.as_str()) {
				return &rule.actions;
			}
		}
		for rule in self.underride.iter().filter(|r| r.enabled) {
			if rule.conditions.iter().all(|c| matcher.condition(c)) {
				return &rule.actions;
			}
		}
		&[]
	}

	fn ids(&self, kind: RuleKind) -> Vec<String> {
		match kind {
			RuleKind::Override => self.override_.iter().map(|r| r.rule_id.clone()).collect(),
			RuleKind::Underride => self.underride.iter().map(|r| r.rule_id.clone()).collect(),
			RuleKind::PostContent => self.postcontent.iter().map(|r| r.rule_id.clone()).collect(),
			RuleKind::Content => self.content.iter().map(|r| r.rule_id.clone()).collect(),
			RuleKind::Room => self.room.iter().map(|r| r.rule_id.as_str().to_owned()).collect(),
			RuleKind::Sender => {
				self.sender.iter().map(|r| r.rule_id.as_str().to_owned()).collect()
			}
		}
	}

	fn remove_unchecked(&mut self, kind: RuleKind, id: &str) {
		match kind {
			RuleKind::Override => self.override_.retain(|r| r.rule_id != id),
			RuleKind::Underride => self.underride.retain(|r| r.rule_id != id),
			RuleKind::PostContent => self.postcontent.retain(|r| r.rule_id != id),
			RuleKind::Content => self.content.retain(|r| r.rule_id != id),
			RuleKind::Room => self.room.retain(|r| r.rule_id.as_str() != id),
			RuleKind::Sender => self.sender.retain(|r| r.rule_id.as_str() != id),
		}
	}

	fn place(&mut self, rule: NewPushRule, at: usize) {
		fn put<T>(list: &mut Vec<T>, at: usize, item: T) {
			list.insert(at.min(list.len()), item);
		}
		match rule {
			NewPushRule::Override(r) => put(&mut self.override_.0, at, new_conditional(r)),
			NewPushRule::Underride(r) => put(&mut self.underride.0, at, new_conditional(r)),
			NewPushRule::PostContent(r) => put(&mut self.postcontent.0, at, new_conditional(r)),
			NewPushRule::Content(r) => put(
				&mut self.content,
				at,
				PatternedPushRule {
					actions: r.actions,
					default: false,
					enabled: true,
					rule_id: r.rule_id,
					pattern: r.pattern,
				},
			),
			NewPushRule::Room(r) => put(&mut self.room.0, at, new_simple(r)),
			NewPushRule::Sender(r) => put(&mut self.sender.0, at, new_simple(r)),
		}
	}
}

fn default_override_rules(user_id: &str) -> Vec<ConditionalPushRule> {
	let sound = Tweak::Sound("default".into());
	let highlight = Tweak::Highlight(true);
	let quiet = Tweak::Highlight(false);
	let mention_user = PushCondition::EventPropertyContains {
		key: "content.m\\.mentions.user_ids".into(),
		value: Value::String(user_id.into()),
	};
	let mention_room = PushCondition::EventPropertyIs {
		key: "content.m\\.mentions.room".into(),
		value: Value::Bool(true),
	};
	let room_permission = PushCondition::SenderNotificationPermission {
		key: "room".into(),
	};

	vec![
		default_conditional(
			PredefinedOverrideRuleId::Master.as_str(),
			false,
			Vec::new(),
			Vec::new(),
		),
		default_conditional(
			PredefinedOverrideRuleId::SuppressNotices.as_str(),
			true,
			vec![event_match("content.msgtype", "m.notice")],
			Vec::new(),
		),
		default_conditional(
			PredefinedOverrideRuleId::InviteForMe.as_str(),
			true,
			vec![
				event_match("type", "m.room.member"),
				event_match("content.membership", "invite"),
				event_match("state_key", user_id),
			],
			action_list(core::slice::from_ref(&sound), true),
		),
		default_conditional(
			PredefinedOverrideRuleId::MemberEvent.as_str(),
			true,
			vec![event_match("type", "m.room.member")],
			Vec::new(),
		),
		default_conditional(
			PredefinedOverrideRuleId::IsUserMention.as_str(),
			true,
			vec![mention_user],
			action_list(&[sound.clone(), highlight.clone()], true),
		),
		default_conditional(
			PredefinedOverrideRuleId::IsRoomMention.as_str(),
			true,
			vec![mention_room, room_permission],
			action_list(&[sound.clone(), highlight], true),
		),
		default_conditional(
			PredefinedOverrideRuleId::Tombstone.as_str(),
			true,
			vec![event_match("type", "m.room.tombstone"), event_match("state_key", "")],
			action_list(&[quiet], true),
		),
		default_conditional(
			PredefinedOverrideRuleId::Reaction.as_str(),
			true,
			vec![event_match("type", "m.reaction")],
			Vec::new(),
		),
		default_conditional(
			PredefinedOverrideRuleId::ServerAcl.as_str(),
			true,
			vec![event_match("type", "m.room.server_acl"), event_match("state_key", "")],
			Vec::new(),
		),
		default_conditional(
			PredefinedOverrideRuleId::SuppressEdits.as_str(),
			true,
			vec![PushCondition::EventPropertyIs {
				key: "content.m\\.relates_to.rel_type".into(),
				value: Value::String("m.replace".into()),
			}],
			Vec::new(),
		),
		// MSC3930: poll responses never notify.
		default_conditional(
			PredefinedOverrideRuleId::PollResponse.as_str(),
			true,
			vec![type_is("org.matrix.msc3381.poll.response")],
			Vec::new(),
		),
		default_conditional(
			PredefinedOverrideRuleId::StablePollResponse.as_str(),
			true,
			vec![type_is("m.poll.response")],
			Vec::new(),
		),
	]
}

fn default_underride_rules() -> Vec<ConditionalPushRule> {
	let sound = Tweak::Sound("default".into());
	let ring = Tweak::Sound("ring".into());
	vec![
		default_conditional(
			PredefinedUnderrideRuleId::Call.as_str(),
			true,
			vec![event_match("type", "m.call.invite")],
			action_list(&[ring, Tweak::Highlight(false)], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::EncryptedRoomOneToOne.as_str(),
			true,
			vec![member_count_is("2"), event_match("type", "m.room.encrypted")],
			action_list(&[sound.clone(), Tweak::Highlight(false)], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::RoomOneToOne.as_str(),
			true,
			vec![member_count_is("2"), event_match("type", "m.room.message")],
			action_list(&[sound.clone(), Tweak::Highlight(false)], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::Message.as_str(),
			true,
			vec![event_match("type", "m.room.message")],
			action_list(&[Tweak::Highlight(false)], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::Encrypted.as_str(),
			true,
			vec![event_match("type", "m.room.encrypted")],
			action_list(&[Tweak::Highlight(false)], true),
		),
		// MSC3930: polls notify like messages do.
		default_conditional(
			PredefinedUnderrideRuleId::PollStartOneToOne.as_str(),
			true,
			vec![member_count_is("2"), type_is("org.matrix.msc3381.poll.start")],
			action_list(core::slice::from_ref(&sound), true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::PollStart.as_str(),
			true,
			vec![type_is("org.matrix.msc3381.poll.start")],
			action_list(&[], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::PollEndOneToOne.as_str(),
			true,
			vec![member_count_is("2"), type_is("org.matrix.msc3381.poll.end")],
			action_list(core::slice::from_ref(&sound), true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::PollEnd.as_str(),
			true,
			vec![type_is("org.matrix.msc3381.poll.end")],
			action_list(&[], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::StablePollStartOneToOne.as_str(),
			true,
			vec![member_count_is("2"), type_is("m.poll.start")],
			action_list(core::slice::from_ref(&sound), true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::StablePollStart.as_str(),
			true,
			vec![type_is("m.poll.start")],
			action_list(&[], true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::StablePollEndOneToOne.as_str(),
			true,
			vec![member_count_is("2"), type_is("m.poll.end")],
			action_list(core::slice::from_ref(&sound), true),
		),
		default_conditional(
			PredefinedUnderrideRuleId::StablePollEnd.as_str(),
			true,
			vec![type_is("m.poll.end")],
			action_list(&[], true),
		),
	]
}

fn default_postcontent_rules() -> Vec<ConditionalPushRule> {
	vec![
		default_conditional(
			PredefinedPostContentRuleId::UnsubscribedThread.as_str(),
			true,
			vec![PushCondition::ThreadSubscription {
				subscribed: false,
			}],
			Vec::new(),
		),
		default_conditional(
			PredefinedPostContentRuleId::SubscribedThread.as_str(),
			true,
			vec![PushCondition::ThreadSubscription {
				subscribed: true,
			}],
			action_list(&[Tweak::Sound("default".into())], true),
		),
	]
}

fn is_default(rule: AnyPushRuleRef<'_>) -> bool {
	match rule {
		AnyPushRuleRef::Override(r)
		| AnyPushRuleRef::Underride(r)
		| AnyPushRuleRef::PostContent(r) => r.default,
		AnyPushRuleRef::Content(r) => r.default,
		AnyPushRuleRef::Room(r) => r.default,
		AnyPushRuleRef::Sender(r) => r.default,
	}
}

fn new_conditional(rule: NewConditionalPushRule) -> ConditionalPushRule {
	ConditionalPushRule {
		actions: rule.actions,
		default: false,
		enabled: true,
		rule_id: rule.rule_id,
		conditions: rule.conditions,
	}
}

fn new_simple<T>(rule: NewSimplePushRule<T>) -> SimplePushRule<T> {
	SimplePushRule {
		actions: rule.actions,
		default: false,
		enabled: true,
		rule_id: rule.rule_id,
	}
}

fn merge_conditional(current: &mut Vec<ConditionalPushRule>, defaults: Vec<ConditionalPushRule>) {
	let mut merged = Vec::with_capacity(current.len());
	let mut remaining = defaults;
	for rule in current.drain(..) {
		if !rule.default {
			merged.push(rule);
			continue;
		}
		if let Some(position) = remaining.iter().position(|d| d.rule_id == rule.rule_id) {
			let mut fresh = remaining.remove(position);
			fresh.enabled = rule.enabled;
			fresh.actions = rule.actions;
			merged.push(fresh);
		}
	}
	merged.extend(remaining);
	*current = merged;
}

struct Matcher<'a> {
	event: &'a Value,
	ctx: &'a PushConditionRoomCtx,
}

/// Splits a dotted key, honouring `\.` and `\\` escapes.
fn key_path(key: &str) -> Vec<String> {
	let mut parts = vec![String::new()];
	let mut chars = key.chars();
	while let Some(c) = chars.next() {
		match c {
			'.' => parts.push(String::new()),
			'\\' => match chars.next() {
				Some(next @ ('.' | '\\')) => {
					if let Some(last) = parts.last_mut() {
						last.push(next);
					}
				}
				Some(other) => {
					if let Some(last) = parts.last_mut() {
						last.push('\\');
						last.push(other);
					}
				}
				None => {
					if let Some(last) = parts.last_mut() {
						last.push('\\');
					}
				}
			},
			other => {
				if let Some(last) = parts.last_mut() {
					last.push(other);
				}
			}
		}
	}
	parts
}

fn is_word(c: char) -> bool {
	c.is_alphanumeric() || c == '_'
}

/// Case-insensitive glob match where `*` matches any run and `?` one char.
fn glob(pattern: &str, text: &str) -> bool {
	let pattern: Vec<char> = pattern.to_lowercase().chars().collect();
	let text: Vec<char> = text.to_lowercase().chars().collect();
	let (mut p, mut t) = (0_usize, 0_usize);
	let mut star: Option<(usize, usize)> = None;
	while let Some(&current) = text.get(t) {
		match pattern.get(p) {
			Some(&c) if c == '?' || c == current => {
				p = p.saturating_add(1);
				t = t.saturating_add(1);
			}
			Some('*') => {
				star = Some((p, t));
				p = p.saturating_add(1);
			}
			_ => {
				let Some((star_p, star_t)) = star else {
					return false;
				};
				let resume = star_t.saturating_add(1);
				p = star_p.saturating_add(1);
				t = resume;
				star = Some((star_p, resume));
			}
		}
	}
	pattern.get(p..).is_some_and(|rest| rest.iter().all(|&c| c == '*'))
}

/// Whether `pattern` matches a whole-word run somewhere in `text`.
fn word_glob(pattern: &str, text: &str) -> bool {
	let chars: Vec<char> = text.chars().collect();
	(0..chars.len()).any(|start| {
		let word_before =
			start.checked_sub(1).and_then(|i| chars.get(i)).copied().is_some_and(is_word);
		!word_before
			&& (start.saturating_add(1)..=chars.len()).any(|end| {
				let word_after = chars.get(end).copied().is_some_and(is_word);
				!word_after && {
					let candidate: String = chars[start..end].iter().collect();
					glob(pattern, &candidate)
				}
			})
	})
}

impl Matcher<'_> {
	fn lookup(&self, key: &str) -> Option<&Value> {
		let mut current = self.event;
		for part in key_path(key) {
			current = current.as_object()?.get(part.as_str())?;
		}
		Some(current)
	}

	fn string(&self, key: &str) -> Option<&str> {
		self.lookup(key)?.as_str()
	}

	fn body_matches(&self, pattern: &str) -> bool {
		self.string("content.body").is_some_and(|body| word_glob(pattern, body))
	}

	fn condition(&self, condition: &PushCondition) -> bool {
		match condition {
			PushCondition::EventMatch {
				key,
				pattern,
			} => {
				if key == "content.body" {
					self.body_matches(pattern)
				} else {
					self.string(key).is_some_and(|text| glob(pattern, text))
				}
			}
			PushCondition::ContainsDisplayName => {
				!self.ctx.user_display_name.is_empty()
					&& self.string("content.body").is_some_and(|body| {
						word_glob(&escape_glob(&self.ctx.user_display_name), body)
					})
			}
			PushCondition::RoomMemberCount {
				is,
			} => member_count_matches(is, self.ctx.member_count),
			PushCondition::SenderNotificationPermission {
				key,
			} => self.sender_may_notify(key),
			PushCondition::EventPropertyIs {
				key,
				value,
			} => self.lookup(key) == Some(value),
			PushCondition::EventPropertyContains {
				key,
				value,
			} => self
				.lookup(key)
				.and_then(Value::as_array)
				.is_some_and(|items| items.contains(value)),
			PushCondition::ThreadSubscription {
				subscribed,
			} => {
				self.string("content.m\\.relates_to.rel_type") == Some("m.thread")
					&& self
						.string("content.m\\.relates_to.event_id")
						.is_some_and(|id| crate::OwnedEventId::parse(id).is_ok())
					&& self.ctx.thread_subscription == Some(*subscribed)
			}
			PushCondition::Unknown(_) => false,
		}
	}

	fn sender_may_notify(&self, key: &str) -> bool {
		let (Some(levels), Some(sender)) = (&self.ctx.power_levels, self.string("sender")) else {
			return false;
		};
		let required = if key == "room" {
			levels.notifications.room
		} else {
			50
		};
		let level = levels
			.users
			.iter()
			.find(|(user, _)| user.as_str() == sender)
			.map_or(levels.users_default, |(_, level)| *level);
		level >= required
	}
}

fn escape_glob(text: &str) -> String {
	text.chars().filter(|c| !matches!(c, '*' | '?')).collect()
}

fn member_count_matches(is: &str, count: UInt) -> bool {
	let (op, number) = ["==", "<=", ">=", "<", ">"]
		.iter()
		.find_map(|op| is.strip_prefix(op).map(|rest| (*op, rest)))
		.unwrap_or(("==", is));
	let Ok(number) = number.parse::<UInt>() else {
		return false;
	};
	match op {
		"<" => count < number,
		">" => count > number,
		"<=" => count <= number,
		">=" => count >= number,
		_ => count == number,
	}
}

impl Serialize for Ruleset {
	fn to_json(&self) -> Value {
		let list = |rules: Vec<Value>| {
			if rules.is_empty() {
				Value::Null
			} else {
				Value::Array(rules)
			}
		};
		Value::Object(object_from(vec![
			("override", list(self.override_.iter().map(Serialize::to_json).collect())),
			("content", list(self.content.iter().map(Serialize::to_json).collect())),
			("room", list(self.room.iter().map(Serialize::to_json).collect())),
			("sender", list(self.sender.iter().map(Serialize::to_json).collect())),
			("underride", list(self.underride.iter().map(Serialize::to_json).collect())),
			("postcontent", list(self.postcontent.iter().map(Serialize::to_json).collect())),
		]))
	}
}

fn list<T: Deserialize>(object: &Object, name: &str) -> Result<Vec<T>, DeError> {
	object.get(name).map_or_else(|| Ok(Vec::new()), Vec::<T>::from_json)
}

impl Deserialize for Ruleset {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("ruleset object"))?;
		Ok(Self {
			override_: list::<_>(object, "override")?.into(),
			content: list::<_>(object, "content")?.into(),
			room: list::<_>(object, "room")?.into(),
			sender: list::<_>(object, "sender")?.into(),
			underride: list::<_>(object, "underride")?.into(),
			postcontent: list::<_>(object, "postcontent")?.into(),
		})
	}
}

impl Deserialize for AnyPushRule {
	/// Guesses the kind from the fields present: `pattern` means a content
	/// rule, `conditions` an override rule, anything else a room rule.
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("rule object"))?;
		Ok(if object.get("pattern").is_some() {
			Self::Content(PatternedPushRule::from_json(value)?)
		} else if object.get("conditions").is_some() {
			Self::Override(ConditionalPushRule::from_json(value)?)
		} else {
			Self::Room(SimplePushRule::from_json(value)?)
		})
	}
}

impl AnyPushRule {
	/// Parses a rule when its kind is known from the endpoint path.
	///
	/// The JSON representation does not contain the rule kind, so callers for
	/// endpoints such as `GET /pushrules/.../{kind}/{rule_id}` must supply it.
	///
	/// # Errors
	///
	/// Returns an error if the response body does not match the requested kind.
	pub fn from_json_with_kind(kind: RuleKind, value: &Value) -> Result<Self, DeError> {
		Ok(match kind {
			RuleKind::Override => Self::Override(ConditionalPushRule::from_json(value)?),
			RuleKind::Underride => Self::Underride(ConditionalPushRule::from_json(value)?),
			RuleKind::PostContent => Self::PostContent(ConditionalPushRule::from_json(value)?),
			RuleKind::Content => Self::Content(PatternedPushRule::from_json(value)?),
			RuleKind::Room => Self::Room(SimplePushRule::from_json(value)?),
			RuleKind::Sender => Self::Sender(SimplePushRule::from_json(value)?),
		})
	}
}

pub mod get_pushrules_all {
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
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/pushrules/");
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
			pub global: crate::push_rules::Ruleset,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"global",
					crate::endpoint::enc(&self.global),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					global: input.body("global")?,
				})
			}
		}
	}
}

pub mod get_pushrules_global_scope {
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
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/pushrules/global/");
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
			pub global: crate::push_rules::Ruleset,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.global)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					global: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	}
}

pub mod get_pushrule {
	pub mod v3 {
		pub struct Request {
			pub scope: crate::push_rules::RuleScope,
			pub kind: crate::push_rules::RuleKind,
			pub rule_id: alloc::string::String,
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
				"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.scope),
					crate::endpoint::path_param(&self.kind),
					crate::endpoint::path_param(&self.rule_id),
				])
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
				let value = Self {
					scope: input.path()?,
					kind: input.path()?,
					rule_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub rule: crate::push_rules::PushRule,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.rule)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					rule: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
		impl Response {
			/// Decodes the response using the rule kind from the request path.
			///
			/// # Errors
			///
			/// Returns an error if the response body does not match the requested kind.
			pub fn from_body_with_kind(
				kind: crate::push_rules::RuleKind,
				body: &crate::json::Value,
			) -> Result<Self, crate::codec::DeError> {
				Ok(Self {
					rule: crate::push_rules::AnyPushRule::from_json_with_kind(kind, body)?,
				})
			}
		}
	}
}

pub mod delete_pushrule {
	pub mod v3 {
		pub struct Request {
			pub scope: crate::push_rules::RuleScope,
			pub kind: crate::push_rules::RuleKind,
			pub rule_id: alloc::string::String,
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
				"DELETE",
				"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.scope),
					crate::endpoint::path_param(&self.kind),
					crate::endpoint::path_param(&self.rule_id),
				])
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
				let value = Self {
					scope: input.path()?,
					kind: input.path()?,
					rule_id: input.path()?,
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
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
		}
	}
}

pub mod get_pushrule_actions {
	pub mod v3 {
		pub struct Request {
			pub scope: crate::push_rules::RuleScope,
			pub kind: crate::push_rules::RuleKind,
			pub rule_id: alloc::string::String,
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
				"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}/actions",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.scope),
					crate::endpoint::path_param(&self.kind),
					crate::endpoint::path_param(&self.rule_id),
				])
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
				let value = Self {
					scope: input.path()?,
					kind: input.path()?,
					rule_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub actions: alloc::vec::Vec<crate::push::Action>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"actions",
					crate::endpoint::enc(&self.actions),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					actions: input.body("actions")?,
				})
			}
		}
	}
}

pub mod set_pushrule_actions {
	pub mod v3 {
		pub struct Request {
			pub scope: crate::push_rules::RuleScope,
			pub kind: crate::push_rules::RuleKind,
			pub rule_id: alloc::string::String,
			pub actions: alloc::vec::Vec<crate::push::Action>,
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
				"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}/actions",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.scope),
					crate::endpoint::path_param(&self.kind),
					crate::endpoint::path_param(&self.rule_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("actions", crate::endpoint::enc(&self.actions))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					scope: input.path()?,
					kind: input.path()?,
					rule_id: input.path()?,
					actions: input.body("actions")?,
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
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
		}
	}
}

pub mod get_pushrule_enabled {
	pub mod v3 {
		pub struct Request {
			pub scope: crate::push_rules::RuleScope,
			pub kind: crate::push_rules::RuleKind,
			pub rule_id: alloc::string::String,
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
				"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}/enabled",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.scope),
					crate::endpoint::path_param(&self.kind),
					crate::endpoint::path_param(&self.rule_id),
				])
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
				let value = Self {
					scope: input.path()?,
					kind: input.path()?,
					rule_id: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub enabled: bool,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"enabled",
					crate::endpoint::enc(&self.enabled),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					enabled: input.body("enabled")?,
				})
			}
		}
	}
}

pub mod set_pushrule_enabled {
	pub mod v3 {
		pub struct Request {
			pub scope: crate::push_rules::RuleScope,
			pub kind: crate::push_rules::RuleKind,
			pub rule_id: alloc::string::String,
			pub enabled: bool,
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
				"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}/enabled",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.scope),
					crate::endpoint::path_param(&self.kind),
					crate::endpoint::path_param(&self.rule_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("enabled", crate::endpoint::enc(&self.enabled))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					scope: input.path()?,
					kind: input.path()?,
					rule_id: input.path()?,
					enabled: input.body("enabled")?,
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
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
		}
	}
}

pub mod set_pushrule {
	pub mod v3 {
		use alloc::{string::String, vec::Vec};

		use crate::{
			codec::{DeError, Deserialize},
			endpoint::{EndpointRequest, EndpointResponse, Input, Metadata},
			json::{Object, Value},
			push_rules::{NewPushRule, RuleKind, RuleScope},
		};

		#[derive(Debug)]
		pub struct Request {
			pub scope: RuleScope,
			pub rule: NewPushRule,
			pub before: Option<String>,
			pub after: Option<String>,
		}

		impl Request {
			#[must_use]
			pub fn new(rule: NewPushRule) -> Self {
				Self {
					scope: RuleScope::Global,
					rule,
					before: None,
					after: None,
				}
			}
		}

		const _: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
			"PUT",
			"/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}",
		);
		impl EndpointRequest for Request {
			type Response = Response;

			const METADATA: Metadata =
				Metadata::new("PUT", "/_matrix/client/v3/pushrules/{scope}/{kind}/{rule_id}");

			fn path_args(&self) -> Vec<String> {
				[
					crate::endpoint::to_param(&self.scope),
					crate::endpoint::to_param(&self.rule.kind()),
					Some(String::from(self.rule.rule_id())),
				]
				.into_iter()
				.map(Option::unwrap_or_default)
				.collect()
			}

			fn query(&self) -> Vec<(String, String)> {
				let mut pairs = Vec::new();
				if let Some(before) = &self.before {
					pairs.push((String::from("before"), before.clone()));
				}
				if let Some(after) = &self.after {
					pairs.push((String::from("after"), after.clone()));
				}
				pairs
			}

			fn body(&self) -> Option<Value> {
				Some(self.rule.body_json())
			}

			fn from_parts(
				path: &[String],
				query: &[(String, String)],
				body: Option<&Value>,
			) -> Result<Self, DeError> {
				let input = Input::new(path, query, body);
				let scope = RuleScope::from_json(&Value::String(
					path.first().cloned().ok_or_else(|| DeError::expected("scope"))?,
				))?;
				let kind = RuleKind::from_json(&Value::String(
					path.get(1).cloned().ok_or_else(|| DeError::expected("kind"))?,
				))?;
				let rule_id = path.get(2).cloned().ok_or_else(|| DeError::expected("rule_id"))?;
				let body = body.ok_or_else(|| DeError::expected("request body"))?;
				let request = Self {
					scope,
					rule: NewPushRule::from_body(kind, rule_id, body)?,
					before: input.query("before")?,
					after: input.query("after")?,
				};
				Ok(request)
			}
		}

		#[derive(Debug, Default)]
		pub struct Response {}

		impl EndpointResponse for Response {
			fn to_body(&self) -> Value {
				Value::Object(Object::new())
			}

			fn from_body(_body: &Value) -> Result<Self, DeError> {
				Ok(Self {})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	fn ctx(members: UInt) -> PushConditionRoomCtx {
		PushConditionRoomCtx {
			room_id: OwnedRoomId::parse("!r:x").unwrap(),
			member_count: members,
			user_id: OwnedUserId::parse("@me:x").unwrap(),
			user_display_name: "Me".into(),
			power_levels: Some(PushConditionPowerLevelsCtx {
				users: BTreeMap::from([(OwnedUserId::parse("@admin:x").unwrap(), 100)]),
				users_default: 0,
				notifications: NotificationPowerLevels::new(),
			}),
			room_version: None,
			thread_subscription: None,
		}
	}

	fn actions(json: &str, members: UInt) -> Vec<Action> {
		let raw = Raw::<AnySyncTimelineEvent>::from_json_text(json).unwrap();
		Ruleset::server_default("@me:x").get_actions(&raw, &ctx(members)).to_vec()
	}

	#[test]
	fn glob_and_word_matching() {
		assert!(glob("m.room.*", "m.room.message"));
		assert!(glob("M.ROOM.?essage", "m.room.message"));
		assert!(!glob("m.room", "m.room.message"));
		assert!(word_glob("hello", "oh, Hello there"));
		assert!(!word_glob("hell", "hello"));
	}

	#[test]
	fn key_paths_honour_escapes() {
		assert_eq!(key_path("content.m\\.mentions.room"), ["content", "m.mentions", "room"]);
	}

	#[test]
	fn plain_message_notifies_quietly() {
		let got = actions(
			r#"{"type":"m.room.message","sender":"@a:x","content":{"body":"hi","msgtype":"m.text"}}"#,
			5,
		);
		assert_eq!(got, [Action::Notify, Action::SetTweak(Tweak::Highlight(false))]);
	}

	#[test]
	fn mention_highlights_and_notices_are_silent() {
		let mention = actions(
			r#"{"type":"m.room.message","sender":"@a:x","content":{"body":"x","m.mentions":{"user_ids":["@me:x"]}}}"#,
			5,
		);
		assert!(mention.contains(&Action::SetTweak(Tweak::Highlight(true))));
		let notice = actions(
			r#"{"type":"m.room.message","sender":"@a:x","content":{"body":"x","msgtype":"m.notice"}}"#,
			5,
		);
		assert!(notice.is_empty());
	}

	#[test]
	fn room_mention_needs_power() {
		let json = |sender| {
			alloc::format!(
				r#"{{"type":"m.room.message","sender":"{sender}","content":{{"body":"x","m.mentions":{{"room":true}}}}}}"#
			)
		};
		assert!(
			actions(&json("@admin:x"), 5).contains(&Action::SetTweak(Tweak::Highlight(true)))
		);
		assert!(
			!actions(&json("@rando:x"), 5).contains(&Action::SetTweak(Tweak::Highlight(true)))
		);
	}

	#[test]
	fn one_to_one_rule_uses_member_count() {
		let json = r#"{"type":"m.room.message","sender":"@a:x","content":{"body":"x"}}"#;
		assert!(actions(json, 2).contains(&Action::SetTweak(Tweak::Sound("default".into()))));
		assert!(!actions(json, 3).contains(&Action::SetTweak(Tweak::Sound("default".into()))));
	}

	#[test]
	fn server_default_includes_the_msc3930_poll_rules() {
		let rules = Ruleset::server_default("@u:example.org");
		assert!(
			rules.get(RuleKind::Override, ".org.matrix.msc3930.rule.poll_response").is_some()
		);
		assert!(rules.get(RuleKind::Override, ".m.rule.poll_response").is_some());
		for id in [
			".org.matrix.msc3930.rule.poll_start_one_to_one",
			".org.matrix.msc3930.rule.poll_start",
			".org.matrix.msc3930.rule.poll_end_one_to_one",
			".org.matrix.msc3930.rule.poll_end",
			".m.rule.poll_start_one_to_one",
			".m.rule.poll_start",
			".m.rule.poll_end_one_to_one",
			".m.rule.poll_end",
		] {
			assert!(rules.get(RuleKind::Underride, id).is_some(), "{id}");
		}
	}

	#[test]
	fn stable_and_unstable_poll_start_notify() {
		let unstable = actions(
			r#"{"type":"org.matrix.msc3381.poll.start","sender":"@a:x","content":{}}"#,
			5,
		);
		assert_eq!(unstable, [Action::Notify]);
		let stable = actions(r#"{"type":"m.poll.start","sender":"@a:x","content":{}}"#, 5);
		assert_eq!(stable, [Action::Notify]);
		let unstable_1to1 = actions(
			r#"{"type":"org.matrix.msc3381.poll.start","sender":"@a:x","content":{}}"#,
			2,
		);
		assert_eq!(
			unstable_1to1,
			[Action::Notify, Action::SetTweak(Tweak::Sound("default".into()))]
		);
		let stable_1to1 = actions(r#"{"type":"m.poll.start","sender":"@a:x","content":{}}"#, 2);
		assert_eq!(
			stable_1to1,
			[Action::Notify, Action::SetTweak(Tweak::Sound("default".into()))]
		);
	}

	#[test]
	fn stable_and_unstable_poll_end_notify() {
		let unstable = actions(
			r#"{"type":"org.matrix.msc3381.poll.end","sender":"@a:x","content":{}}"#,
			5,
		);
		assert_eq!(unstable, [Action::Notify]);
		let stable = actions(r#"{"type":"m.poll.end","sender":"@a:x","content":{}}"#, 5);
		assert_eq!(stable, [Action::Notify]);
		let unstable_1to1 = actions(
			r#"{"type":"org.matrix.msc3381.poll.end","sender":"@a:x","content":{}}"#,
			2,
		);
		assert_eq!(
			unstable_1to1,
			[Action::Notify, Action::SetTweak(Tweak::Sound("default".into()))]
		);
		let stable_1to1 = actions(r#"{"type":"m.poll.end","sender":"@a:x","content":{}}"#, 2);
		assert_eq!(
			stable_1to1,
			[Action::Notify, Action::SetTweak(Tweak::Sound("default".into()))]
		);
	}

	#[test]
	fn stable_and_unstable_poll_response_are_silent() {
		let unstable = actions(
			r#"{"type":"org.matrix.msc3381.poll.response","sender":"@a:x","content":{}}"#,
			5,
		);
		assert!(unstable.is_empty());
		let stable = actions(r#"{"type":"m.poll.response","sender":"@a:x","content":{}}"#, 5);
		assert!(stable.is_empty());
	}

	#[test]
	fn server_default_includes_postcontent_thread_rules() {
		let rules = Ruleset::server_default("@u:example.org");
		let unsubscribed = rules
			.postcontent
			.get(PredefinedPostContentRuleId::UnsubscribedThread.as_str())
			.unwrap();
		assert_eq!(
			unsubscribed.conditions,
			[PushCondition::ThreadSubscription {
				subscribed: false
			}]
		);
		assert!(unsubscribed.actions.is_empty());
		let subscribed = rules
			.postcontent
			.get(PredefinedPostContentRuleId::SubscribedThread.as_str())
			.unwrap();
		assert_eq!(
			subscribed.conditions,
			[PushCondition::ThreadSubscription {
				subscribed: true
			}]
		);
		assert_eq!(
			subscribed.actions,
			[Action::Notify, Action::SetTweak(Tweak::Sound("default".into()))]
		);
	}

	#[test]
	fn postcontent_rules_round_trip_and_mutate_independently() {
		let rules = Ruleset::server_default("@u:example.org");
		let json = rules.to_json();
		assert!(json.get("postcontent").is_some());
		let parsed = Ruleset::from_json(&json).unwrap();
		assert!(
			parsed
				.get(RuleKind::PostContent, PredefinedPostContentRuleId::SubscribedThread)
				.is_some()
		);

		let mut custom = Ruleset::default();
		custom.postcontent.insert(ConditionalPushRule {
			actions: Vec::new(),
			default: false,
			enabled: true,
			rule_id: "custom".into(),
			conditions: Vec::new(),
		});
		custom.set_enabled(RuleKind::PostContent, "custom", false).unwrap();
		assert!(!custom.get(RuleKind::PostContent, "custom").unwrap().enabled());
		assert!(custom.remove(RuleKind::PostContent, "custom").is_ok());
	}

	#[test]
	fn thread_subscription_conditions_match_only_valid_thread_state() {
		let event = crate::json::Value::parse(
			r#"{"type":"m.room.message","content":{"m.relates_to":{"rel_type":"m.thread","event_id":"$root:example.org"}}}"#,
		)
		.unwrap();
		let rules = Ruleset::server_default("@u:example.org");
		let unsubscribed = &rules.postcontent[0].conditions[0];
		let subscribed = &rules.postcontent[1].conditions[0];

		for (state, expected_unsubscribed, expected_subscribed) in
			[(Some(false), true, false), (Some(true), false, true), (None, false, false)]
		{
			let mut context = ctx(2);
			context.thread_subscription = state;
			let matcher = Matcher {
				event: &event,
				ctx: &context,
			};
			assert_eq!(matcher.condition(unsubscribed), expected_unsubscribed);
			assert_eq!(matcher.condition(subscribed), expected_subscribed);
		}

		let malformed = crate::json::Value::parse(
			r#"{"type":"m.room.message","content":{"m.relates_to":{"rel_type":"m.thread","event_id":"not-an-event-id"}}}"#,
		)
		.unwrap();
		let mut context = ctx(2);
		context.thread_subscription = Some(true);
		let matcher = Matcher {
			event: &malformed,
			ctx: &context,
		};
		assert!(!matcher.condition(subscribed));
	}

	#[test]
	fn insert_orders_and_validates() {
		let mut rules = Ruleset::new();
		let simple = |id: &str| NewSimplePushRule {
			rule_id: OwnedRoomId::from_trusted(id),
			actions: Vec::new(),
		};
		rules.insert(NewPushRule::Room(simple("!abc:example.org")), None, None).unwrap();
		rules.insert(NewPushRule::Room(simple("!def:example.org")), None, None).unwrap();
		assert_eq!(rules.ids(RuleKind::Room), ["!def:example.org", "!abc:example.org"]);
		rules
			.insert(NewPushRule::Room(simple("!def:example.org")), Some("!abc:example.org"), None)
			.unwrap();
		assert_eq!(rules.ids(RuleKind::Room), ["!abc:example.org", "!def:example.org"]);
		assert_eq!(
			rules.insert(NewPushRule::Room(simple(".m.x")), None, None),
			Err(InsertPushRuleError::ServerDefaultRuleId)
		);
		assert_eq!(
			rules.insert(NewPushRule::Room(simple("!c:x")), Some("nope"), None),
			Err(InsertPushRuleError::UnknownRuleId)
		);
	}

	#[test]
	fn defaults_cannot_be_removed_but_can_be_disabled() {
		let mut rules = Ruleset::server_default("@me:x");
		let id = PredefinedOverrideRuleId::Master;
		assert_eq!(rules.remove(RuleKind::Override, id), Err(RemovePushRuleError::ServerDefault));
		rules.set_enabled(RuleKind::Override, id, true).unwrap();
		assert!(rules.get(RuleKind::Override, id).unwrap().enabled());
		assert_eq!(rules.remove(RuleKind::Room, "!x"), Err(RemovePushRuleError::NotFound));
	}

	#[test]
	fn update_keeps_user_choices_and_adds_missing() {
		let mut rules = Ruleset::server_default("@me:x");
		rules.override_.retain(|r| r.rule_id != ".m.rule.reaction");
		rules.set_enabled(RuleKind::Override, PredefinedOverrideRuleId::Master, true).unwrap();
		rules.update_with_server_default(Ruleset::server_default("@me:x"));
		assert!(rules.get(RuleKind::Override, PredefinedOverrideRuleId::Reaction).is_some());
		assert!(
			rules.get(RuleKind::Override, PredefinedOverrideRuleId::Master).unwrap().enabled()
		);
	}

	#[test]
	fn ruleset_round_trips() {
		let rules = Ruleset::server_default("@me:x");
		assert_eq!(from_str::<Ruleset>(&to_string(&rules)).unwrap(), rules);
	}
}

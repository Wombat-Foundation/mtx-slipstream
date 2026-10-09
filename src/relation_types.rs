//! Event relations (`m.relates_to`) and their bundled aggregations.

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
	OwnedEventId, UInt,
	codec::{DeError, Deserialize, Serialize},
	endpoint::{Input, object_from},
	json::{Object, Value},
	sswire::Raw,
};

/// The event a reply points at.
#[derive(Debug, Eq, PartialEq)]
pub struct InReplyTo {
	pub event_id: OwnedEventId,
}

impl InReplyTo {
	#[must_use]
	pub fn new(event_id: OwnedEventId) -> Self {
		Self {
			event_id,
		}
	}
}

impl crate::codec::Serialize for InReplyTo {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(event_id),
			crate::endpoint::enc(&self.event_id),
		)])
	}
}
impl crate::codec::Deserialize for InReplyTo {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(InReplyTo)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			event_id: input.body(stringify!(event_id))?,
		})
	}
}

/// A relation placing an event in a thread.
#[derive(Debug, Eq, PartialEq)]
pub struct Thread {
	/// The root of the thread.
	pub event_id: OwnedEventId,
	/// The latest event of the thread, for clients without thread support.
	pub in_reply_to: Option<InReplyTo>,
	pub is_falling_back: bool,
}

impl Thread {
	#[must_use]
	pub fn plain(event_id: OwnedEventId, latest: OwnedEventId) -> Self {
		Self {
			event_id,
			in_reply_to: Some(InReplyTo::new(latest)),
			is_falling_back: true,
		}
	}
}

impl crate::codec::Serialize for Thread {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(event_id), crate::endpoint::enc(&self.event_id)),
			(stringify!(in_reply_to), crate::endpoint::enc(&self.in_reply_to)),
			(stringify!(is_falling_back), crate::endpoint::enc(&self.is_falling_back)),
		])
	}
}
impl crate::codec::Deserialize for Thread {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Thread)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			event_id: input.body(stringify!(event_id))?,
			in_reply_to: input.body_or_default(stringify!(in_reply_to))?,
			is_falling_back: input.body_or_default(stringify!(is_falling_back))?,
		})
	}
}

/// Replaces (edits) another event.
#[derive(Debug, Eq, PartialEq)]
pub struct Replacement {
	pub event_id: OwnedEventId,
}

impl crate::codec::Serialize for Replacement {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(event_id),
			crate::endpoint::enc(&self.event_id),
		)])
	}
}
impl crate::codec::Deserialize for Replacement {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Replacement)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			event_id: input.body(stringify!(event_id))?,
		})
	}
}

/// References another event without changing how it is shown.
#[derive(Debug, Eq, PartialEq)]
pub struct Reference {
	pub event_id: OwnedEventId,
}

impl crate::codec::Serialize for Reference {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(event_id),
			crate::endpoint::enc(&self.event_id),
		)])
	}
}
impl crate::codec::Deserialize for Reference {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Reference)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			event_id: input.body(stringify!(event_id))?,
		})
	}
}

/// An annotation, such as a reaction, on another event.
#[derive(Debug, Eq, PartialEq)]
pub struct Annotation {
	pub event_id: OwnedEventId,
	pub key: String,
}

impl crate::codec::Serialize for Annotation {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(event_id), crate::endpoint::enc(&self.event_id)),
			(stringify!(key), crate::endpoint::enc(&self.key)),
		])
	}
}
impl crate::codec::Deserialize for Annotation {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(Annotation)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			event_id: input.body(stringify!(event_id))?,
			key: input.body(stringify!(key))?,
		})
	}
}

/// A relation of a kind this server does not know.
#[derive(Debug, Eq, PartialEq)]
pub struct CustomRelation {
	pub rel_type: String,
	pub data: Object,
}

/// How an event relates to another.
#[derive(Debug, Eq, PartialEq)]
pub enum Relation {
	Reply {
		in_reply_to: InReplyTo,
	},
	Replacement(Replacement),
	Thread(Thread),
	Reference(Reference),
	Annotation(Annotation),
	_Custom(CustomRelation),
}

impl Serialize for Relation {
	fn to_json(&self) -> Value {
		let (rel_type, body) = match self {
			Self::Reply {
				in_reply_to,
			} => {
				return Value::Object(object_from(alloc::vec![(
					"m.in_reply_to",
					in_reply_to.to_json()
				)]));
			}
			Self::Replacement(r) => ("m.replace", r.to_json()),
			Self::Thread(r) => ("m.thread", r.to_json()),
			Self::Reference(r) => ("m.reference", r.to_json()),
			Self::Annotation(r) => ("m.annotation", r.to_json()),
			Self::_Custom(r) => (r.rel_type.as_str(), Value::Object(r.data.clone())),
		};
		let mut object = body.as_object().cloned().unwrap_or_default();
		object.insert("rel_type".into(), Value::String(rel_type.into()));
		if matches!(self, Self::Thread(_))
			&& let Some(Value::Object(reply)) = object.remove("in_reply_to")
		{
			object.insert("m.in_reply_to".into(), Value::Object(reply));
		}
		Value::Object(object)
	}
}

impl Deserialize for Relation {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("relation object"))?;
		let Some(rel_type) = object.get("rel_type").and_then(Value::as_str) else {
			let reply = object
				.get("m.in_reply_to")
				.ok_or_else(|| DeError::expected("rel_type or m.in_reply_to"))?;
			return Ok(Self::Reply {
				in_reply_to: InReplyTo::from_json(reply)?,
			});
		};
		match rel_type {
			"m.thread" => {
				let mut thread = object.clone();
				if let Some(reply) = thread.remove("m.in_reply_to") {
					thread.insert("in_reply_to".into(), reply);
				}
				Thread::from_json(&Value::Object(thread)).map(Self::Thread)
			}
			"m.replace" => Replacement::from_json(value).map(Self::Replacement),
			"m.reference" => Reference::from_json(value).map(Self::Reference),
			"m.annotation" => Annotation::from_json(value).map(Self::Annotation),
			other => Ok(Self::_Custom(CustomRelation {
				rel_type: other.into(),
				data: object.clone(),
			})),
		}
	}
}

/// Aggregated thread information bundled into a thread root.
#[derive(Debug)]
pub struct BundledThread {
	pub latest_event: Raw<Value>,
	pub count: UInt,
	pub current_user_participated: bool,
}

impl crate::codec::Serialize for BundledThread {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(latest_event), crate::endpoint::enc(&self.latest_event)),
			(stringify!(count), crate::endpoint::enc(&self.count)),
			(
				stringify!(current_user_participated),
				crate::endpoint::enc(&self.current_user_participated),
			),
		])
	}
}
impl crate::codec::Deserialize for BundledThread {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(BundledThread)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			latest_event: input.body(stringify!(latest_event))?,
			count: input.body(stringify!(count))?,
			current_user_participated: input
				.body_or_default(stringify!(current_user_participated))?,
		})
	}
}

/// An event that references the bundling event.
#[derive(Debug, Eq, PartialEq)]
pub struct BundledReference {
	pub event_id: OwnedEventId,
}

impl BundledReference {
	#[must_use]
	pub fn new(event_id: OwnedEventId) -> Self {
		Self {
			event_id,
		}
	}
}

impl crate::codec::Serialize for BundledReference {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(event_id),
			crate::endpoint::enc(&self.event_id),
		)])
	}
}
impl crate::codec::Deserialize for BundledReference {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(BundledReference)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			event_id: input.body(stringify!(event_id))?,
		})
	}
}

/// All references to the bundling event.
#[derive(Debug, Eq, PartialEq)]
pub struct ReferenceChunk {
	pub chunk: Vec<BundledReference>,
}

impl ReferenceChunk {
	#[must_use]
	pub fn new(chunk: Vec<BundledReference>) -> Self {
		Self {
			chunk,
		}
	}
}

impl crate::codec::Serialize for ReferenceChunk {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(chunk),
			crate::endpoint::enc(&self.chunk),
		)])
	}
}
impl crate::codec::Deserialize for ReferenceChunk {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(ReferenceChunk)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			chunk: input.body(stringify!(chunk))?,
		})
	}
}

/// The aggregations bundled into an event's `unsigned.m.relations`.
///
/// `T` is the representation of the replacing event.
#[derive(Debug)]
pub struct BundledMessageLikeRelations<T> {
	pub replace: Option<Box<T>>,
	pub has_invalid_replacement: bool,
	pub thread: Option<Box<BundledThread>>,
	pub reference: Option<Box<ReferenceChunk>>,
}

impl<T> Default for BundledMessageLikeRelations<T> {
	fn default() -> Self {
		Self {
			replace: None,
			has_invalid_replacement: false,
			thread: None,
			reference: None,
		}
	}
}

impl<T> BundledMessageLikeRelations<T> {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Whether nothing is bundled.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.replace.is_none() && self.thread.is_none() && self.reference.is_none()
	}
}

impl<T: Serialize> Serialize for BundledMessageLikeRelations<T> {
	fn to_json(&self) -> Value {
		let mut fields = Vec::new();
		if let Some(replace) = &self.replace {
			fields.push(("m.replace", replace.to_json()));
		}
		if let Some(thread) = &self.thread {
			fields.push(("m.thread", thread.to_json()));
		}
		if let Some(reference) = &self.reference {
			fields.push(("m.reference", reference.to_json()));
		}
		Value::Object(object_from(fields))
	}
}

impl<T: Deserialize> Deserialize for BundledMessageLikeRelations<T> {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let input = Input::new(&[], &[], Some(value));
		Ok(Self {
			replace: input.body::<Option<T>>("m.replace")?.map(Box::new),
			has_invalid_replacement: false,
			thread: input.body::<Option<BundledThread>>("m.thread")?.map(Box::new),
			reference: input.body::<Option<ReferenceChunk>>("m.reference")?.map(Box::new),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn reply_has_no_rel_type() {
		let relation: Relation = from_str(r#"{"m.in_reply_to":{"event_id":"$a"}}"#).unwrap();
		assert!(matches!(relation, Relation::Reply { .. }));
		assert!(!to_string(&relation).contains("rel_type"));
	}

	#[test]
	fn thread_round_trips_with_fallback_reply() {
		let text = r#"{"rel_type":"m.thread","event_id":"$root","m.in_reply_to":{"event_id":"$last"},"is_falling_back":true}"#;
		let relation: Relation = from_str(text).unwrap();
		let Relation::Thread(thread) = &relation else {
			panic!("not a thread")
		};
		assert_eq!(thread.event_id.as_str(), "$root");
		assert!(thread.is_falling_back);
		assert_eq!(from_str::<Relation>(&to_string(&relation)).unwrap(), relation);
	}

	#[test]
	fn unknown_relation_is_kept() {
		let relation: Relation = from_str(r#"{"rel_type":"org.x","event_id":"$a"}"#).unwrap();
		assert!(matches!(relation, Relation::_Custom(_)));
	}
}

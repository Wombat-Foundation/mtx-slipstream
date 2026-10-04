//! MSC4500 state hashes carried in `/send` transactions.

use alloc::{collections::BTreeMap, string::String};

use crate::{
	OwnedEventId,
	codec::{DeError, Deserialize, Serialize},
	json::{Object, Value},
};

/// Algorithm identifier for the primary and redaction digests.
pub const ALGORITHM: &str = "lthash16-blake3-v1+redactions-blake3-v1";

/// Algorithm identifier that additionally commits the resolution-input set.
pub const ALGORITHM_WITH_INPUTS: &str =
	"lthash16-blake3-v1+redactions-blake3-v1+resolution-inputs-blake3-v1";

/// The `state_hashes` object of a `/send` transaction.
///
/// One `algorithm` governs every entry; `entries` is keyed by PDU ID.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StateHashes {
	pub algorithm: String,
	pub entries: BTreeMap<OwnedEventId, StateHashEntry>,
}

impl StateHashes {
	/// Whether this server understands the transaction's algorithm. An unknown
	/// algorithm defers validation of the whole transaction.
	#[must_use]
	pub fn is_known_algorithm(&self) -> bool {
		self.algorithm == ALGORITHM || self.algorithm == ALGORITHM_WITH_INPUTS
	}

	/// Whether entries may carry a resolution-input digest.
	#[must_use]
	pub fn has_resolution_inputs(&self) -> bool {
		self.algorithm == ALGORITHM_WITH_INPUTS
	}
}

/// One PDU's assertion.
///
/// A `limited` entry serializes `before` and `redactions_before` as `null` and
/// omits both `after` fields. `resolution_inputs_before` is omitted under the
/// base algorithm; under the input algorithm it is a string, or `null` when
/// the sender has no assertion for that component.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StateHashEntry {
	pub before: Option<String>,
	pub after: Option<String>,
	pub redactions_before: Option<String>,
	pub redactions_after: Option<String>,
	pub resolution_inputs_before: Option<Option<String>>,
	pub limited: bool,
}

impl StateHashEntry {
	/// An explicit deferral: the sender cannot resolve this DAG point.
	#[must_use]
	pub fn limited(with_inputs: bool) -> Self {
		Self {
			limited: true,
			resolution_inputs_before: with_inputs.then_some(None),
			..Self::default()
		}
	}
}

impl Serialize for StateHashEntry {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("before".into(), self.before.to_json());
		object.insert("redactions_before".into(), self.redactions_before.to_json());
		if let Some(after) = &self.after {
			object.insert("after".into(), Value::String(after.clone()));
		}
		if let Some(after) = &self.redactions_after {
			object.insert("redactions_after".into(), Value::String(after.clone()));
		}
		if let Some(inputs) = &self.resolution_inputs_before {
			object.insert("resolution_inputs_before".into(), inputs.to_json());
		}
		if self.limited {
			object.insert("limited".into(), Value::Bool(true));
		}
		Value::Object(object)
	}
}

impl Deserialize for StateHashEntry {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("state hash entry"))?;
		let text = |name: &str| {
			object.get(name).filter(|v| !v.is_null()).map(String::from_json).transpose()
		};
		Ok(Self {
			before: text("before")?,
			after: text("after")?,
			redactions_before: text("redactions_before")?,
			redactions_after: text("redactions_after")?,
			resolution_inputs_before: object
				.get("resolution_inputs_before")
				.map(Option::<String>::from_json)
				.transpose()?,
			limited: object.get("limited").and_then(Value::as_bool).unwrap_or(false),
		})
	}
}

impl Serialize for StateHashes {
	fn to_json(&self) -> Value {
		let mut object = Object::new();
		object.insert("algorithm".into(), Value::String(self.algorithm.clone()));
		object.insert("entries".into(), self.entries.to_json());
		Value::Object(object)
	}
}

impl Deserialize for StateHashes {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("state hashes"))?;
		Ok(Self {
			algorithm: object
				.get("algorithm")
				.map(String::from_json)
				.transpose()?
				.ok_or_else(|| DeError::expected("algorithm"))?,
			entries: object
				.get("entries")
				.map(Deserialize::from_json)
				.transpose()?
				.ok_or_else(|| DeError::expected("entries"))?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn limited_entry_nulls_before_and_omits_after() {
		let entry = StateHashEntry::limited(true);
		assert_eq!(
			to_string(&entry),
			r#"{"before":null,"limited":true,"redactions_before":null,"resolution_inputs_before":null}"#
		);
		assert_eq!(from_str::<StateHashEntry>(&to_string(&entry)).unwrap(), entry);
	}

	#[test]
	fn transaction_object_round_trips() {
		let mut hashes = StateHashes {
			algorithm: ALGORITHM.into(),
			entries: BTreeMap::new(),
		};
		hashes.entries.insert(
			OwnedEventId::from("$e"),
			StateHashEntry {
				before: Some("b".into()),
				after: Some("a".into()),
				redactions_before: Some("rb".into()),
				redactions_after: Some("ra".into()),
				..StateHashEntry::default()
			},
		);
		let again = from_str::<StateHashes>(&to_string(&hashes)).unwrap();
		assert_eq!(again, hashes);
		assert!(again.is_known_algorithm() && !again.has_resolution_inputs());
	}
}

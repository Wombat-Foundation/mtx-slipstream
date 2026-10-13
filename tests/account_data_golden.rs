//! Golden fixtures for stored account data.
//!
//! Each fixture is the literal JSON the database holds in
//! `roomuserdataid_accountdata`: the event envelope (`type`, `content`) in
//! canonical (sorted, compact) form. Decoding then re-encoding must reproduce
//! it exactly, which pins field names, omitted/default fields and shapes
//! against what earlier (ruma/serde) versions wrote.

use mtx_slipstream::{
	codec::{Deserialize, Serialize, from_str, to_string},
	events::{direct::DirectEvent, ignored_user_list::IgnoredUserListEvent, tag::TagEvent},
	json::Value,
};

fn golden<T: Deserialize + Serialize>(fixture: &str) -> T {
	let event: T = from_str(fixture).unwrap_or_else(|e| panic!("{fixture}: {e:?}"));
	assert_eq!(to_string(&event), fixture, "re-encoding must reproduce the stored bytes");
	event
}

#[test]
fn direct_event() {
	let event: DirectEvent = golden(concat!(
		r#"{"content":{"@bob:example.org":["!a:example.org","!b:example.org"],"#,
		r#""@carol:example.org":[]},"type":"m.direct"}"#,
	));

	assert_eq!(event.content.0.len(), 2);
	let lens: Vec<usize> = event.content.0.values().map(Vec::len).collect();
	assert_eq!(lens, [2, 0]);
}

#[test]
fn tag_event_numeric_order() {
	let event: TagEvent = golden(concat!(
		r#"{"content":{"tags":{"m.favourite":{"order":0.25},"u.custom":{}}},"#,
		r#""type":"m.tag"}"#,
	));

	assert_eq!(event.content.tags[&"m.favourite".into()].order, Some(0.25));
	assert_eq!(event.content.tags[&"u.custom".into()].order, None);
}

/// Older clients wrote `order` as a string. The previous build enabled ruma's
/// `compat` features, whose `compat-tag-info` decodes it; this pins whatever the
/// codec does today so a change is a decision.
#[test]
fn tag_event_legacy_string_order() {
	let text = r#"{"content":{"tags":{"u.old":{"order":"0.5"}}},"type":"m.tag"}"#;
	let event = from_str::<TagEvent>(text).expect("legacy string order decodes");

	assert_eq!(event.content.tags[&"u.old".into()].order, Some(0.5));
	// Re-encoding normalises it to the spec's numeric form.
	assert_eq!(
		to_string(&event),
		r#"{"content":{"tags":{"u.old":{"order":0.5}}},"type":"m.tag"}"#
	);
}

/// Unknown fields inside a tag are not part of `TagInfo` (ruma's didn't keep them
/// either: it has only `order`), so they are intentionally discarded.
#[test]
fn tag_event_unknown_fields_are_discarded() {
	let text = r#"{"content":{"tags":{"u.x":{"order":0.5,"extra":true}}},"type":"m.tag"}"#;
	let event = from_str::<TagEvent>(text).expect("unknown tag field is ignored");

	assert_eq!(to_string(&event), r#"{"content":{"tags":{"u.x":{"order":0.5}}},"type":"m.tag"}"#);
}

#[test]
fn ignored_user_list() {
	let event: IgnoredUserListEvent = golden(concat!(
		r#"{"content":{"ignored_users":{"@spam:example.org":{}}},"#,
		r#""type":"m.ignored_user_list"}"#,
	));

	assert_eq!(event.content.ignored_users.len(), 1);
}

/// The spec defines no fields for an ignored user; whatever a client wrote is
/// kept (ruma flattened them into `IgnoredUser`).
#[test]
fn ignored_user_extra_fields_are_kept() {
	golden::<IgnoredUserListEvent>(concat!(
		r#"{"content":{"ignored_users":{"@spam:example.org":{"reason":"noise"}}},"#,
		r#""type":"m.ignored_user_list"}"#,
	));
}

/// `AccountData::delete` stores `{"type":..,"content":{}}` as a tombstone.
/// `m.direct` is a bare map, so an empty content decodes as an empty value. For
/// `m.tag` and `m.ignored_user_list` the previous (ruma) types had a required
/// map field with no default, so an empty content was an error there too; this
/// pins that parity (callers already treat a decode error as "no data").
#[test]
fn tombstone_content_decoding_matches_ruma() {
	let direct: DirectEvent = golden(r#"{"content":{},"type":"m.direct"}"#);
	assert!(direct.content.0.is_empty());

	assert!(from_str::<TagEvent>(r#"{"content":{},"type":"m.tag"}"#).is_err());
	assert!(
		from_str::<IgnoredUserListEvent>(r#"{"content":{},"type":"m.ignored_user_list"}"#)
			.is_err()
	);
}

/// Push rules are an opaque JSON object to the API layer; the stored text must
/// parse and print back unchanged.
#[test]
fn push_rules_object_is_preserved_verbatim() {
	let fixture = concat!(
		r#"{"content":{"global":{"override":[{"actions":["dont_notify"],"default":true,"#,
		r#""enabled":false,"rule_id":".m.rule.master"}],"underride":[]}},"type":"m.push_rules"}"#,
	);
	let value = Value::parse(fixture).expect("parses");
	assert_eq!(to_string(&value), fixture);
}

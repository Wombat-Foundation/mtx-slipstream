use mtx_slipstream::{
	OwnedEventId,
	codec::{from_str, to_string},
	events::room::redaction::RoomRedactionEventContent,
};

#[test]
fn redaction_reason_is_optional_and_round_trips() {
	let without = RoomRedactionEventContent {
		redacts: None,
		reason: None,
	};
	assert_eq!(to_string(&without), "{}");
	let with = RoomRedactionEventContent {
		redacts: Some(OwnedEventId::from("$redact:example.org")),
		reason: Some("because".into()),
	};
	let json = r#"{"reason":"because","redacts":"$redact:example.org"}"#;
	assert_eq!(to_string(&with), json);
	assert_eq!(to_string(&from_str::<RoomRedactionEventContent>(json).unwrap()), json);
}

#[test]
fn fully_read_event_has_the_ruma_envelope() {
	let json = r#"{"content":{"event_id":"$event:example.org"},"type":"m.fully_read"}"#;
	let event: mtx_slipstream::events::fully_read::FullyReadEvent = from_str(json).unwrap();
	assert_eq!(to_string(&event), json);
}

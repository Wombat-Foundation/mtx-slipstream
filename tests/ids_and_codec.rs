#[test]
fn matrix_ids_parse_via_from_str() {
	let id: mtx_slipstream::OwnedUserId = "@a:example.org".parse().unwrap();
	assert_eq!(id.as_str(), "@a:example.org");
}

#[test]
fn ids_convert_into_string_and_triples_round_trip() {
	use mtx_slipstream::codec::{from_str, to_string};
	let id = mtx_slipstream::OwnedEventId::parse("$e").unwrap();
	assert_eq!(String::from(&id), "$e");
	assert_eq!(String::from(id), "$e");
	let triple = (1_u64, "x".to_owned(), 3_u64);
	let back: (u64, String, u64) = from_str(&to_string(&triple)).unwrap();
	assert_eq!(back, triple);
}

#[test]
fn room_version_parse() {
	use mtx_slipstream::RoomVersionId;
	assert_eq!(RoomVersionId::parse("10").unwrap(), RoomVersionId::V10);
	assert_eq!(RoomVersionId::parse("custom-1").unwrap().as_str(), "custom-1");
}

#[test]
fn id_accessors() {
	use mtx_slipstream::{OwnedRoomAliasId, OwnedRoomId, OwnedServerName};
	let alias = OwnedRoomAliasId::parse("#room:example.org").unwrap();
	assert_eq!(alias.alias(), "room");
	let room = OwnedRoomId::parse("!abc:example.org").unwrap();
	assert_eq!(room.localpart(), "abc");
	let plain = OwnedServerName::parse("example.org").unwrap();
	assert_eq!((plain.host(), plain.port(), plain.is_ip_literal()), ("example.org", None, false));
	let with_port = OwnedServerName::parse("example.org:8448").unwrap();
	assert_eq!((with_port.host(), with_port.port()), ("example.org", Some(8448)));
	let v6 = OwnedServerName::parse("[::1]:8448").unwrap();
	assert_eq!((v6.host(), v6.port(), v6.is_ip_literal()), ("[::1]", Some(8448), true));
	let v4 = OwnedServerName::parse("10.0.0.1:1").unwrap();
	assert!(v4.is_ip_literal());
}

#[derive(Debug, PartialEq)]
struct Sample {
	kind: String,
	sender: mtx_slipstream::OwnedUserId,
	redacts: Option<mtx_slipstream::OwnedEventId>,
	tags: Vec<String>,
}

mtx_slipstream::codec_struct!(Sample {
	kind: String = ("type"),
	sender: mtx_slipstream::OwnedUserId = ("sender"),
	redacts: Option<mtx_slipstream::OwnedEventId> = ("redacts", omit),
	tags: Vec<String> = ("tags", omit),
});

#[test]
fn codec_struct_renames_omits_and_ignores_unknown_keys() {
	use mtx_slipstream::codec::{from_str, to_string};
	let sample = Sample {
		kind: "m.room.message".into(),
		sender: mtx_slipstream::OwnedUserId::parse("@a:example.org").unwrap(),
		redacts: None,
		tags: vec![],
	};
	assert_eq!(
		to_string(&sample),
		r#"{"sender":"@a:example.org","tags":[],"type":"m.room.message"}"#
	);
	let parsed: Sample =
		from_str(r#"{"type":"m.room.message","sender":"@a:example.org","extra":1}"#).unwrap();
	assert_eq!(parsed, sample);
	assert!(from_str::<Sample>(r#"{"sender":"@a:example.org"}"#).is_err());
}

#[test]
fn error_codes_round_trip_for_every_named_kind() {
	use mtx_slipstream::api::client::error::ErrorKind;
	for code in [
		"M_THREEPID_IN_USE",
		"M_THREEPID_DENIED",
		"M_THREEPID_AUTH_FAILED",
		"M_INVITE_BLOCKED",
		"M_NOT_YET_UPLOADED",
		"M_CANNOT_OVERWRITE_MEDIA",
		"M_FEATURE_DISABLED",
		"UK.TIMEDOUT.MSC4406.SENDER_IGNORED",
		"M_WRONG_ROOM_KEYS_VERSION",
		"M_EXCLUSIVE",
	] {
		assert_eq!(ErrorKind::from_errcode(code).errcode(), code);
	}
	assert_eq!(ErrorKind::from_errcode("M_SOMETHING_ELSE").errcode(), "M_UNKNOWN");
	assert_eq!(
		ErrorKind::from_errcode("M_SENDER_IGNORED").errcode(),
		"UK.TIMEDOUT.MSC4406.SENDER_IGNORED"
	);
}

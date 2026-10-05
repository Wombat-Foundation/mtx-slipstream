//! Wire-shape goldens for event contents that are embedded in signed PDUs.
//! Expected strings are what the previous (ruma + serde) build emitted: sorted
//! keys, `None` and default values omitted, renamed keys preserved.

use mtx_slipstream::{
	codec::{from_str, to_string},
	events::{
		room::{
			member::{MembershipState, RoomMemberEventContent},
			power_levels::RoomPowerLevelsEventContent,
		},
		space::child::SpaceChildEventContent,
	},
};

fn member(json: &str) -> RoomMemberEventContent { from_str(json).unwrap() }

#[test]
fn member_minimal_omits_every_none() {
	let content = RoomMemberEventContent::new(MembershipState::Join);
	assert_eq!(to_string(&content), r#"{"membership":"join"}"#);
}

#[test]
fn member_keeps_renamed_keys() {
	let mut content = RoomMemberEventContent::new(MembershipState::Join);
	content.blurhash = Some("hash".into());
	content.redact_events = Some(true);
	content.join_authorized_via_users_server = Some("@a:b".into());
	assert_eq!(
		to_string(&content),
		concat!(
			r#"{"join_authorised_via_users_server":"@a:b","membership":"join","#,
			r#""org.matrix.msc4293.redact_events":true,"xyz.amorgan.blurhash":"hash"}"#
		)
	);
}

#[test]
fn member_decodes_renamed_keys() {
	let content = member(
		r#"{"membership":"join","join_authorised_via_users_server":"@a:b","org.matrix.msc4293.redact_events":true,"xyz.amorgan.blurhash":"h"}"#,
	);
	assert_eq!(content.join_authorized_via_users_server.as_ref().map(ToString::to_string).as_deref(), Some("@a:b"));
	assert_eq!(content.redact_events, Some(true));
	assert_eq!(content.blurhash.as_deref(), Some("h"));
}

#[test]
fn member_null_optionals_are_none() {
	let content = member(r#"{"membership":"leave","displayname":null,"reason":null}"#);
	assert!(content.displayname.is_none() && content.reason.is_none());
}

#[test]
fn member_empty_avatar_url_is_none() {
	// ruma `compat-empty-string-null`
	assert!(member(r#"{"membership":"join","avatar_url":""}"#).avatar_url.is_none());
}

#[test]
fn member_third_party_invite_round_trips_whole() {
	let text = concat!(
		r#"{"membership":"invite","third_party_invite":{"display_name":"Alice","signed":"#,
		r#"{"mxid":"@a:b","signatures":{"s.org":{"ed25519:0":"sig"}},"token":"t"}}}"#
	);
	assert_eq!(to_string(&member(text)), text);
}

#[test]
fn member_unknown_membership_is_rejected() {
	assert!(from_str::<RoomMemberEventContent>(r#"{"membership":"bogus"}"#).is_err());
}

#[test]
fn space_child_omits_defaults() {
	let bare: SpaceChildEventContent = from_str(r#"{"via":["a.org"]}"#).unwrap();
	assert!(!bare.suggested);
	assert_eq!(to_string(&bare), r#"{"via":["a.org"]}"#);
	let full: SpaceChildEventContent =
		from_str(r#"{"order":"01","suggested":true,"via":["a.org"]}"#).unwrap();
	assert_eq!(to_string(&full), r#"{"order":"01","suggested":true,"via":["a.org"]}"#);
	let explicit_false: SpaceChildEventContent =
		from_str(r#"{"suggested":false,"via":["a.org"]}"#).unwrap();
	assert_eq!(to_string(&explicit_false), r#"{"via":["a.org"]}"#);
}

#[test]
fn space_child_wrong_types_match_serde() {
	// serde: `suggested` is `bool` with `default`, so null and non-bool are errors;
	// `order` is `Option<String>`, so null is None and a number is an error.
	assert!(from_str::<SpaceChildEventContent>(r#"{"suggested":null,"via":[]}"#).is_err());
	assert!(from_str::<SpaceChildEventContent>(r#"{"suggested":"yes","via":[]}"#).is_err());
	assert!(from_str::<SpaceChildEventContent>(r#"{"order":5,"via":[]}"#).is_err());
	assert!(from_str::<SpaceChildEventContent>(r#"{"order":null,"via":[]}"#).unwrap().order.is_none());
	assert!(from_str::<SpaceChildEventContent>(r#"{"suggested":true}"#).is_err());
}

#[test]
fn power_levels_defaults_and_v1_strings() {
	let content: RoomPowerLevelsEventContent =
		from_str(r#"{"users":{"@a:b":"100"},"events":{"m.room.name":"50"},"ban":"75"}"#).unwrap();
	assert_eq!(content.ban, 75.into());
	assert_eq!(content.kick, 50.into());
	assert_eq!(content.users.values().next(), Some(&100.into()));
}

#[test]
fn power_levels_wire_shape() {
	let text = concat!(
		r#"{"ban":50,"events":{"m.room.name":50},"events_default":0,"invite":0,"kick":50,"#,
		r#""notifications":{"room":50},"redact":50,"state_default":50,"#,
		r#""users":{"@a:b":100},"users_default":0}"#
	);
	let content: RoomPowerLevelsEventContent = from_str(text).unwrap();
	// previous build: ruma serialized every level field; only empty maps were skipped.
	assert_eq!(to_string(&content), text);
}

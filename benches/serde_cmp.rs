#![feature(test)]
#![allow(
	clippy::pedantic,
	clippy::cargo,
	clippy::multiple_crate_versions,
	clippy::module_name_repetitions,
	clippy::similar_names,
	dead_code
)]

extern crate test;

use mtx_slipstream::{
	federation::{
		pdu_stream::PduStreamWriter,
		raw_pdu::{canonical_to_bytes, canonical_to_bytes_without},
	},
	writer::to_bytes,
};
use simd_json::prelude::*;
use test::Bencher;

// ── Payload generators ───────────────────────────────────────────────

/// Minimal room-create PDU (~300 bytes).
fn small_pdu() -> simd_json::OwnedValue {
	simd_json::json!({
		"event_id": "$aBcDeFgHiJkLmNoPqRsTuVwXyZ0123456789:example.com",
		"type": "m.room.create",
		"room_id": "!room:example.com",
		"sender": "@alice:example.com",
		"origin_server_ts": 1_700_000_000_000u64,
		"content": {"creator": "@alice:example.com", "room_version": "11"},
		"state_key": "",
		"hashes": {"sha256": "aa"},
		"signatures": {"example.com": {"ed25519:key1": "sig"}}
	})
}

/// Deeply nested sync v3 response with many rooms (~100 KB+).
#[allow(clippy::arithmetic_side_effects)]
fn huge_sync_response() -> simd_json::OwnedValue {
	let mut rooms_json = String::with_capacity(12 * 1024 * 1024);
	rooms_json.push('{');
	for room_idx in 0..2000_usize {
		if room_idx > 0 {
			rooms_json.push(',');
		}
		let room_id = format!("!room{room_idx}:example.com");
		rooms_json.push_str(&format!(r#""{room_id}":{{"timeline":{{"events":["#));
		for i in 0..20_usize {
			if i > 0 {
				rooms_json.push(',');
			}
			rooms_json.push_str(&format!(
				r#"{{"type":"m.room.message","content":{{"msgtype":"m.text","body":"message {i} in room {room_idx} — some padding text to make this event larger and more realistic for benchmarking purposes"}},"event_id":"$evt{room_idx}_{i}:example.com","sender":"@user{room_idx}:example.com","origin_server_ts":{ts}}}"#,
				ts = 1_700_000_000_000u64.saturating_add((room_idx * 10 + i) as u64)
			));
		}
		rooms_json.push_str(&format!(
			r#"],"limited":true,"prev_batch":"t{room_idx}"}},"state":{{"events":[{{"type":"m.room.create","content":{{"creator":"@user{room_idx}:example.com"}}}},{{"type":"m.room.join_rules","content":{{"join_rule":"public"}}}}]}},"ephemeral":{{"events":[{{"type":"m.typing","content":{{"users":[{{"user_id":"@user{room_idx}:example.com","typing":true}}]}}}}]}},"account_data":{{"events":[]}},"unread_notifications":{{"notification_count":{notif},"highlight_count":{hl}}}}}"#,
			notif = room_idx % 10,
			hl = room_idx % 3
		));
	}
	rooms_json.push('}');

	let full = format!(
		r#"{{"next_batch":"s1234567890_abcdef_1234567890_abcdef","rooms":{{"join":{rooms},"leave":{{}},"invite":{{}}}},"presence":{{"events":[]}},"account_data":{{"events":[]}},"to_device":{{"events":[]}},"device_lists":{{"changed":[],"left":[]}},"device_one_time_keys_count":{{"signed_curve25519":50}}}}"#,
		rooms = rooms_json
	);
	let mut bytes = full.into_bytes();
	simd_json::to_owned_value(&mut bytes).unwrap()
}

fn encode_bytes(val: &simd_json::OwnedValue) -> Vec<u8> { val.encode().into_bytes() }

// ── Parse benchmarks ─────────────────────────────────────────────────

#[bench]
fn bench_simd_parse_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let mut json = encode_bytes(&pdu);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_owned_value(&mut json).unwrap();
	});
}

#[bench]
fn bench_serde_parse_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let json = encode_bytes(&pdu);

	b.iter(|| {
		let data = json.clone();
		serde_json::from_slice::<serde_json::Value>(&data).unwrap();
	});
}

#[bench]
fn bench_simd_parse_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let mut json = encode_bytes(&val);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_owned_value(&mut json).unwrap();
	});
}

#[bench]
fn bench_serde_parse_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let json = encode_bytes(&val);

	b.iter(|| {
		let data = json.clone();
		serde_json::from_slice::<serde_json::Value>(&data).unwrap();
	});
}

// ── Serialize benchmarks ─────────────────────────────────────────────

#[bench]
fn bench_simd_serialize_small(b: &mut Bencher) {
	let pdu = small_pdu();
	b.iter(|| to_bytes(&pdu).unwrap());
}

#[bench]
fn bench_serde_serialize_small(b: &mut Bencher) {
	let pdu = small_pdu();
	// Convert simd_json value → serde_json value once up front
	let json_str = encode_bytes(&pdu);
	let serde_val: serde_json::Value = serde_json::from_slice(&json_str).unwrap();
	b.iter(|| serde_json::to_string(&serde_val).unwrap());
}

#[bench]
fn bench_simd_serialize_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	b.iter(|| to_bytes(&val).unwrap());
}

#[bench]
fn bench_serde_serialize_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let json_str = encode_bytes(&val);
	let serde_val: serde_json::Value = serde_json::from_slice(&json_str).unwrap();
	b.iter(|| serde_json::to_string(&serde_val).unwrap());
}

// ── Roundtrip benchmarks ─────────────────────────────────────────────

#[bench]
fn bench_simd_roundtrip_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let json = encode_bytes(&pdu);

	b.iter(|| {
		let mut input = json.clone();
		let val = simd_json::to_owned_value(&mut input).unwrap();
		to_bytes(&val).unwrap()
	});
}

#[bench]
fn bench_serde_roundtrip_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let json = encode_bytes(&pdu);

	b.iter(|| {
		let data = json.clone();
		let val: serde_json::Value = serde_json::from_slice(&data).unwrap();
		serde_json::to_string(&val).unwrap()
	});
}

#[bench]
fn bench_simd_roundtrip_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let json = encode_bytes(&val);

	b.iter(|| {
		let mut input = json.clone();
		let parsed = simd_json::to_owned_value(&mut input).unwrap();
		to_bytes(&parsed).unwrap()
	});
}

#[bench]
fn bench_serde_roundtrip_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let json = encode_bytes(&val);

	b.iter(|| {
		let data = json.clone();
		let parsed: serde_json::Value = serde_json::from_slice(&data).unwrap();
		serde_json::to_string(&parsed).unwrap()
	});
}

// ── Canonical serialize benchmarks ───────────────────────────────────

#[bench]
fn bench_simd_canonical_small(b: &mut Bencher) {
	let pdu = small_pdu();
	b.iter(|| canonical_to_bytes(&pdu).unwrap());
}

#[bench]
fn bench_simd_canonical_without_fields(b: &mut Bencher) {
	let pdu = small_pdu();
	b.iter(|| canonical_to_bytes_without(&pdu, &["unsigned"]).unwrap());
}

// ── Zero-copy parse benchmarks ───────────────────────────────────────
// These use simd_json::to_borrowed_value which borrows directly from the
// input buffer — no OwnedValue tree construction. This is where SIMD
// acceleration actually shines: the lex/validate pass is pure scan.

#[bench]
fn bench_simd_zerocopy_parse_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let mut json = encode_bytes(&pdu);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_borrowed_value(&mut json).unwrap();
	});
}

#[bench]
fn bench_simd_zerocopy_parse_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let mut json = encode_bytes(&val);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_borrowed_value(&mut json).unwrap();
	});
}

// ── Raw passthrough benchmarks ───────────────────────────────────────
// The actual hot path for PDUs that don't need patching: just shove the
// raw JSON bytes through the stream writer. Zero parse, zero serialize.

#[bench]
fn bench_raw_passthrough_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let json_str = encode_bytes(&pdu);
	let json_str = String::from_utf8(json_str).unwrap();

	b.iter(|| {
		let mut stream = PduStreamWriter::with_capacity(1);
		stream.write_raw_pdu(&json_str);
		stream.finish()
	});
}

#[bench]
fn bench_raw_passthrough_huge(b: &mut Bencher) {
	let val = huge_sync_response();
	let json_str = encode_bytes(&val);
	let json_str = String::from_utf8(json_str).unwrap();

	b.iter(|| {
		let mut stream = PduStreamWriter::with_capacity(1);
		stream.write_raw_pdu(&json_str);
		stream.finish()
	});
}

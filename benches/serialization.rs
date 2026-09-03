#![feature(test)]
extern crate test;

use bytes::BytesMut;
use simd_json::prelude::*;
use test::Bencher;

use mtx_slipstream::federation::pdu_stream::{FederationResponseWriter, PduStreamWriter};
use mtx_slipstream::federation::raw_pdu::{canonical_to_bytes, canonical_to_bytes_without};
use mtx_slipstream::writer::{to_bytes, BufWriter};

// ── Shared payloads ──────────────────────────────────────────────────

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

/// Typical m.room.message PDU (~1 KB).
fn medium_pdu() -> simd_json::OwnedValue {
	simd_json::json!({
		"event_id": "$bCdEfGhIjKlMnOpQrStUvWxYz01234567890:example.com",
		"type": "m.room.message",
		"room_id": "!room:example.com",
		"sender": "@bob:example.com",
		"origin_server_ts": 1_700_000_060_000u64,
		"content": {
			"msgtype": "m.text",
			"body": "Hello, world! This is a longer message to simulate a realistic payload with some extra text."
		},
		"state_key": null,
		"unsigned": {"age": 42, "transaction_id": "t1234"},
		"hashes": {"sha256": "bb"},
		"signatures": {"example.com": {"ed25519:key1": "sig"}}
	})
}

/// Deeply nested sync v3 response (~5 KB).
fn sync_response() -> simd_json::OwnedValue {
	simd_json::json!({
		"next_batch": "s1234567890_abcdef_1234567890_abcdef",
		"rooms": {
			"join": {
				"!abc123:example.com": {
					"timeline": {
						"events": [
							{"type": "m.room.message", "content": {"body": "hi", "msgtype": "m.text"}},
							{"type": "m.room.message", "content": {"body": "hello", "msgtype": "m.text"}},
							{"type": "m.room.member", "content": {"membership": "join"}, "state_key": "@alice:example.com"}
						],
						"limited": true,
						"prev_batch": "t111"
					},
					"state": {"events": [
						{"type": "m.room.create", "content": {"creator": "@alice:example.com"}},
						{"type": "m.room.join_rules", "content": {"join_rule": "public"}}
					]},
					"ephemeral": {"events": [
						{"type": "m.typing", "content": {"users": [{"user_id": "@alice:example.com", "typing": true}]}}
					]},
					"account_data": {"events": []},
					"unread_notifications": {"notification_count": 3, "highlight_count": 1}
				},
				"!def456:example.com": {
					"timeline": {
						"events": [
							{"type": "m.room.message", "content": {"body": "test", "msgtype": "m.text"}}
						],
						"limited": false,
						"prev_batch": "t222"
					},
					"state": {"events": []},
					"ephemeral": {"events": []},
					"account_data": {"events": []},
					"unread_notifications": {"notification_count": 0, "highlight_count": 0}
				}
			},
			"leave": {},
			"invite": {}
		},
		"presence": {"events": []},
		"account_data": {"events": []},
		"to_device": {"events": []},
		"device_lists": {"changed": [], "left": []},
		"device_one_time_keys_count": {"signed_curve25519": 50}
	})
}

/// N PDUs for stream benchmarks.
fn pdus(n: usize) -> Vec<simd_json::OwnedValue> {
	(0..n)
		.map(|i| {
			simd_json::json!({
				"event_id": format!("$pdu{i}:example.com"),
				"type": "m.room.message",
				"room_id": "!room:example.com",
				"sender": format!("@user{i}:example.com"),
				"origin_server_ts": 1_700_000_000_000u64 + i as u64,
				"content": {"msgtype": "m.text", "body": format!("message {i}")}
			})
		})
		.collect()
}

/// Encode a value to a `Vec<u8>` without serde (replaces `simd_json::to_string`).
fn encode_bytes(val: &simd_json::OwnedValue) -> Vec<u8> {
	val.encode().into_bytes()
}

// ── Parse benchmarks ─────────────────────────────────────────────────

#[bench]
fn bench_parse_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let mut json = encode_bytes(&pdu);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_owned_value(&mut json).unwrap();
	});
}

#[bench]
fn bench_parse_medium(b: &mut Bencher) {
	let pdu = medium_pdu();
	let mut json = encode_bytes(&pdu);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_owned_value(&mut json).unwrap();
	});
}

#[bench]
fn bench_parse_sync_response(b: &mut Bencher) {
	let val = sync_response();
	let mut json = encode_bytes(&val);
	let original = json.clone();

	b.iter(|| {
		json.clone_from_slice(&original);
		simd_json::to_owned_value(&mut json).unwrap();
	});
}

// ── Serialize benchmarks ─────────────────────────────────────────────

#[bench]
fn bench_to_bytes_small(b: &mut Bencher) {
	let pdu = small_pdu();
	b.iter(|| to_bytes(&pdu).unwrap());
}

#[bench]
fn bench_to_bytes_medium(b: &mut Bencher) {
	let pdu = medium_pdu();
	b.iter(|| to_bytes(&pdu).unwrap());
}

#[bench]
fn bench_to_bytes_sync(b: &mut Bencher) {
	let val = sync_response();
	b.iter(|| to_bytes(&val).unwrap());
}

#[bench]
fn bench_canonical_small(b: &mut Bencher) {
	let pdu = small_pdu();
	b.iter(|| canonical_to_bytes(&pdu).unwrap());
}

#[bench]
fn bench_canonical_medium(b: &mut Bencher) {
	let pdu = medium_pdu();
	b.iter(|| canonical_to_bytes(&pdu).unwrap());
}

#[bench]
fn bench_canonical_without_fields(b: &mut Bencher) {
	let pdu = medium_pdu();
	b.iter(|| canonical_to_bytes_without(&pdu, &["unsigned"]).unwrap());
}

// ── Parse → serialize round-trip ─────────────────────────────────────

#[bench]
fn bench_roundtrip_small(b: &mut Bencher) {
	let pdu = small_pdu();
	let json = encode_bytes(&pdu);

	b.iter(|| {
		let mut input = json.clone();
		let val = simd_json::to_owned_value(&mut input).unwrap();
		to_bytes(&val).unwrap()
	});
}

#[bench]
fn bench_roundtrip_sync(b: &mut Bencher) {
	let val = sync_response();
	let json = encode_bytes(&val);

	b.iter(|| {
		let mut input = json.clone();
		let parsed = simd_json::to_owned_value(&mut input).unwrap();
		to_bytes(&parsed).unwrap()
	});
}

// ── PDU stream writer benchmarks ─────────────────────────────────────

#[bench]
fn bench_pdu_stream_10(b: &mut Bencher) {
	let list = pdus(10);
	b.iter(|| {
		let mut stream = PduStreamWriter::with_capacity(list.len());
		for pdu in &list {
			stream.write_pdu(pdu).unwrap();
		}
		stream.finish()
	});
}

#[bench]
fn bench_pdu_stream_50(b: &mut Bencher) {
	let list = pdus(50);
	b.iter(|| {
		let mut stream = PduStreamWriter::with_capacity(list.len());
		for pdu in &list {
			stream.write_pdu(pdu).unwrap();
		}
		stream.finish()
	});
}

#[bench]
fn bench_pdu_stream_100(b: &mut Bencher) {
	let list = pdus(100);
	b.iter(|| {
		let mut stream = PduStreamWriter::with_capacity(list.len());
		for pdu in &list {
			stream.write_pdu(pdu).unwrap();
		}
		stream.finish()
	});
}

#[bench]
fn bench_federation_response_10(b: &mut Bencher) {
	let state = pdus(10);
	let auth_chain = pdus(5);
	b.iter(|| {
		let mut writer =
			FederationResponseWriter::with_capacity(state.len(), auth_chain.len());
		for pdu in &state {
			writer.write_state_pdu(pdu).unwrap();
		}
		writer.begin_auth_chain();
		for pdu in &auth_chain {
			writer.write_auth_chain_pdu(pdu).unwrap();
		}
		writer.finish()
	});
}

// ── Raw BufWriter benchmark ──────────────────────────────────────────

#[bench]
fn bench_bufwriter_sync(b: &mut Bencher) {
	let val = sync_response();
	b.iter(|| {
		let mut buf = BytesMut::with_capacity(8192);
		let mut writer = BufWriter(&mut buf);
		val.write(&mut writer).unwrap();
		buf
	});
}

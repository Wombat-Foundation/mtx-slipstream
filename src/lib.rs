//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;

pub mod federation;
pub mod sync;
pub mod writer;

/// Canonical JSON substrate supplied by Rezzy.
pub use rezzy::json;

/// Primitive Matrix scalar compatibility types.
pub type Int = i64;
pub type UInt = u64;

/// Matrix-facing names shared by the server and the serialization layer.
///
/// This is intentionally kept at the crate root so the eventual migration
/// from `ruma` can be a namespace change rather than another JSON rewrite.
pub type CanonicalJsonObject = json::Object;
pub type CanonicalJsonValue = json::Value;
pub type CanonicalJsonArray = Vec<json::Value>;

pub mod canonical_json {
	pub use crate::{CanonicalJsonArray, CanonicalJsonObject, CanonicalJsonValue};
}

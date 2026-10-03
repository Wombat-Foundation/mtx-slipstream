//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and
//! `send_join` responses.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod federation;
pub mod sync;
pub mod writer;

/// Canonical JSON substrate supplied by Rezzy.
pub use rezzy::json;

/// Matrix compatibility façade. These exports allow downstream crates to
/// migrate from `ruma::...` to `mtx_slipstream::...` while the underlying
/// implementations are progressively moved onto Rezzy.
pub use ruma::*;
pub use ruma::{api, canonical_json, events, http_headers, serde, signatures};

/// Matrix-facing names shared by the server and the serialization layer.
///
/// This is intentionally kept at the crate root so the eventual migration
/// from `ruma` can be a namespace change rather than another JSON rewrite.
pub type CanonicalJsonObject = ruma::CanonicalJsonObject;
pub type CanonicalJsonValue = ruma::CanonicalJsonValue;
pub type CanonicalJsonArray = ruma::CanonicalJsonArray;

pub mod canonical_json {
	pub use crate::{CanonicalJsonArray, CanonicalJsonObject, CanonicalJsonValue};
}

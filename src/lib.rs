//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and send_join
//! responses.

#![allow(
	clippy::arithmetic_side_effects,
	clippy::missing_assert_message,
	clippy::must_use_candidate,
	clippy::return_self_not_must_use,
	clippy::as_conversions,
	clippy::str_to_string,
	clippy::string_lit_as_bytes,
	clippy::default_trait_access,
	clippy::unseparated_literal_suffix,
	clippy::unnecessary_struct_initialization,
	clippy::missing_errors_doc,
	clippy::doc_markdown,
	clippy::cargo_common_metadata,
	clippy::collapsible_if
)]

pub mod federation;
pub mod sync;
pub mod writer;

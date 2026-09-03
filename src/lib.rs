//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and send_join
//! responses. Instead of the current triple-serialization pattern:
//!
//! ```text
//! ruma types → serialize → bytes → deserialize → Value → patch → serialize → bytes
//! ```
//!
//! mtx-slipstream provides:
//! - **Direct-to-bytes sync response construction** with inline patching
//! - **Streaming federation PDU responses** that avoid `Vec` materialization
//! - **Direct `CanonicalJsonObject` → bytes** conversion without intermediate
//!   `Box<RawValue>`

pub mod federation;
pub mod sync;
pub mod writer;

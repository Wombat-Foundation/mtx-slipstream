//! # mtx-slipstream
//!
//! High-performance serialization for Matrix Client-Server and Federation APIs.
//!
//! Eliminates redundant serialize/deserialize round-trips in sync and send_join
//! responses.

#![feature(coverage_attribute)]

pub mod federation;
pub mod sync;
pub mod writer;

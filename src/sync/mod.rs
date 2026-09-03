//! Sync response optimization.
//!
//! Eliminates the triple-serialization pattern in sync responses by:
//! 1. Using `serde_json::to_value()` instead of `try_into_http_response`
//! 2. Patching the `Value` tree directly
//! 3. Serializing to bytes via `JsonWriter` instead of `axum::Json`

pub mod v3;
pub mod v5;

//! Canonical JSON compatibility primitives.

pub use crate::json;

pub type Object = json::Object;
pub type Value = json::Value;
pub type Array = alloc::vec::Vec<Value>;

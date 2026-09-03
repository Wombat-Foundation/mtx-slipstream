//! Federation response optimization.
//!
//! Provides streaming PDU serialization and direct `CanonicalJsonObject` → bytes
//! conversion to avoid per-PDU round-trips in send_join, event_auth, and other
//! federation endpoints.

pub mod pdu_stream;
pub mod raw_pdu;

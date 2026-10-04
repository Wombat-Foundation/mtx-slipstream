use alloc::collections::BTreeMap;
use crate::{OwnedEventId, OwnedRoomId, OwnedUserId, UInt};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReceiptType { Read, ReadPrivate, FullyRead }
impl Default for ReceiptType { fn default() -> Self { Self::Read } }
crate::impl_codec_enum!(ReceiptType { Read => "m.read", ReadPrivate => "m.read.private", FullyRead => "m.fully_read" });

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReceiptThread { Unthreaded, Thread(OwnedEventId) }
impl Default for ReceiptThread { fn default() -> Self { Self::Unthreaded } }

#[derive(Clone, Debug, Default)]
pub struct Receipt { pub ts: Option<UInt>, pub thread: ReceiptThread }
pub type ReceiptEventContent = BTreeMap<OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>>;

#[derive(Clone, Debug)]
pub struct ReceiptEvent { pub content: ReceiptEventContent, pub room_id: OwnedRoomId }

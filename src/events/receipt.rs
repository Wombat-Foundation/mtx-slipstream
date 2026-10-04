use crate::{OwnedEventId, OwnedRoomId, OwnedUserId, UInt};
use alloc::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum ReceiptType {
	#[default]
	Read,
	ReadPrivate,
	FullyRead,
}
crate::impl_codec_enum!(ReceiptType { Read => "m.read", ReadPrivate => "m.read.private", FullyRead => "m.fully_read" });

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum ReceiptThread {
	#[default]
	Unthreaded,
	Thread(OwnedEventId),
}

#[derive(Clone, Debug, Default)]
pub struct Receipt {
	pub ts: Option<UInt>,
	pub thread: ReceiptThread,
}
pub type ReceiptEventContent =
	BTreeMap<OwnedEventId, BTreeMap<ReceiptType, BTreeMap<OwnedUserId, Receipt>>>;

#[derive(Clone, Debug)]
pub struct ReceiptEvent {
	pub content: ReceiptEventContent,
	pub room_id: OwnedRoomId,
}

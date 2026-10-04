//! Thread listing.

use crate::impl_codec_enum;

/// Which threads to list.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IncludeThreads {
	/// Every thread in the room.
	#[default]
	All,
	/// Only threads the user has participated in.
	Participated,
}

impl_codec_enum!(IncludeThreads { All => "all", Participated => "participated" });

impl IncludeThreads {
	#[must_use]
	pub fn as_str(self) -> &'static str {
		match self {
			Self::All => "all",
			Self::Participated => "participated",
		}
	}
}

pub mod get_threads {
	pub mod v1 {
		use alloc::{string::String, vec::Vec};

		use crate::{OwnedRoomId, UInt, federation_api::RawPdu, threads::IncludeThreads};

		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v1/rooms/{room_id}/threads",
			request {
				path { room_id: OwnedRoomId }
				query { include: IncludeThreads, limit: Option<UInt>, from: Option<String> }
				body {}
			}
			response { chunk: Vec<RawPdu>, next_batch: Option<String> }
		}
	}
}

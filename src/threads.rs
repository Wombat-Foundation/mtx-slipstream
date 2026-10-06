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

		use crate::{OwnedRoomId, UInt, federation_api::RawPdu};

		pub use crate::threads::IncludeThreads;

		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v1/rooms/{room_id}/threads",
			request {
				path { room_id: OwnedRoomId }
				query { include: IncludeThreads = IncludeThreads::All, limit: Option<UInt>, from: Option<String> }
				body {}
			}
			response { chunk: Vec<RawPdu>, next_batch: Option<String> }
		}
	}
}

#[cfg(test)]
mod get_threads_tests {
	use super::{IncludeThreads, get_threads::v1::Request};
	use crate::endpoint::EndpointRequest;

	#[test]
	fn include_defaults_to_all() {
		let path = ["!room:example.org".to_owned()];
		let request = Request::from_parts(&path, &[], None).unwrap();
		assert_eq!(request.include, IncludeThreads::All);
		assert!(request.limit.is_none() && request.from.is_none());
	}
}

//! Thread listing.

/// Which threads to list.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IncludeThreads {
	/// Every thread in the room.
	#[default]
	All,
	/// Only threads the user has participated in.
	Participated,
}

impl crate::codec::Serialize for IncludeThreads {
	fn to_json(&self) -> crate::json::Value {
		crate::json::Value::String(::alloc::string::String::from(match self {
			Self::All => "all",
			Self::Participated => "participated",
		}))
	}
}
impl crate::codec::Deserialize for IncludeThreads {
	fn from_json(value: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
		match value.as_str() {
			Some("all") => Ok(Self::All),
			Some("participated") => Ok(Self::Participated),
			_ => Err(crate::codec::DeError::expected(stringify!(IncludeThreads))),
		}
	}
}

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

		pub struct Request {
			pub room_id: OwnedRoomId,
			pub include: IncludeThreads,
			pub limit: Option<UInt>,
			pub from: Option<String>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		crate::endpoint_request_impl! {
			method: "GET",
			path: "/_matrix/client/v1/rooms/{room_id}/threads",
			path { room_id }
			query { include => "include" = IncludeThreads::All, limit => "limit", from => "from" }
			body {}
		}
		pub struct Response {
			pub chunk: Vec<RawPdu>,
			pub next_batch: Option<String>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		crate::endpoint_response_impl! {
			body { chunk => "chunk", next_batch => "next_batch" }
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

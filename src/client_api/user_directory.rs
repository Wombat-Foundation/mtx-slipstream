pub mod search_users {
	pub mod v3 {
		use crate::{OwnedMxcUri, UInt, endpoint};
		#[derive(Debug)]
		pub struct User {
			pub user_id: crate::OwnedUserId,
			pub display_name: Option<String>,
			pub avatar_url: Option<OwnedMxcUri>,
		}
		crate::impl_codec_struct!(User { user_id: crate::OwnedUserId, display_name: Option<String>, avatar_url: Option<OwnedMxcUri> });
		endpoint! { method: "POST", path: "/_matrix/client/v3/user_directory/search", request { path {} query {} body { search_term: String, limit: UInt = 10, language: Option<String> } } response { results: Vec<User>, limited: bool } }
	}
}

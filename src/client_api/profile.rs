pub use crate::client_api::profile_keys::{delete_profile_key, get_profile_key, set_profile_key};

pub mod get_profile {
	pub mod v3 {
		use crate::{OwnedMxcUri, OwnedUserId, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/client/v3/profile/{userId}",
			request { path { user_id: OwnedUserId } query {} body {} }
			response { avatar_url: Option<OwnedMxcUri>, displayname: Option<String> }
		}
	}
}

pub mod get_display_name {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/client/v3/profile/{userId}/displayname",
			request { path { user_id: OwnedUserId } query {} body {} }
			response { displayname: Option<String> }
		}
	}
}

pub mod set_display_name {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};

		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/profile/{userId}/displayname",
			request { path { user_id: OwnedUserId } query {} body { displayname: Option<String> } }
			response {}
		}
	}
}

pub mod get_avatar_url {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/client/v3/profile/{userId}/avatar_url",
			request { path { user_id: OwnedUserId } query {} body {} }
			response { avatar_url: Option<crate::OwnedMxcUri> }
		}
	}
}

pub mod set_avatar_url {
	pub mod v3 {
		use crate::{OwnedMxcUri, OwnedUserId, endpoint};

		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/profile/{userId}/avatar_url",
			request { path { user_id: OwnedUserId } query {} body { avatar_url: Option<OwnedMxcUri> } }
			response {}
		}
	}
}

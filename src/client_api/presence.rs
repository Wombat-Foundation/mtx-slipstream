pub mod get_presence {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint, events::presence::PresenceState};
		endpoint! {
			method: "GET", path: "/_matrix/client/v3/presence/{userId}/status",
			request { path { user_id: OwnedUserId } query {} body {} }
			response { presence: PresenceState, last_active_ago: Option<std::time::Duration>, status_msg: Option<String>, currently_active: Option<bool> }
		}
	}
}

pub mod set_presence {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint, events::presence::PresenceState};
		endpoint! {
			method: "PUT", path: "/_matrix/client/v3/presence/{userId}/status",
			request { path { user_id: OwnedUserId } query {} body { presence: PresenceState = PresenceState::Online, status_msg: Option<String> } }
			response {}
		}
	}
}

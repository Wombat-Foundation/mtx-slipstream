pub mod get_turn_server_info {
	pub mod v3 {
		use crate::{UInt, endpoint};
		endpoint! { method: "GET", path: "/_matrix/client/v3/voip/turnServer", request { path {} query {} body {} } response { username: String, password: String, uris: Vec<String>, ttl: UInt } }
	}
}

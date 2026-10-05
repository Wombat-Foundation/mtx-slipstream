pub mod create_alias {
	pub mod v3 {
		use crate::{OwnedRoomAliasId, OwnedRoomId, endpoint};
		endpoint! { method: "PUT", path: "/_matrix/client/v3/directory/room/{roomAlias}", request { path { room_alias: OwnedRoomAliasId } query {} body { room_id: OwnedRoomId } } response {} }
	}
}
pub mod delete_alias {
	pub mod v3 {
		use crate::{OwnedRoomAliasId, endpoint};
		endpoint! { method: "DELETE", path: "/_matrix/client/v3/directory/room/{roomAlias}", request { path { room_alias: OwnedRoomAliasId } query {} body {} } response {} }
	}
}
pub mod get_alias {
	pub mod v3 {
		use crate::{OwnedRoomAliasId, OwnedRoomId, endpoint};
		endpoint! { method: "GET", path: "/_matrix/client/v3/directory/room/{roomAlias}", request { path { room_alias: OwnedRoomAliasId } query {} body {} } response { room_id: OwnedRoomId, servers: Vec<String> } }
	}
}

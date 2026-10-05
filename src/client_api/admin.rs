pub mod get_suspended {
	pub mod v1 {
		use crate::{OwnedUserId, endpoint};
		endpoint! { method: "GET", path: "/_matrix/client/unstable/uk.timedout.msc4323/admin/suspend/{userId}", request { path { user_id: OwnedUserId } query {} body {} } response { suspended: bool } }
		impl Response {
			pub fn new(suspended: bool) -> Self {
				Self {
					suspended,
				}
			}
		}
	}
}
pub mod set_suspended {
	pub mod v1 {
		use crate::{OwnedUserId, endpoint};
		endpoint! { method: "PUT", path: "/_matrix/client/unstable/uk.timedout.msc4323/admin/suspend/{userId}", request { path { user_id: OwnedUserId } query {} body { suspended: bool } } response { suspended: bool } }
		impl Response {
			pub fn new(suspended: bool) -> Self {
				Self {
					suspended,
				}
			}
		}
	}
}

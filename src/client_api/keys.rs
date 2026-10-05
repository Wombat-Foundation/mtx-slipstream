pub mod upload_keys {
	pub mod v3 {
		use crate::{OwnedOneTimeKeyId, encryption::DeviceKeys, endpoint, serde::Raw};
		use std::collections::BTreeMap;

		endpoint! {
			method: "POST", path: "/_matrix/client/v3/keys/upload",
			request {
				path {} query {}
				body {
					device_keys: Option<Raw<DeviceKeys>>,
					one_time_keys: BTreeMap<OwnedOneTimeKeyId, Raw<crate::encryption::OneTimeKey>>,
					fallback_keys: BTreeMap<OwnedOneTimeKeyId, Raw<crate::encryption::OneTimeKey>>,
				}
			}
			response { one_time_key_counts: BTreeMap<crate::OneTimeKeyAlgorithm, crate::UInt> }
		}
	}
}

pub mod get_keys {
	pub mod v3 {
		use crate::{
			OwnedDeviceId, OwnedUserId,
			encryption::{CrossSigningKey, DeviceKeys},
			endpoint,
			serde::Raw,
		};
		use std::collections::BTreeMap;

		endpoint! {
			method: "POST", path: "/_matrix/client/v3/keys/query",
			request { path {} query {} body { device_keys: BTreeMap<OwnedUserId, Vec<OwnedDeviceId>>, timeout: Option<std::time::Duration> } }
			response { device_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<DeviceKeys>>>, master_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>, self_signing_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>, user_signing_keys: BTreeMap<OwnedUserId, Raw<CrossSigningKey>>, failures: BTreeMap<String, crate::json::Value> }
		}
	}
}

pub mod claim_keys {
	pub mod v3 {
		use crate::{
			OneTimeKeyAlgorithm, OwnedDeviceId, OwnedOneTimeKeyId, OwnedUserId, endpoint,
			serde::Raw,
		};
		use std::collections::BTreeMap;

		/// The one-time keys for a single device.
		pub type OneTimeKeys = BTreeMap<
			OwnedDeviceId,
			BTreeMap<OwnedOneTimeKeyId, Raw<crate::encryption::OneTimeKey>>,
		>;

		endpoint! {
			method: "POST", path: "/_matrix/client/v3/keys/claim",
			request {
				path {} query {}
				body {
					one_time_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, OneTimeKeyAlgorithm>>,
					timeout: Option<std::time::Duration>
				}
			}
			response {
				one_time_keys: BTreeMap<OwnedUserId, OneTimeKeys>,
				failures: BTreeMap<String, crate::json::Value>
			}
		}

		impl Response {
			/// Creates a response with the given keys and no failures.
			#[must_use]
			pub fn new(one_time_keys: BTreeMap<OwnedUserId, OneTimeKeys>) -> Self {
				Self {
					failures: BTreeMap::new(),
					one_time_keys,
				}
			}
		}
	}
}

pub mod upload_signing_keys {
	pub mod v3 {
		use crate::{encryption::CrossSigningKey, endpoint, serde::Raw};

		endpoint! {
			method: "POST", path: "/_matrix/client/v3/keys/device_signing/upload",
			request {
				path {} query {}
				body {
					auth: Option<crate::uiaa::AuthData>,
					master_key: Option<Raw<CrossSigningKey>>,
					self_signing_key: Option<Raw<CrossSigningKey>>,
					user_signing_key: Option<Raw<CrossSigningKey>>,
				}
			}
			response {}
		}
	}
}

pub mod upload_signatures {
	pub mod v3 {
		use crate::{
			OwnedDeviceId, OwnedUserId, encryption::CrossSigningKey, endpoint, serde::Raw,
		};
		use std::collections::BTreeMap;

		endpoint! {
			method: "POST", path: "/_matrix/client/v3/keys/signatures/upload",
			request {
				path {} query {}
				body { signed_keys: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<CrossSigningKey>>> }
			}
			response { failures: BTreeMap<OwnedUserId, BTreeMap<OwnedDeviceId, Raw<crate::json::Value>>> }
		}
	}
}

pub mod get_key_changes {
	pub mod v3 {
		use crate::{OwnedUserId, endpoint};

		endpoint! {
			method: "GET", path: "/_matrix/client/v3/keys/changes",
			request { path {} query { from: String, to: String } body {} }
			response { changed: Vec<OwnedUserId>, left: Vec<OwnedUserId> }
		}
	}
}

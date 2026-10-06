//! Server-side encrypted key backup.

use alloc::{collections::BTreeMap, string::String};

use crate::{UInt, impl_codec_struct, json::Value, sswire::Raw};

/// A backup algorithm and its public parameters.
#[derive(Debug)]
pub struct BackupAlgorithm {
	pub algorithm: String,
	pub auth_data: Value,
}

impl_codec_struct!(BackupAlgorithm {
	algorithm: String,
	auth_data: Value
});

/// One backed-up room key.
#[derive(Debug)]
pub struct KeyBackupData {
	pub first_message_index: UInt,
	pub forwarded_count: UInt,
	pub is_verified: bool,
	/// The encrypted session data.
	pub session_data: Value,
}

impl_codec_struct!(KeyBackupData {
	first_message_index: UInt,
	forwarded_count: UInt,
	is_verified: bool,
	session_data: Value,
});

/// The backed-up sessions of one room.
#[derive(Debug, Default)]
pub struct RoomKeyBackup {
	pub sessions: BTreeMap<String, Raw<KeyBackupData>>,
}

impl_codec_struct!(RoomKeyBackup { sessions: BTreeMap<String, Raw<KeyBackupData>> });

/// Response body of the backup-version endpoints: the algorithm fields sit at
/// the top level next to `count`, `etag` and `version`.
macro_rules! backup_info_response {
	() => {
		#[derive(Debug)]
		pub struct Response {
			pub algorithm: $crate::sswire::Raw<$crate::backup::BackupAlgorithm>,
			pub count: $crate::UInt,
			pub etag: ::alloc::string::String,
			pub version: ::alloc::string::String,
		}

		impl $crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> $crate::json::Value {
				let mut object = match $crate::codec::Serialize::to_json(&self.algorithm) {
					$crate::json::Value::Object(object) => object,
					_ => $crate::json::Object::new(),
				};
				object.extend($crate::endpoint::object_from(::alloc::vec![
					("count", $crate::codec::Serialize::to_json(&self.count)),
					("etag", $crate::codec::Serialize::to_json(&self.etag)),
					("version", $crate::codec::Serialize::to_json(&self.version)),
				]));
				$crate::json::Value::Object(object)
			}

			fn from_body(body: &$crate::json::Value) -> Result<Self, $crate::codec::DeError> {
				let input = $crate::endpoint::Input::new(&[], &[], Some(body));
				Ok(Self {
					algorithm: $crate::codec::Deserialize::from_json(body)?,
					count: input.body("count")?,
					etag: input.body("etag")?,
					version: input.body("version")?,
				})
			}
		}
	};
}

pub mod create_backup_version {
	pub mod v3 {
		crate::endpoint! {
			method: "POST", path: "/_matrix/client/v3/room_keys/version",
			request {
				path {}
				query {}
				body { algorithm: crate::sswire::Raw<crate::backup::BackupAlgorithm> }
			}
			response { version: alloc::string::String }
		}
	}
}

pub mod update_backup_version {
	pub mod v3 {
		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/client/v3/room_keys/version/{version}",
			request {
				path { version: alloc::string::String }
				query {}
				raw_body { algorithm: crate::sswire::Raw<crate::backup::BackupAlgorithm> }
			}
		}
		crate::endpoint_response! { response {} }
	}
}

pub mod get_latest_backup_info {
	pub mod v3 {
		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/room_keys/version",
			request { path {} query {} body {} }
		}
		backup_info_response!();
	}
}

pub mod get_backup_info {
	pub mod v3 {
		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/room_keys/version/{version}",
			request { path { version: alloc::string::String } query {} body {} }
		}
		backup_info_response!();
	}
}

pub mod delete_backup_version {
	pub mod v3 {
		crate::endpoint! {
			method: "DELETE", path: "/_matrix/client/v3/room_keys/version/{version}",
			request { path { version: alloc::string::String } query {} body {} }
			response {}
		}
	}
}

pub mod add_backup_keys {
	pub mod v3 {
		crate::endpoint! {
			method: "PUT", path: "/_matrix/client/v3/room_keys/keys",
			request {
				path {}
				query { version: alloc::string::String }
				body {
					rooms: alloc::collections::BTreeMap<crate::OwnedRoomId, crate::backup::RoomKeyBackup>
				}
			}
			response { count: crate::UInt, etag: alloc::string::String }
		}
	}
}

pub mod add_backup_keys_for_room {
	pub mod v3 {
		crate::endpoint! {
			method: "PUT", path: "/_matrix/client/v3/room_keys/keys/{room_id}",
			request {
				path { room_id: crate::OwnedRoomId }
				query { version: alloc::string::String }
				body {
					sessions: alloc::collections::BTreeMap<
						alloc::string::String,
						crate::sswire::Raw<crate::backup::KeyBackupData>
					>
				}
			}
			response { count: crate::UInt, etag: alloc::string::String }
		}
	}
}

pub mod add_backup_keys_for_session {
	pub mod v3 {
		crate::endpoint_request_raw! {
			method: "PUT", path: "/_matrix/client/v3/room_keys/keys/{room_id}/{session_id}",
			request {
				path { room_id: crate::OwnedRoomId, session_id: alloc::string::String }
				query { version: alloc::string::String }
				raw_body { session_data: crate::sswire::Raw<crate::backup::KeyBackupData> }
			}
		}
		crate::endpoint_response! { response { count: crate::UInt, etag: alloc::string::String } }
	}
}

pub mod get_backup_keys {
	pub mod v3 {
		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/room_keys/keys",
			request { path {} query { version: alloc::string::String } body {} }
			response {
				rooms: alloc::collections::BTreeMap<crate::OwnedRoomId, crate::backup::RoomKeyBackup>
			}
		}
	}
}

pub mod get_backup_keys_for_room {
	pub mod v3 {
		crate::endpoint! {
			method: "GET", path: "/_matrix/client/v3/room_keys/keys/{room_id}",
			request {
				path { room_id: crate::OwnedRoomId }
				query { version: alloc::string::String }
				body {}
			}
			response {
				sessions: alloc::collections::BTreeMap<
					alloc::string::String,
					crate::sswire::Raw<crate::backup::KeyBackupData>
				>
			}
		}
	}
}

pub mod get_backup_keys_for_session {
	pub mod v3 {
		crate::endpoint_request! {
			method: "GET", path: "/_matrix/client/v3/room_keys/keys/{room_id}/{session_id}",
			request {
				path { room_id: crate::OwnedRoomId, session_id: alloc::string::String }
				query { version: alloc::string::String }
				body {}
			}
		}
		crate::endpoint_response_flat!(key_data: crate::sswire::Raw<crate::backup::KeyBackupData>);
	}
}

pub mod delete_backup_keys {
	pub mod v3 {
		crate::endpoint! {
			method: "DELETE", path: "/_matrix/client/v3/room_keys/keys",
			request { path {} query { version: alloc::string::String } body {} }
			response { count: crate::UInt, etag: alloc::string::String }
		}
	}
}

pub mod delete_backup_keys_for_room {
	pub mod v3 {
		crate::endpoint! {
			method: "DELETE", path: "/_matrix/client/v3/room_keys/keys/{room_id}",
			request {
				path { room_id: crate::OwnedRoomId }
				query { version: alloc::string::String }
				body {}
			}
			response { count: crate::UInt, etag: alloc::string::String }
		}
	}
}

pub mod delete_backup_keys_for_session {
	pub mod v3 {
		crate::endpoint! {
			method: "DELETE", path: "/_matrix/client/v3/room_keys/keys/{room_id}/{session_id}",
			request {
				path { room_id: crate::OwnedRoomId, session_id: alloc::string::String }
				query { version: alloc::string::String }
				body {}
			}
			response { count: crate::UInt, etag: alloc::string::String }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{codec::from_str, endpoint::EndpointResponse};

	#[test]
	fn backup_info_flattens_algorithm() {
		let body = from_str::<Value>(
			r#"{"algorithm":"m.megolm_backup.v1.curve25519-aes-sha2","auth_data":{},
			"count":2,"etag":"7","version":"3"}"#,
		)
		.unwrap();
		let response = get_backup_info::v3::Response::from_body(&body).unwrap();
		assert_eq!(response.count, 2);
		let back = response.to_body();
		assert_eq!(
			back.get("algorithm").and_then(Value::as_str),
			Some("m.megolm_backup.v1.curve25519-aes-sha2")
		);
		assert_eq!(back.get("version").and_then(Value::as_str), Some("3"));
	}
}

//! Server-side encrypted key backup.

use alloc::{collections::BTreeMap, string::String};

use crate::{UInt, json::Value, sswire::Raw};

/// A backup algorithm and its public parameters.
#[derive(Debug)]
pub struct BackupAlgorithm {
	pub algorithm: String,
	pub auth_data: Value,
}

impl crate::codec::Serialize for BackupAlgorithm {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(algorithm), crate::endpoint::enc(&self.algorithm)),
			(stringify!(auth_data), crate::endpoint::enc(&self.auth_data)),
		])
	}
}
impl crate::codec::Deserialize for BackupAlgorithm {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(BackupAlgorithm)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			algorithm: input.body(stringify!(algorithm))?,
			auth_data: input.body(stringify!(auth_data))?,
		})
	}
}

/// One backed-up room key.
#[derive(Debug)]
pub struct KeyBackupData {
	pub first_message_index: UInt,
	pub forwarded_count: UInt,
	pub is_verified: bool,
	/// The encrypted session data.
	pub session_data: Value,
}

impl crate::codec::Serialize for KeyBackupData {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [
			(stringify!(first_message_index), crate::endpoint::enc(&self.first_message_index)),
			(stringify!(forwarded_count), crate::endpoint::enc(&self.forwarded_count)),
			(stringify!(is_verified), crate::endpoint::enc(&self.is_verified)),
			(stringify!(session_data), crate::endpoint::enc(&self.session_data)),
		])
	}
}
impl crate::codec::Deserialize for KeyBackupData {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(KeyBackupData)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			first_message_index: input.body(stringify!(first_message_index))?,
			forwarded_count: input.body(stringify!(forwarded_count))?,
			is_verified: input.body(stringify!(is_verified))?,
			session_data: input.body(stringify!(session_data))?,
		})
	}
}

/// The backed-up sessions of one room.
#[derive(Debug, Default)]
pub struct RoomKeyBackup {
	pub sessions: BTreeMap<String, Raw<KeyBackupData>>,
}

impl crate::codec::Serialize for RoomKeyBackup {
	fn to_json(&self) -> crate::json::Value {
		crate::endpoint::body_object(&mut [(
			stringify!(sessions),
			crate::endpoint::enc(&self.sessions),
		)])
	}
}
impl crate::codec::Deserialize for RoomKeyBackup {
	fn from_json(value: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
		// A struct is a JSON object; anything else is malformed, not "all defaults".
		if value.as_object().is_none() {
			return Err(crate::codec::DeError::expected(stringify!(RoomKeyBackup)));
		}
		let input = crate::endpoint::Input::body_only(value);
		Ok(Self {
			sessions: input.body(stringify!(sessions))?,
		})
	}
}

pub mod create_backup_version {
	pub mod v3 {
		pub struct Request {
			pub algorithm: crate::sswire::Raw<crate::backup::BackupAlgorithm>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("POST", "/_matrix/client/v3/room_keys/version");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("algorithm", crate::endpoint::enc(&self.algorithm))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					algorithm: input.body("algorithm")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					version: input.body("version")?,
				})
			}
		}
	}
}

pub mod update_backup_version {
	pub mod v3 {
		pub struct Request {
			pub version: alloc::string::String,
			pub algorithm: crate::sswire::Raw<crate::backup::BackupAlgorithm>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/v3/room_keys/version/{version}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.version)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				Some(crate::endpoint::enc(&self.algorithm))
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					version: input.path()?,
					algorithm: crate::codec::Deserialize::from_json(
						body.ok_or_else(|| crate::codec::DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [])
			}
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
		}
	}
}

pub mod get_latest_backup_info {
	pub mod v3 {
		pub struct Request {}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/room_keys/version");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {};
				input.finish()?;
				Ok(value)
			}
		}
		#[derive(Debug)]
		pub struct Response {
			pub algorithm: crate::sswire::Raw<crate::backup::BackupAlgorithm>,
			pub count: crate::UInt,
			pub etag: ::alloc::string::String,
			pub version: ::alloc::string::String,
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				let mut object = match crate::codec::Serialize::to_json(&self.algorithm) {
					crate::json::Value::Object(object) => object,
					_ => crate::json::Object::new(),
				};
				object.extend(crate::endpoint::object_from(::alloc::vec![
					("count", crate::codec::Serialize::to_json(&self.count)),
					("etag", crate::codec::Serialize::to_json(&self.etag)),
					("version", crate::codec::Serialize::to_json(&self.version)),
				]));
				crate::json::Value::Object(object)
			}
			fn from_body(body: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let input = crate::endpoint::Input::new(&[], &[], Some(body));
				Ok(Self {
					algorithm: crate::codec::Deserialize::from_json(body)?,
					count: input.body("count")?,
					etag: input.body("etag")?,
					version: input.body("version")?,
				})
			}
		}
	}
}

pub mod get_backup_info {
	pub mod v3 {
		pub struct Request {
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/room_keys/version/{version}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.version)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					version: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		#[derive(Debug)]
		pub struct Response {
			pub algorithm: crate::sswire::Raw<crate::backup::BackupAlgorithm>,
			pub count: crate::UInt,
			pub etag: ::alloc::string::String,
			pub version: ::alloc::string::String,
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				let mut object = match crate::codec::Serialize::to_json(&self.algorithm) {
					crate::json::Value::Object(object) => object,
					_ => crate::json::Object::new(),
				};
				object.extend(crate::endpoint::object_from(::alloc::vec![
					("count", crate::codec::Serialize::to_json(&self.count)),
					("etag", crate::codec::Serialize::to_json(&self.etag)),
					("version", crate::codec::Serialize::to_json(&self.version)),
				]));
				crate::json::Value::Object(object)
			}
			fn from_body(body: &crate::json::Value) -> Result<Self, crate::codec::DeError> {
				let input = crate::endpoint::Input::new(&[], &[], Some(body));
				Ok(Self {
					algorithm: crate::codec::Deserialize::from_json(body)?,
					count: input.body("count")?,
					etag: input.body("etag")?,
					version: input.body("version")?,
				})
			}
		}
	}
}

pub mod delete_backup_version {
	pub mod v3 {
		pub struct Request {
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"DELETE",
				"/_matrix/client/v3/room_keys/version/{version}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.version)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					version: input.path()?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [])
			}
			fn from_body(_body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {})
			}
		}
	}
}

pub mod add_backup_keys {
	pub mod v3 {
		pub struct Request {
			pub version: alloc::string::String,
			pub rooms:
				alloc::collections::BTreeMap<crate::OwnedRoomId, crate::backup::RoomKeyBackup>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("PUT", "/_matrix/client/v3/room_keys/keys");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("rooms", crate::endpoint::enc(&self.rooms))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					version: input.query("version")?,
					rooms: input.body("rooms")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub count: crate::UInt,
			pub etag: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("count", crate::endpoint::enc(&self.count)),
					("etag", crate::endpoint::enc(&self.etag)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					count: input.body("count")?,
					etag: input.body("etag")?,
				})
			}
		}
	}
}

pub mod add_backup_keys_for_room {
	pub mod v3 {
		pub struct Request {
			pub room_id: crate::OwnedRoomId,
			pub version: alloc::string::String,
			pub sessions: alloc::collections::BTreeMap<
				alloc::string::String,
				crate::sswire::Raw<crate::backup::KeyBackupData>,
			>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/v3/room_keys/keys/{room_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [("sessions", crate::endpoint::enc(&self.sessions))],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					version: input.query("version")?,
					sessions: input.body("sessions")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub count: crate::UInt,
			pub etag: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("count", crate::endpoint::enc(&self.count)),
					("etag", crate::endpoint::enc(&self.etag)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					count: input.body("count")?,
					etag: input.body("etag")?,
				})
			}
		}
	}
}

pub mod add_backup_keys_for_session {
	pub mod v3 {
		pub struct Request {
			pub room_id: crate::OwnedRoomId,
			pub session_id: alloc::string::String,
			pub version: alloc::string::String,
			pub session_data: crate::sswire::Raw<crate::backup::KeyBackupData>,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"PUT",
				"/_matrix/client/v3/room_keys/keys/{room_id}/{session_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.session_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				Some(crate::endpoint::enc(&self.session_data))
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					session_id: input.path()?,
					version: input.query("version")?,
					session_data: crate::codec::Deserialize::from_json(
						body.ok_or_else(|| crate::codec::DeError::expected("request body"))?,
					)?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub count: crate::UInt,
			pub etag: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("count", crate::endpoint::enc(&self.count)),
					("etag", crate::endpoint::enc(&self.etag)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					count: input.body("count")?,
					etag: input.body("etag")?,
				})
			}
		}
	}
}

pub mod get_backup_keys {
	pub mod v3 {
		pub struct Request {
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("GET", "/_matrix/client/v3/room_keys/keys");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					version: input.query("version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub rooms:
				alloc::collections::BTreeMap<crate::OwnedRoomId, crate::backup::RoomKeyBackup>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [("rooms", crate::endpoint::enc(&self.rooms))])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					rooms: input.body("rooms")?,
				})
			}
		}
	}
}

pub mod get_backup_keys_for_room {
	pub mod v3 {
		pub struct Request {
			pub room_id: crate::OwnedRoomId,
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/room_keys/keys/{room_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					version: input.query("version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub sessions: alloc::collections::BTreeMap<
				alloc::string::String,
				crate::sswire::Raw<crate::backup::KeyBackupData>,
			>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [(
					"sessions",
					crate::endpoint::enc(&self.sessions),
				)])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					sessions: input.body("sessions")?,
				})
			}
		}
	}
}

pub mod get_backup_keys_for_session {
	pub mod v3 {
		pub struct Request {
			pub room_id: crate::OwnedRoomId,
			pub session_id: alloc::string::String,
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"GET",
				"/_matrix/client/v3/room_keys/keys/{room_id}/{session_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.session_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					session_id: input.path()?,
					version: input.query("version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub key_data: crate::sswire::Raw<crate::backup::KeyBackupData>,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::enc(&self.key_data)
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				Ok(Self {
					key_data: crate::codec::Deserialize::from_json(body)?,
				})
			}
		}
	}
}

pub mod delete_backup_keys {
	pub mod v3 {
		pub struct Request {
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata =
				crate::endpoint::Metadata::new("DELETE", "/_matrix/client/v3/room_keys/keys");
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					version: input.query("version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub count: crate::UInt,
			pub etag: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("count", crate::endpoint::enc(&self.count)),
					("etag", crate::endpoint::enc(&self.etag)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					count: input.body("count")?,
					etag: input.body("etag")?,
				})
			}
		}
	}
}

pub mod delete_backup_keys_for_room {
	pub mod v3 {
		pub struct Request {
			pub room_id: crate::OwnedRoomId,
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"DELETE",
				"/_matrix/client/v3/room_keys/keys/{room_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [crate::endpoint::path_param(&self.room_id)])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					version: input.query("version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub count: crate::UInt,
			pub etag: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("count", crate::endpoint::enc(&self.count)),
					("etag", crate::endpoint::enc(&self.etag)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					count: input.body("count")?,
					etag: input.body("etag")?,
				})
			}
		}
	}
}

pub mod delete_backup_keys_for_session {
	pub mod v3 {
		pub struct Request {
			pub room_id: crate::OwnedRoomId,
			pub session_id: alloc::string::String,
			pub version: alloc::string::String,
		}
		impl ::core::fmt::Debug for Request {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Request")
			}
		}
		const _: crate::endpoint::Metadata =
			<Request as crate::endpoint::EndpointRequest>::METADATA;
		impl crate::endpoint::EndpointRequest for Request {
			type Response = Response;
			const METADATA: crate::endpoint::Metadata = crate::endpoint::Metadata::new(
				"DELETE",
				"/_matrix/client/v3/room_keys/keys/{room_id}/{session_id}",
			);
			fn path_args(&self) -> crate::endpoint::Strs {
				crate::endpoint::path_args_from(&mut [
					crate::endpoint::path_param(&self.room_id),
					crate::endpoint::path_param(&self.session_id),
				])
			}
			fn query(&self) -> crate::endpoint::Pairs {
				crate::endpoint::query_pairs_mut(&mut [(
					"version",
					crate::endpoint::enc(&self.version),
				)])
			}
			fn body(&self) -> Option<crate::json::Value> {
				crate::endpoint::body_value(
					<Self as crate::endpoint::EndpointRequest>::METADATA.method,
					&mut [],
				)
			}
			fn from_parts(
				path: &[crate::endpoint::Str],
				query: &[crate::endpoint::Pair],
				body: Option<&crate::json::Value>,
			) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::new(path, query, body);
				let value = Self {
					room_id: input.path()?,
					session_id: input.path()?,
					version: input.query("version")?,
				};
				input.finish()?;
				Ok(value)
			}
		}
		pub struct Response {
			pub count: crate::UInt,
			pub etag: alloc::string::String,
		}
		impl ::core::fmt::Debug for Response {
			fn fmt(&self, f: &mut crate::endpoint::Fmt<'_>) -> crate::endpoint::FmtResult {
				crate::endpoint::opaque_debug(f, "Response")
			}
		}
		impl crate::endpoint::EndpointResponse for Response {
			fn to_body(&self) -> crate::json::Value {
				crate::endpoint::body_object(&mut [
					("count", crate::endpoint::enc(&self.count)),
					("etag", crate::endpoint::enc(&self.etag)),
				])
			}
			fn from_body(body: &crate::json::Value) -> crate::endpoint::Parsed<Self> {
				let input = crate::endpoint::Input::body_only(body);
				Ok(Self {
					count: input.body("count")?,
					etag: input.body("etag")?,
				})
			}
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

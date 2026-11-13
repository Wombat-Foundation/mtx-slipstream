//! Small compatibility adapter backed by Rezzy's JSON DOM.

pub use rezzy::json;
pub use rezzy::json::{Error, Value as OwnedValue};

pub mod value {
	pub mod owned {
		pub use rezzy::json::Object;
	}
}

/// Parses `input` permissively (a repeated object key keeps its last value).
pub fn to_owned_value(input: &[u8]) -> Result<OwnedValue, Error> {
	OwnedValue::parse_bytes(input)
}

pub mod prelude {
	use super::OwnedValue;
	use std::io;

	/// Permissive (non-canonical) JSON output. Fine for unsigned responses such
	/// as sync; use `rezzy::json::to_canonical_bytes` for anything signed or hashed.
	pub trait Writable {
		fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
	}

	impl Writable for OwnedValue {
		fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
			let json = rezzy::json::write_string_value(self).map_err(|_| {
				io::Error::new(io::ErrorKind::InvalidData, "JSON serialization failed")
			})?;
			writer.write_all(json.as_bytes())
		}
	}
}

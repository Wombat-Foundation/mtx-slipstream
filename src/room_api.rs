//! Room-level client endpoints.

use crate::impl_codec_enum;

/// Whether a room is listed in the public directory.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Visibility {
	Public,
	#[default]
	Private,
}

impl_codec_enum!(Visibility { Public => "public", Private => "private" });

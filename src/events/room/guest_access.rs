#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GuestAccess { #[default] Forbidden, CanJoin }
crate::impl_codec_enum!(GuestAccess { Forbidden => "forbidden", CanJoin => "can_join" });
#[derive(Clone, Debug, Default)]
pub struct RoomGuestAccessEventContent { pub guest_access: GuestAccess }

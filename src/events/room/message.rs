#[derive(Clone, Debug, Default)]
pub struct RoomMessageEventContent { pub body: String }
impl RoomMessageEventContent {
	pub fn text_plain(body: impl Into<String>) -> Self { Self { body: body.into() } }
	pub fn text_markdown(body: impl Into<String>) -> Self { Self::text_plain(body) }
	pub fn notice_markdown(body: impl Into<String>) -> Self { Self::text_plain(body) }
}
#[derive(Clone, Debug)] pub enum MediaSource { Plain(String) }
#[derive(Clone, Debug)] pub enum MessageType { File(FileMessageEventContent) }
#[derive(Clone, Debug, Default)] pub struct FileInfo;
#[derive(Clone, Debug)] pub struct FileMessageEventContent { pub body: String, pub source: MediaSource, pub info: Option<FileInfo> }
#[derive(Clone, Debug)] pub enum Relation { None }

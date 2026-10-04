#[derive(Clone, Debug, Default)]
pub struct RoomMessageEventContent {
	pub body: String,
	pub msgtype: MessageType,
	pub relates_to: Option<Relation>,
	pub mentions: Option<crate::events::Mentions>,
}
impl RoomMessageEventContent {
	#[must_use]
	pub fn new(msgtype: MessageType) -> Self {
		let body = match &msgtype {
			MessageType::Text(body) | MessageType::Notice(body) => body.clone(),
			MessageType::File(file) => file.body.clone(),
		};
		Self {
			body,
			msgtype,
			relates_to: None,
			mentions: None,
		}
	}
	#[must_use]
	pub fn body(&self) -> &str {
		&self.body
	}
	#[must_use]
	pub fn add_mentions(mut self, mentions: crate::events::Mentions) -> Self {
		self.mentions = Some(mentions);
		self
	}
	pub fn text_plain(body: impl Into<String>) -> Self {
		let body = body.into();
		Self {
			body: body.clone(),
			msgtype: MessageType::Text(body),
			relates_to: None,
			mentions: None,
		}
	}
	pub fn text_markdown(body: impl Into<String>) -> Self {
		Self::text_plain(body)
	}
	pub fn notice_markdown(body: impl Into<String>) -> Self {
		Self::text_plain(body)
	}
}
#[derive(Clone, Debug)]
pub enum MediaSource {
	Plain(String),
}
#[derive(Clone, Debug)]
pub enum MessageType {
	Text(String),
	Notice(String),
	File(FileMessageEventContent),
}
impl Default for MessageType {
	fn default() -> Self {
		Self::Text(String::new())
	}
}
#[derive(Clone, Debug, Default)]
pub struct FileInfo {
	pub mimetype: Option<String>,
	pub size: Option<u64>,
	pub thumbnail_info: Option<()>,
	pub thumbnail_source: Option<MediaSource>,
}
#[derive(Clone, Debug)]
pub struct FileMessageEventContent {
	pub body: String,
	pub source: MediaSource,
	pub info: Option<FileInfo>,
	pub filename: Option<String>,
	pub formatted: Option<()>,
}
pub use crate::relation_types::Relation;

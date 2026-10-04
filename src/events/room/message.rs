//! `m.room.message` content.

use alloc::string::String;

pub use crate::relation_types::Relation;
use crate::{
	UInt,
	codec::{DeError, Deserialize, Serialize},
	endpoint::object_from,
	events::Mentions,
	json::{Object, Value},
};

/// How a formatted body is encoded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageFormat {
	Html,
	Custom(String),
}

impl MessageFormat {
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Html => "org.matrix.custom.html",
			Self::Custom(format) => format,
		}
	}
}

impl From<&str> for MessageFormat {
	fn from(format: &str) -> Self {
		if format == "org.matrix.custom.html" {
			Self::Html
		} else {
			Self::Custom(format.into())
		}
	}
}

/// A body in a rich format, such as HTML.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormattedBody {
	pub format: MessageFormat,
	pub body: String,
}

impl FormattedBody {
	#[must_use]
	pub fn html(body: impl Into<String>) -> Self {
		Self {
			format: MessageFormat::Html,
			body: body.into(),
		}
	}
}

/// A text or notice message body.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextMessageEventContent {
	pub body: String,
	pub formatted: Option<FormattedBody>,
}

impl TextMessageEventContent {
	#[must_use]
	pub fn plain(body: impl Into<String>) -> Self {
		Self {
			body: body.into(),
			formatted: None,
		}
	}

	#[must_use]
	pub fn html(body: impl Into<String>, html: impl Into<String>) -> Self {
		Self {
			body: body.into(),
			formatted: Some(FormattedBody::html(html)),
		}
	}
}

pub type NoticeMessageEventContent = TextMessageEventContent;

/// Where a file's bytes live.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MediaSource {
	Plain(String),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileInfo {
	pub mimetype: Option<String>,
	pub size: Option<UInt>,
	pub thumbnail_info: Option<()>,
	pub thumbnail_source: Option<MediaSource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileMessageEventContent {
	pub body: String,
	pub source: MediaSource,
	pub info: Option<FileInfo>,
	pub filename: Option<String>,
	pub formatted: Option<FormattedBody>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
	Text(TextMessageEventContent),
	Notice(NoticeMessageEventContent),
	File(FileMessageEventContent),
}

impl Default for MessageType {
	fn default() -> Self {
		Self::Text(TextMessageEventContent::default())
	}
}

impl MessageType {
	#[must_use]
	pub fn body(&self) -> &str {
		match self {
			Self::Text(text) | Self::Notice(text) => &text.body,
			Self::File(file) => &file.body,
		}
	}

	#[must_use]
	pub fn msgtype(&self) -> &'static str {
		match self {
			Self::Text(_) => "m.text",
			Self::Notice(_) => "m.notice",
			Self::File(_) => "m.file",
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct RoomMessageEventContent {
	pub msgtype: MessageType,
	pub relates_to: Option<Relation>,
	pub mentions: Option<Mentions>,
}

impl RoomMessageEventContent {
	#[must_use]
	pub fn new(msgtype: MessageType) -> Self {
		Self {
			msgtype,
			relates_to: None,
			mentions: None,
		}
	}

	#[must_use]
	pub fn body(&self) -> &str {
		self.msgtype.body()
	}

	#[must_use]
	pub fn add_mentions(mut self, mentions: Mentions) -> Self {
		self.mentions = Some(mentions);
		self
	}

	pub fn text_plain(body: impl Into<String>) -> Self {
		Self::new(MessageType::Text(TextMessageEventContent::plain(body)))
	}

	pub fn text_html(body: impl Into<String>, html: impl Into<String>) -> Self {
		Self::new(MessageType::Text(TextMessageEventContent::html(body, html)))
	}

	pub fn notice_plain(body: impl Into<String>) -> Self {
		Self::new(MessageType::Notice(TextMessageEventContent::plain(body)))
	}

	pub fn notice_html(body: impl Into<String>, html: impl Into<String>) -> Self {
		Self::new(MessageType::Notice(TextMessageEventContent::html(body, html)))
	}

	/// A text message whose body is Markdown source.
	///
	/// This crate has no Markdown renderer, so no HTML is attached; clients
	/// show the source. Use [`text_html`](Self::text_html) to attach HTML.
	pub fn text_markdown(body: impl Into<String>) -> Self {
		Self::text_plain(body)
	}

	/// A notice whose body is Markdown source; see [`text_markdown`](Self::text_markdown).
	pub fn notice_markdown(body: impl Into<String>) -> Self {
		Self::notice_plain(body)
	}
}

fn formatted_fields(
	formatted: Option<&FormattedBody>,
	fields: &mut alloc::vec::Vec<(&str, Value)>,
) {
	if let Some(formatted) = formatted {
		fields.push(("format", Value::String(formatted.format.as_str().into())));
		fields.push(("formatted_body", Value::String(formatted.body.clone())));
	}
}

fn read_formatted(object: &Object) -> Option<FormattedBody> {
	let format = object.get("format").and_then(Value::as_str)?;
	let body = object.get("formatted_body").and_then(Value::as_str)?;
	Some(FormattedBody {
		format: MessageFormat::from(format),
		body: body.into(),
	})
}

impl Serialize for FileInfo {
	fn to_json(&self) -> Value {
		Value::Object(object_from(alloc::vec![
			("mimetype", self.mimetype.to_json()),
			("size", self.size.to_json()),
		]))
	}
}

impl Deserialize for FileInfo {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("info object"))?;
		Ok(Self {
			mimetype: object.get("mimetype").and_then(Value::as_str).map(String::from),
			size: object.get("size").and_then(Value::as_u64),
			thumbnail_info: None,
			thumbnail_source: None,
		})
	}
}

impl Serialize for RoomMessageEventContent {
	fn to_json(&self) -> Value {
		let mut fields = alloc::vec![
			("msgtype", Value::String(self.msgtype.msgtype().into())),
			("body", Value::String(self.body().into())),
		];
		match &self.msgtype {
			MessageType::Text(text) | MessageType::Notice(text) => {
				formatted_fields(text.formatted.as_ref(), &mut fields);
			}
			MessageType::File(file) => {
				formatted_fields(file.formatted.as_ref(), &mut fields);
				let MediaSource::Plain(url) = &file.source;
				fields.push(("url", Value::String(url.clone())));
				fields.push(("info", file.info.to_json()));
				fields.push(("filename", file.filename.to_json()));
			}
		}
		fields.push(("m.mentions", self.mentions.to_json()));
		fields.push(("m.relates_to", self.relates_to.to_json()));
		Value::Object(object_from(fields))
	}
}

impl Deserialize for RoomMessageEventContent {
	fn from_json(value: &Value) -> Result<Self, DeError> {
		let object = value.as_object().ok_or_else(|| DeError::expected("message object"))?;
		let body: String = object
			.get("body")
			.and_then(Value::as_str)
			.ok_or_else(|| DeError::expected("body"))?
			.into();
		let formatted = read_formatted(object);
		let url = object.get("url").and_then(Value::as_str);
		let msgtype = match (object.get("msgtype").and_then(Value::as_str), url) {
			(Some("m.notice"), _) => MessageType::Notice(TextMessageEventContent {
				body,
				formatted,
			}),
			(Some("m.file"), Some(url)) => MessageType::File(FileMessageEventContent {
				body,
				source: MediaSource::Plain(url.into()),
				info: object
					.get("info")
					.filter(|v| !v.is_null())
					.map(FileInfo::from_json)
					.transpose()?,
				filename: object.get("filename").and_then(Value::as_str).map(String::from),
				formatted,
			}),
			_ => MessageType::Text(TextMessageEventContent {
				body,
				formatted,
			}),
		};
		let present = |name: &str| object.get(name).filter(|v| !v.is_null());
		Ok(Self {
			msgtype,
			relates_to: present("m.relates_to").map(Relation::from_json).transpose()?,
			mentions: present("m.mentions").map(Mentions::from_json).transpose()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::codec::{from_str, to_string};

	#[test]
	fn text_message_round_trips_with_html_and_mentions() {
		let mut content = RoomMessageEventContent::text_html("hi", "<b>hi</b>")
			.add_mentions(Mentions::with_room_mention());
		content.relates_to = None;
		let text = to_string(&content);
		assert!(text.contains("\"msgtype\":\"m.text\"") && text.contains("formatted_body"));
		let again = from_str::<RoomMessageEventContent>(&text).unwrap();
		assert_eq!(again.msgtype, content.msgtype);
		assert!(again.mentions.unwrap().room);
	}

	#[test]
	fn notice_and_file_messages() {
		let notice = RoomMessageEventContent::notice_plain("n");
		assert!(to_string(&notice).contains("m.notice"));
		let file = RoomMessageEventContent::new(MessageType::File(FileMessageEventContent {
			body: "b".into(),
			source: MediaSource::Plain("mxc://x/y".into()),
			info: Some(FileInfo {
				mimetype: Some("text/plain".into()),
				size: Some(4),
				..FileInfo::default()
			}),
			filename: Some("o.md".into()),
			formatted: None,
		}));
		let again = from_str::<RoomMessageEventContent>(&to_string(&file)).unwrap();
		assert_eq!(again.msgtype, file.msgtype);
	}
}

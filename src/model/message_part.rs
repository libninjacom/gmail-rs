use serde::{Serialize, Deserialize};
use super::{Header, MessagePartBody};
///A single MIME message part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePart {
    ///The message part body for this part, which may be empty for container MIME message parts.
    pub body: MessagePartBody,
    ///The filename of the attachment. Only present if this message part represents an attachment.
    pub filename: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<Header>,
    ///The MIME type of the message part.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    ///The immutable ID of the message part.
    #[serde(rename = "partId")]
    pub part_id: String,
    ///The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<MessagePart>,
}

impl MessagePart {
    pub fn find_mime(&self, mime_type: &str) -> Option<&MessagePart> {
        if self.mime_type == mime_type {
            Some(self)
        } else if self.mime_type.starts_with("multipart/") {
            self.parts.iter().find_map(|part| part.find_mime(mime_type))
        } else {
            None
        }
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find_map(|header| {
            if header.name == name {
                Some(header.value.as_str())
            } else {
                None
            }
        })
    }

    pub fn find(&self, f: impl Fn(&MessagePart) -> bool + Copy + 'static) -> Option<&MessagePart> {
        if f(self) {
            Some(self)
        } else if self.mime_type.starts_with("multipart/") {
            self.parts.iter().find_map(|part| part.find(f))
        } else {
            None
        }
    }
}

impl std::fmt::Display for MessagePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
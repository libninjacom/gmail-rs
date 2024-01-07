use serde::{Serialize, Deserialize};
use super::Header;
///A single MIME message part.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessagePart {
    ///The message part body for this part, which may be empty for container MIME message parts.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub body: serde_json::Value,
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
impl std::fmt::Display for MessagePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
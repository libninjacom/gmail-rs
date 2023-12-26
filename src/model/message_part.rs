use serde::{Serialize, Deserialize};
use super::MessagePart;
///A single MIME message part.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessagePart {
    ///The message part body for this part, which may be empty for container MIME message parts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    ///The filename of the attachment. Only present if this message part represents an attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<serde_json::Value>>,
    ///The MIME type of the message part.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///The immutable ID of the message part.
    #[serde(rename = "partId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    ///The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<MessagePart>>,
}
impl std::fmt::Display for MessagePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
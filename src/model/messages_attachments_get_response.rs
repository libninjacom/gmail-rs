use serde::{Serialize, Deserialize};
///The body of a single MIME message part.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessagesAttachmentsGetResponse {
    ///When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.
    #[serde(rename = "attachmentId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    ///The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    ///Number of bytes for the message part data (encoding notwithstanding).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
impl std::fmt::Display for MessagesAttachmentsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
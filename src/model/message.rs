use serde::{Serialize, Deserialize};
use super::MessagePart;
///An email message.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    ///The ID of the last history record that modified this message.
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
    ///The immutable ID of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.
    #[serde(rename = "internalDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_date: Option<String>,
    ///List of IDs of labels applied to this message.
    #[serde(rename = "labelIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<String>>,
    ///A single MIME message part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<MessagePart>,
    ///The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    ///Estimated size in bytes of the message.
    #[serde(rename = "sizeEstimate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<i64>,
    ///A short part of the message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    ///The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
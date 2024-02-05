use serde::{Serialize, Deserialize};
use super::MessagePart;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageCompact {
    id: String,
    #[serde(default, rename = "labelIds")]
    label_ids: Vec<String>,
    #[serde(rename = "threadId")]
    thread_id: String,
}

///An email message.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    ///The ID of the last history record that modified this message.
    #[serde(rename = "historyId")]
    pub history_id: String,
    ///The immutable ID of the message.
    pub id: String,
    ///The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.
    #[serde(rename = "internalDate")]
    pub internal_date: String,
    ///List of IDs of labels applied to this message.
    #[serde(rename = "labelIds")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label_ids: Vec<String>,
    ///A single MIME message part.
    pub payload: MessagePart,
    ///The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    ///Estimated size in bytes of the message.
    #[serde(rename = "sizeEstimate")]
    pub size_estimate: i64,
    ///A short part of the message text.
    pub snippet: String,
    ///The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.
    #[serde(rename = "threadId")]
    pub thread_id: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
use serde::{Deserialize, Serialize};

use super::CompactMessage;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CompactMessageContainer {
    pub message: CompactMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LabelMessageContainer {
    #[serde(rename = "labelIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<String>>,

    pub message: CompactMessage,
}

///A record of a change to the user's mailbox. Each history change may affect multiple messages in multiple ways.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    ///The mailbox sequence ID.
    pub id: String,
    ///Labels added to messages in this history record.
    #[serde(rename = "labelsAdded")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels_added: Vec<LabelMessageContainer>,
    ///Labels removed from messages in this history record.
    #[serde(rename = "labelsRemoved")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels_removed: Vec<LabelMessageContainer>,
    ///List of messages changed in this history record. The fields for specific change types, such as `messagesAdded` may duplicate messages in this field. We recommend using the specific change-type fields instead of this.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<CompactMessage>,
    ///Messages added to the mailbox in this history record.
    #[serde(rename = "messagesAdded")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages_added: Vec<CompactMessageContainer>,
    ///Messages deleted (not Trashed) from the mailbox in this history record.
    #[serde(rename = "messagesDeleted")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages_deleted: Vec<CompactMessageContainer>,
}
impl std::fmt::Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
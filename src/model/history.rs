use serde::{Serialize, Deserialize};
use super::{
    HistoryLabelAdded, HistoryLabelRemoved, HistoryMessageAdded, HistoryMessageDeleted,
    Message,
};
///A record of a change to the user's mailbox. Each history change may affect multiple messages in multiple ways.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    ///The mailbox sequence ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Labels added to messages in this history record.
    #[serde(rename = "labelsAdded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_added: Option<Vec<HistoryLabelAdded>>,
    ///Labels removed from messages in this history record.
    #[serde(rename = "labelsRemoved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_removed: Option<Vec<HistoryLabelRemoved>>,
    ///List of messages changed in this history record. The fields for specific change types, such as `messagesAdded` may duplicate messages in this field. We recommend using the specific change-type fields instead of this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    ///Messages added to the mailbox in this history record.
    #[serde(rename = "messagesAdded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_added: Option<Vec<HistoryMessageAdded>>,
    ///Messages deleted (not Trashed) from the mailbox in this history record.
    #[serde(rename = "messagesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_deleted: Option<Vec<HistoryMessageDeleted>>,
}
impl std::fmt::Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
use serde::{Serialize, Deserialize};
use super::Message;
///A collection of messages representing a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Thread {
    ///The ID of the last history record that modified this thread.
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
    ///The unique ID of the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The list of messages in the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    ///A short part of the message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}
impl std::fmt::Display for Thread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
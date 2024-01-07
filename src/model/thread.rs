use serde::{Serialize, Deserialize};
use super::Message;
///A collection of messages representing a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Thread {
    ///The ID of the last history record that modified this thread.
    #[serde(rename = "historyId")]
    pub history_id: String,
    ///The unique ID of the thread.
    pub id: String,
    ///The list of messages in the thread.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,
    ///A short part of the message text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}
impl std::fmt::Display for Thread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
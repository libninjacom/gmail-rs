use serde::{Serialize, Deserialize};
use super::Message;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryLabelRemoved {
    ///Label IDs removed from the message.
    #[serde(rename = "labelIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<String>>,
    ///An email message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}
impl std::fmt::Display for HistoryLabelRemoved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
use serde::{Serialize, Deserialize};
use super::Message;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryMessageDeleted {
    ///An email message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}
impl std::fmt::Display for HistoryMessageDeleted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
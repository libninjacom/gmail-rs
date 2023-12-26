use serde::{Serialize, Deserialize};
use super::Message;
///A draft email in the user's mailbox.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Draft {
    ///The immutable ID of the draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///An email message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}
impl std::fmt::Display for Draft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
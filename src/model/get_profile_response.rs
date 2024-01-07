use serde::{Serialize, Deserialize};
///Profile for a Gmail user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProfileResponse {
    ///The user's email address.
    #[serde(rename = "emailAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The ID of the mailbox's current history record.
    #[serde(rename = "historyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
    ///The total number of messages in the mailbox.
    #[serde(rename = "messagesTotal")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_total: Option<i64>,
    ///The total number of threads in the mailbox.
    #[serde(rename = "threadsTotal")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads_total: Option<i64>,
}
impl std::fmt::Display for GetProfileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
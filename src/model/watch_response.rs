use serde::{Serialize, Deserialize};
///Push notification watch response.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchResponse {
    ///When Gmail will stop sending notifications for mailbox updates (epoch millis). Call `watch` again before this time to renew the watch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    ///The ID of the mailbox's current history record.
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
}
impl std::fmt::Display for WatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
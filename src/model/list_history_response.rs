use serde::{Serialize, Deserialize};
use super::History;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListHistoryResponse {
    ///List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<History>>,
    ///The ID of the mailbox's current history record.
    #[serde(rename = "historyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
    ///Page token to retrieve the next page of results in the list.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
impl std::fmt::Display for ListHistoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
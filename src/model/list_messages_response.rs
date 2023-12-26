use serde::{Serialize, Deserialize};
use super::Message;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListMessagesResponse {
    ///List of messages. Note that each message resource contains only an `id` and a `threadId`. Additional message details can be fetched using the messages.get method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    ///Token to retrieve the next page of results in the list.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    ///Estimated total number of results.
    #[serde(rename = "resultSizeEstimate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_size_estimate: Option<i64>,
}
impl std::fmt::Display for ListMessagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
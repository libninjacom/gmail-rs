use serde::{Serialize, Deserialize};
use super::Draft;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDraftsResponse {
    ///List of drafts. Note that the `Message` property in each `Draft` resource only contains an `id` and a `threadId`. The messages.get method can fetch additional message details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drafts: Option<Vec<Draft>>,
    ///Token to retrieve the next page of results in the list.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    ///Estimated total number of results.
    #[serde(rename = "resultSizeEstimate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_size_estimate: Option<i64>,
}
impl std::fmt::Display for ListDraftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
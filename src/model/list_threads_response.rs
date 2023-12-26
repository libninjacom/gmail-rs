use serde::{Serialize, Deserialize};
use super::Thread;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListThreadsResponse {
    ///Page token to retrieve the next page of results in the list.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    ///Estimated total number of results.
    #[serde(rename = "resultSizeEstimate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_size_estimate: Option<i64>,
    ///List of threads. Note that each thread resource does not contain a list of `messages`. The list of `messages` for a given thread can be fetched using the threads.get method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<Thread>>,
}
impl std::fmt::Display for ListThreadsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
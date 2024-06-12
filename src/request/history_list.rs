use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::history_list`].

On request success, this will return a [`ListHistoryResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListRequest {
    pub history_types: Option<Vec<String>>,
    pub label_id: Option<String>,
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
    pub start_history_id: Option<String>,
    pub user_id: String,
}
impl HistoryListRequest {}
impl FluentRequest<'_, HistoryListRequest> {
    pub fn history_types(
        mut self,
        history_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .history_types = Some(
            history_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn label_id(mut self, label_id: &str) -> Self {
        self.params.label_id = Some(label_id.to_owned());
        self
    }
    pub fn max_results(mut self, max_results: i64) -> Self {
        self.params.max_results = Some(max_results);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.params.page_token = Some(page_token.to_owned());
        self
    }
    pub fn start_history_id(mut self, start_history_id: &str) -> Self {
        self.params.start_history_id = Some(start_history_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, HistoryListRequest> {
    type Output = httpclient::InMemoryResult<ListHistoryResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/history", user_id = self.params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}

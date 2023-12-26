use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::messages_list`].

On request success, this will return a [`ListMessagesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesListRequest {
    pub include_spam_trash: Option<bool>,
    pub label_ids: Option<Vec<String>>,
    pub max_results: Option<i64>,
    pub page_token: Option<String>,
    pub q: Option<String>,
    pub user_id: String,
}
impl MessagesListRequest {}
impl FluentRequest<'_, MessagesListRequest> {
    pub fn include_spam_trash(mut self, include_spam_trash: bool) -> Self {
        self.params.include_spam_trash = Some(include_spam_trash);
        self
    }
    pub fn label_ids(
        mut self,
        label_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .label_ids = Some(
            label_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
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
    pub fn q(mut self, q: &str) -> Self {
        self.params.q = Some(q.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesListRequest> {
    type Output = httpclient::InMemoryResult<ListMessagesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/messages", user_id = self.params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
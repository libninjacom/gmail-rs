use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::threads_get`].

On request success, this will return a [`Thread`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadsGetRequest {
    pub format: Option<String>,
    pub id: String,
    pub metadata_headers: Option<Vec<String>>,
    pub user_id: String,
}
impl ThreadsGetRequest {}
impl FluentRequest<'_, ThreadsGetRequest> {
    pub fn format(mut self, format: &str) -> Self {
        self.params.format = Some(format.to_owned());
        self
    }
    pub fn metadata_headers(
        mut self,
        metadata_headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .metadata_headers = Some(
            metadata_headers.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ThreadsGetRequest> {
    type Output = httpclient::InMemoryResult<Thread>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/threads/{id}", id = self.params.id, user_id =
                self.params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
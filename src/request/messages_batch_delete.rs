use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::messages_batch_delete`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesBatchDeleteRequest {
    pub ids: Option<Vec<String>>,
    pub user_id: String,
}
impl MessagesBatchDeleteRequest {}
impl FluentRequest<'_, MessagesBatchDeleteRequest> {
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.params.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesBatchDeleteRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/messages/batchDelete", user_id = self.params
                .user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.ids {
                r = r.json(json!({ "ids" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
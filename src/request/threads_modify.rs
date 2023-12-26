use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::threads_modify`].

On request success, this will return a [`Thread`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadsModifyRequest {
    pub add_label_ids: Option<Vec<String>>,
    pub id: String,
    pub remove_label_ids: Option<Vec<String>>,
    pub user_id: String,
}
impl ThreadsModifyRequest {}
impl FluentRequest<'_, ThreadsModifyRequest> {
    pub fn add_label_ids(
        mut self,
        add_label_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .add_label_ids = Some(
            add_label_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn remove_label_ids(
        mut self,
        remove_label_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .remove_label_ids = Some(
            remove_label_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ThreadsModifyRequest> {
    type Output = httpclient::InMemoryResult<Thread>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/threads/{id}/modify", id = self.params.id,
                user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.add_label_ids {
                r = r.json(json!({ "addLabelIds" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.remove_label_ids {
                r = r.json(json!({ "removeLabelIds" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::labels_get`].

On request success, this will return a [`LabelsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelsGetRequest {
    pub id: String,
    pub user_id: String,
}
impl LabelsGetRequest {}
impl FluentRequest<'_, LabelsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LabelsGetRequest> {
    type Output = httpclient::InMemoryResult<LabelsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/labels/{id}", id = self.params.id, user_id =
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
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::messages_attachments_get`].

On request success, this will return a [`MessagesAttachmentsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesAttachmentsGetRequest {
    pub id: String,
    pub message_id: String,
    pub user_id: String,
}
impl MessagesAttachmentsGetRequest {}
impl FluentRequest<'_, MessagesAttachmentsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesAttachmentsGetRequest> {
    type Output = httpclient::InMemoryResult<MessagesAttachmentsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/messages/{message_id}/attachments/{id}", id =
                self.params.id, message_id = self.params.message_id, user_id = self
                .params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
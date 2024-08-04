use std::vec;

use base64::{engine::general_purpose, Engine as _};

use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::{InMemoryBody, InMemoryResponseExt};
use crate::GmailClient;
/**You should use this struct via [`GmailClient::messages_send`].

On request success, this will return a [`Message`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesSendRequest {
    pub user_id: String,
    pub message: InMemoryBody,
    // TODO(joey): docs say The special value `me` can be used to indicate the authenticated user. do we wana default to that?
}
impl MessagesSendRequest {}
impl FluentRequest<'_, MessagesSendRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesSendRequest> {
    type Output = httpclient::InMemoryResult<CompactMessage>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/upload/gmail/v1/users/{user_id}/messages/send", user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url)
                .content_type("message/rfc822")
                .body(self.params.message);
            r = self.client.authenticate(r);
            let res = r.await?;
            dbg!(res.body());
            res.json().map_err(Into::into)
        })
    }
}

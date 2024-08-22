use std::vec;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use crate::model::*;
use crate::FluentRequest;
use crate::GmailClient;
use httpclient::{InMemoryBody, InMemoryResponseExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
/**You should use this struct via [`GmailClient::messages_send`].

On request success, this will return a [`Message`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesSendRequest {
    pub user_id: String,
    pub message: InMemoryBody,
    pub thread_id: Option<String>,
}
impl MessagesSendRequest {}
impl FluentRequest<'_, MessagesSendRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesSendRequest> {
    type Output = httpclient::InMemoryResult<CompactMessage>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let mut r = if let Some(thread_id) = &self.params.thread_id {
                let url = format!("/gmail/v1/users/{}/messages/send", self.params.user_id);
                let encoded_message = URL_SAFE.encode(self.params.message.bytes().unwrap());
                self.client.client.post(&url)
                    .json(json!({"raw": encoded_message, "threadId": thread_id}))
            } else {
                let url = format!("/upload/gmail/v1/users/{}/messages/send", self.params.user_id);
                self.client.client.post(&url)
                    .content_type("message/rfc822")
                    .body(self.params.message)
            };

            r = self.client.authenticate(r);
            let res = r.await?;
            dbg!(res.body());
            res.json().map_err(Into::into)
        })
    }
}

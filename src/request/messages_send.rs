use std::vec;

use base64::{engine::general_purpose, Engine as _};

use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::messages_send`].

On request success, this will return a [`Message`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesSendRequest {
    pub user_id: String,
    pub message: String,
    // TODO(joey): docs say The special value `me` can be used to indicate the authenticated user. do we wana default to that?
}
impl MessagesSendRequest {}
impl FluentRequest<'_, MessagesSendRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesSendRequest> {
    type Output = httpclient::InMemoryResult<Message>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/messages/send", user_id = self.params.user_id
            );
            dbg!(url);
            let mut r = self.client.client.post(url);
            // r = r.set_query(self.params);

            // r = r.body(httpclient::InMemoryBody::Json(json!({"message": {"raw": self.params.message}})));            
            r = r.body(httpclient::InMemoryBody::Json(json!({"raw": self.params.message})));            
            // r = r.body(httpclient::InMemoryBody::Text(json!({"message": {"raw": self.params.message}}).to_string()));            

            // dbg!(&r);
            
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}

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
}
impl MessagesSendRequest {}
impl FluentRequest<'_, MessagesSendRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesSendRequest> {
    type Output = httpclient::InMemoryResult<CompactMessage>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/messages/send",
                user_id = self.params.user_id
            );

            // let message_bytes = self.params.message.bytes().unwrap();
            // let encoded_body= URL_SAFE.encode(httpclient::InMemoryBody::Json(
            //     json!(
            //                 {
            //                     "raw": self.params.message,
            //                     "threadId": "<CANYxo_FSfNqjc3OWaE2dWjoBh_J5D_pey8b8z-_iFhC-wDyv_w@mail.gmail.com>"
            //                 }
            //          )
            //     ).bytes().unwrap()
            // );
            // let mut r = self
            //     .client
            //     .client
            //     .post(url)
            //     .body(httpclient::InMemoryBody::Text(encoded_body));
            // dbg!(&r);



            // let encoded_message = URL_SAFE.encode(self.params.message.bytes().unwrap());
            // let mut r = self.client.client.post(url)
            //     .body(httpclient::InMemoryBody::Json(json!({"raw": encoded_message, "threadId": "<CANYxo_FSfNqjc3OWaE2dWjoBh_J5D_pey8b8z-_iFhC-wDyv_w@mail.gmail.com>"})));
            // dbg!(&r);

            // r = self.client.authenticate(r);
            // let res = r.await?;
            // dbg!(res.body());
            // res.json().map_err(Into::into)

            
            let encoded_message = URL_SAFE.encode(self.params.message.bytes().unwrap());
            let mut r = self.client.client.post(url)
                .json(json!({"raw": encoded_message, "threadId": "1913f56101148ff9"}));
            dbg!(&r);

            r = self.client.authenticate(r);
            let res = r.await?;
            dbg!(res.body());
            res.json().map_err(Into::into)

        })
    }
}

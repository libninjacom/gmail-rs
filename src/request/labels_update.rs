use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::labels_update`].

On request success, this will return a [`LabelsUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelsUpdateRequest {
    pub color: Option<serde_json::Value>,
    pub id: String,
    pub label_list_visibility: Option<String>,
    pub message_list_visibility: Option<String>,
    pub messages_total: Option<i64>,
    pub messages_unread: Option<i64>,
    pub name: Option<String>,
    pub threads_total: Option<i64>,
    pub threads_unread: Option<i64>,
    pub type_: Option<String>,
    pub user_id: String,
}
impl LabelsUpdateRequest {}
impl FluentRequest<'_, LabelsUpdateRequest> {
    pub fn color(mut self, color: serde_json::Value) -> Self {
        self.params.color = Some(color);
        self
    }
    pub fn label_list_visibility(mut self, label_list_visibility: &str) -> Self {
        self.params.label_list_visibility = Some(label_list_visibility.to_owned());
        self
    }
    pub fn message_list_visibility(mut self, message_list_visibility: &str) -> Self {
        self.params.message_list_visibility = Some(message_list_visibility.to_owned());
        self
    }
    pub fn messages_total(mut self, messages_total: i64) -> Self {
        self.params.messages_total = Some(messages_total);
        self
    }
    pub fn messages_unread(mut self, messages_unread: i64) -> Self {
        self.params.messages_unread = Some(messages_unread);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.params.name = Some(name.to_owned());
        self
    }
    pub fn threads_total(mut self, threads_total: i64) -> Self {
        self.params.threads_total = Some(threads_total);
        self
    }
    pub fn threads_unread(mut self, threads_unread: i64) -> Self {
        self.params.threads_unread = Some(threads_unread);
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LabelsUpdateRequest> {
    type Output = httpclient::InMemoryResult<LabelsUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/labels/{id}", id = self.params.id, user_id =
                self.params.user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.color {
                r = r.json(json!({ "color" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.label_list_visibility {
                r = r.json(json!({ "labelListVisibility" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.message_list_visibility {
                r = r.json(json!({ "messageListVisibility" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.messages_total {
                r = r.json(json!({ "messagesTotal" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.messages_unread {
                r = r.json(json!({ "messagesUnread" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.name {
                r = r.json(json!({ "name" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.threads_total {
                r = r.json(json!({ "threadsTotal" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.threads_unread {
                r = r.json(json!({ "threadsUnread" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.type_ {
                r = r.json(json!({ "type" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
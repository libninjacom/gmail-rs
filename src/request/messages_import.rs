use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::messages_import`].

On request success, this will return a [`Message`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesImportRequest {
    pub deleted: Option<bool>,
    pub internal_date_source: Option<String>,
    pub never_mark_spam: Option<bool>,
    pub process_for_calendar: Option<bool>,
    pub user_id: String,
}
impl MessagesImportRequest {}
impl FluentRequest<'_, MessagesImportRequest> {
    pub fn deleted(mut self, deleted: bool) -> Self {
        self.params.deleted = Some(deleted);
        self
    }
    pub fn internal_date_source(mut self, internal_date_source: &str) -> Self {
        self.params.internal_date_source = Some(internal_date_source.to_owned());
        self
    }
    pub fn never_mark_spam(mut self, never_mark_spam: bool) -> Self {
        self.params.never_mark_spam = Some(never_mark_spam);
        self
    }
    pub fn process_for_calendar(mut self, process_for_calendar: bool) -> Self {
        self.params.process_for_calendar = Some(process_for_calendar);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, MessagesImportRequest> {
    type Output = httpclient::InMemoryResult<Message>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/messages/import", user_id = self.params
                .user_id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
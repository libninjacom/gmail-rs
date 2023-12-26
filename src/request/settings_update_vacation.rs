use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_update_vacation`].

On request success, this will return a [`SettingsUpdateVacationResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUpdateVacationRequest {
    pub enable_auto_reply: Option<bool>,
    pub end_time: Option<String>,
    pub response_body_html: Option<String>,
    pub response_body_plain_text: Option<String>,
    pub response_subject: Option<String>,
    pub restrict_to_contacts: Option<bool>,
    pub restrict_to_domain: Option<bool>,
    pub start_time: Option<String>,
    pub user_id: String,
}
impl SettingsUpdateVacationRequest {}
impl FluentRequest<'_, SettingsUpdateVacationRequest> {
    pub fn enable_auto_reply(mut self, enable_auto_reply: bool) -> Self {
        self.params.enable_auto_reply = Some(enable_auto_reply);
        self
    }
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.params.end_time = Some(end_time.to_owned());
        self
    }
    pub fn response_body_html(mut self, response_body_html: &str) -> Self {
        self.params.response_body_html = Some(response_body_html.to_owned());
        self
    }
    pub fn response_body_plain_text(mut self, response_body_plain_text: &str) -> Self {
        self.params.response_body_plain_text = Some(response_body_plain_text.to_owned());
        self
    }
    pub fn response_subject(mut self, response_subject: &str) -> Self {
        self.params.response_subject = Some(response_subject.to_owned());
        self
    }
    pub fn restrict_to_contacts(mut self, restrict_to_contacts: bool) -> Self {
        self.params.restrict_to_contacts = Some(restrict_to_contacts);
        self
    }
    pub fn restrict_to_domain(mut self, restrict_to_domain: bool) -> Self {
        self.params.restrict_to_domain = Some(restrict_to_domain);
        self
    }
    pub fn start_time(mut self, start_time: &str) -> Self {
        self.params.start_time = Some(start_time.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsUpdateVacationRequest> {
    type Output = httpclient::InMemoryResult<SettingsUpdateVacationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/vacation", user_id = self.params
                .user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.enable_auto_reply {
                r = r.json(json!({ "enableAutoReply" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_time {
                r = r.json(json!({ "endTime" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.response_body_html {
                r = r.json(json!({ "responseBodyHtml" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.response_body_plain_text {
                r = r.json(json!({ "responseBodyPlainText" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.response_subject {
                r = r.json(json!({ "responseSubject" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.restrict_to_contacts {
                r = r.json(json!({ "restrictToContacts" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.restrict_to_domain {
                r = r.json(json!({ "restrictToDomain" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_time {
                r = r.json(json!({ "startTime" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_update_auto_forwarding`].

On request success, this will return a [`SettingsUpdateAutoForwardingResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUpdateAutoForwardingRequest {
    pub disposition: Option<String>,
    pub email_address: Option<String>,
    pub enabled: Option<bool>,
    pub user_id: String,
}
impl SettingsUpdateAutoForwardingRequest {}
impl FluentRequest<'_, SettingsUpdateAutoForwardingRequest> {
    pub fn disposition(mut self, disposition: &str) -> Self {
        self.params.disposition = Some(disposition.to_owned());
        self
    }
    pub fn email_address(mut self, email_address: &str) -> Self {
        self.params.email_address = Some(email_address.to_owned());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.params.enabled = Some(enabled);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsUpdateAutoForwardingRequest> {
    type Output = httpclient::InMemoryResult<SettingsUpdateAutoForwardingResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/autoForwarding", user_id = self
                .params.user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.disposition {
                r = r.json(json!({ "disposition" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.email_address {
                r = r.json(json!({ "emailAddress" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.enabled {
                r = r.json(json!({ "enabled" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
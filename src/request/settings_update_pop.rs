use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_update_pop`].

On request success, this will return a [`SettingsUpdatePopResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUpdatePopRequest {
    pub access_window: Option<String>,
    pub disposition: Option<String>,
    pub user_id: String,
}
impl SettingsUpdatePopRequest {}
impl FluentRequest<'_, SettingsUpdatePopRequest> {
    pub fn access_window(mut self, access_window: &str) -> Self {
        self.params.access_window = Some(access_window.to_owned());
        self
    }
    pub fn disposition(mut self, disposition: &str) -> Self {
        self.params.disposition = Some(disposition.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsUpdatePopRequest> {
    type Output = httpclient::InMemoryResult<SettingsUpdatePopResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/pop", user_id = self.params.user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.access_window {
                r = r.json(json!({ "accessWindow" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.disposition {
                r = r.json(json!({ "disposition" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
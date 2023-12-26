use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_update_imap`].

On request success, this will return a [`SettingsUpdateImapResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUpdateImapRequest {
    pub auto_expunge: Option<bool>,
    pub enabled: Option<bool>,
    pub expunge_behavior: Option<String>,
    pub max_folder_size: Option<i64>,
    pub user_id: String,
}
impl SettingsUpdateImapRequest {}
impl FluentRequest<'_, SettingsUpdateImapRequest> {
    pub fn auto_expunge(mut self, auto_expunge: bool) -> Self {
        self.params.auto_expunge = Some(auto_expunge);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.params.enabled = Some(enabled);
        self
    }
    pub fn expunge_behavior(mut self, expunge_behavior: &str) -> Self {
        self.params.expunge_behavior = Some(expunge_behavior.to_owned());
        self
    }
    pub fn max_folder_size(mut self, max_folder_size: i64) -> Self {
        self.params.max_folder_size = Some(max_folder_size);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsUpdateImapRequest> {
    type Output = httpclient::InMemoryResult<SettingsUpdateImapResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/imap", user_id = self.params.user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.auto_expunge {
                r = r.json(json!({ "autoExpunge" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.enabled {
                r = r.json(json!({ "enabled" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.expunge_behavior {
                r = r.json(json!({ "expungeBehavior" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.max_folder_size {
                r = r.json(json!({ "maxFolderSize" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
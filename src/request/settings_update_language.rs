use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_update_language`].

On request success, this will return a [`SettingsUpdateLanguageResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUpdateLanguageRequest {
    pub display_language: Option<String>,
    pub user_id: String,
}
impl SettingsUpdateLanguageRequest {}
impl FluentRequest<'_, SettingsUpdateLanguageRequest> {
    pub fn display_language(mut self, display_language: &str) -> Self {
        self.params.display_language = Some(display_language.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsUpdateLanguageRequest> {
    type Output = httpclient::InMemoryResult<SettingsUpdateLanguageResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/language", user_id = self.params
                .user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.display_language {
                r = r.json(json!({ "displayLanguage" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
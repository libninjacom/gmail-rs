use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_filters_create`].

On request success, this will return a [`SettingsFiltersCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsFiltersCreateRequest {
    pub action: Option<serde_json::Value>,
    pub criteria: Option<serde_json::Value>,
    pub id: Option<String>,
    pub user_id: String,
}
impl SettingsFiltersCreateRequest {}
impl FluentRequest<'_, SettingsFiltersCreateRequest> {
    pub fn action(mut self, action: serde_json::Value) -> Self {
        self.params.action = Some(action);
        self
    }
    pub fn criteria(mut self, criteria: serde_json::Value) -> Self {
        self.params.criteria = Some(criteria);
        self
    }
    pub fn id(mut self, id: &str) -> Self {
        self.params.id = Some(id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsFiltersCreateRequest> {
    type Output = httpclient::InMemoryResult<SettingsFiltersCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/filters", user_id = self.params
                .user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.action {
                r = r.json(json!({ "action" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.criteria {
                r = r.json(json!({ "criteria" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.id {
                r = r.json(json!({ "id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
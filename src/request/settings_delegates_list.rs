use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_delegates_list`].

On request success, this will return a [`SettingsDelegatesListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsDelegatesListRequest {
    pub user_id: String,
}
impl SettingsDelegatesListRequest {}
impl FluentRequest<'_, SettingsDelegatesListRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsDelegatesListRequest> {
    type Output = httpclient::InMemoryResult<SettingsDelegatesListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/delegates", user_id = self.params
                .user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
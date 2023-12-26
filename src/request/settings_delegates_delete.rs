use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_delegates_delete`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsDelegatesDeleteRequest {
    pub delegate_email: String,
    pub user_id: String,
}
impl SettingsDelegatesDeleteRequest {}
impl FluentRequest<'_, SettingsDelegatesDeleteRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsDelegatesDeleteRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/delegates/{delegate_email}",
                delegate_email = self.params.delegate_email, user_id = self.params
                .user_id
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_get_auto_forwarding`].

On request success, this will return a [`SettingsGetAutoForwardingResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsGetAutoForwardingRequest {
    pub user_id: String,
}
impl SettingsGetAutoForwardingRequest {}
impl FluentRequest<'_, SettingsGetAutoForwardingRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsGetAutoForwardingRequest> {
    type Output = httpclient::InMemoryResult<SettingsGetAutoForwardingResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/autoForwarding", user_id = self
                .params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
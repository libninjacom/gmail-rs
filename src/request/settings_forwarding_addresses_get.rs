use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_forwarding_addresses_get`].

On request success, this will return a [`SettingsForwardingAddressesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsForwardingAddressesGetRequest {
    pub forwarding_email: String,
    pub user_id: String,
}
impl SettingsForwardingAddressesGetRequest {}
impl FluentRequest<'_, SettingsForwardingAddressesGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsForwardingAddressesGetRequest> {
    type Output = httpclient::InMemoryResult<SettingsForwardingAddressesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/forwardingAddresses/{forwarding_email}",
                forwarding_email = self.params.forwarding_email, user_id = self.params
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
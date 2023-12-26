use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_forwarding_addresses_list`].

On request success, this will return a [`SettingsForwardingAddressesListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsForwardingAddressesListRequest {
    pub user_id: String,
}
impl SettingsForwardingAddressesListRequest {}
impl FluentRequest<'_, SettingsForwardingAddressesListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsForwardingAddressesListRequest> {
    type Output = httpclient::InMemoryResult<SettingsForwardingAddressesListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/forwardingAddresses", user_id = self
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
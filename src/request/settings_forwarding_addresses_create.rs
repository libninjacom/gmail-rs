use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_forwarding_addresses_create`].

On request success, this will return a [`SettingsForwardingAddressesCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsForwardingAddressesCreateRequest {
    pub forwarding_email: Option<String>,
    pub user_id: String,
    pub verification_status: Option<String>,
}
impl SettingsForwardingAddressesCreateRequest {}
impl FluentRequest<'_, SettingsForwardingAddressesCreateRequest> {
    pub fn forwarding_email(mut self, forwarding_email: &str) -> Self {
        self.params.forwarding_email = Some(forwarding_email.to_owned());
        self
    }
    pub fn verification_status(mut self, verification_status: &str) -> Self {
        self.params.verification_status = Some(verification_status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsForwardingAddressesCreateRequest> {
    type Output = httpclient::InMemoryResult<SettingsForwardingAddressesCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/forwardingAddresses", user_id = self
                .params.user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.forwarding_email {
                r = r.json(json!({ "forwardingEmail" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.verification_status {
                r = r.json(json!({ "verificationStatus" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_send_as_smime_info_set_default`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSendAsSmimeInfoSetDefaultRequest {
    pub id: String,
    pub send_as_email: String,
    pub user_id: String,
}
impl SettingsSendAsSmimeInfoSetDefaultRequest {}
impl FluentRequest<'_, SettingsSendAsSmimeInfoSetDefaultRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsSendAsSmimeInfoSetDefaultRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/sendAs/{send_as_email}/smimeInfo/{id}/setDefault",
                id = self.params.id, send_as_email = self.params.send_as_email, user_id =
                self.params.user_id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_send_as_smime_info_list`].

On request success, this will return a [`SettingsSendAsSmimeInfoListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSendAsSmimeInfoListRequest {
    pub send_as_email: String,
    pub user_id: String,
}
impl SettingsSendAsSmimeInfoListRequest {}
impl FluentRequest<'_, SettingsSendAsSmimeInfoListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsSendAsSmimeInfoListRequest> {
    type Output = httpclient::InMemoryResult<SettingsSendAsSmimeInfoListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/sendAs/{send_as_email}/smimeInfo",
                send_as_email = self.params.send_as_email, user_id = self.params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
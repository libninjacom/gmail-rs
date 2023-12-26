use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::get_profile`].

On request success, this will return a [`GetProfileResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileRequest {
    pub user_id: String,
}
impl GetProfileRequest {}
impl FluentRequest<'_, GetProfileRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetProfileRequest> {
    type Output = httpclient::InMemoryResult<GetProfileResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/profile", user_id = self.params.user_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
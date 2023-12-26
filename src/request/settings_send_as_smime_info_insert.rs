use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_send_as_smime_info_insert`].

On request success, this will return a [`SettingsSendAsSmimeInfoInsertResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSendAsSmimeInfoInsertRequest {
    pub encrypted_key_password: Option<String>,
    pub expiration: Option<String>,
    pub id: Option<String>,
    pub is_default: Option<bool>,
    pub issuer_cn: Option<String>,
    pub pem: Option<String>,
    pub pkcs12: Option<String>,
    pub send_as_email: String,
    pub user_id: String,
}
impl SettingsSendAsSmimeInfoInsertRequest {}
impl FluentRequest<'_, SettingsSendAsSmimeInfoInsertRequest> {
    pub fn encrypted_key_password(mut self, encrypted_key_password: &str) -> Self {
        self.params.encrypted_key_password = Some(encrypted_key_password.to_owned());
        self
    }
    pub fn expiration(mut self, expiration: &str) -> Self {
        self.params.expiration = Some(expiration.to_owned());
        self
    }
    pub fn id(mut self, id: &str) -> Self {
        self.params.id = Some(id.to_owned());
        self
    }
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.params.is_default = Some(is_default);
        self
    }
    pub fn issuer_cn(mut self, issuer_cn: &str) -> Self {
        self.params.issuer_cn = Some(issuer_cn.to_owned());
        self
    }
    pub fn pem(mut self, pem: &str) -> Self {
        self.params.pem = Some(pem.to_owned());
        self
    }
    pub fn pkcs12(mut self, pkcs12: &str) -> Self {
        self.params.pkcs12 = Some(pkcs12.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SettingsSendAsSmimeInfoInsertRequest> {
    type Output = httpclient::InMemoryResult<SettingsSendAsSmimeInfoInsertResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/sendAs/{send_as_email}/smimeInfo",
                send_as_email = self.params.send_as_email, user_id = self.params.user_id
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.encrypted_key_password {
                r = r.json(json!({ "encryptedKeyPassword" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.expiration {
                r = r.json(json!({ "expiration" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.id {
                r = r.json(json!({ "id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.is_default {
                r = r.json(json!({ "isDefault" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.issuer_cn {
                r = r.json(json!({ "issuerCn" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.pem {
                r = r.json(json!({ "pem" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.pkcs12 {
                r = r.json(json!({ "pkcs12" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
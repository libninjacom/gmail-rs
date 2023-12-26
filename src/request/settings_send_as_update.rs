use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::GmailClient;
/**You should use this struct via [`GmailClient::settings_send_as_update`].

On request success, this will return a [`SettingsSendAsUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSendAsUpdateRequest {
    pub display_name: Option<String>,
    pub is_default: Option<bool>,
    pub is_primary: Option<bool>,
    pub reply_to_address: Option<String>,
    pub send_as_email: String,
    pub signature: Option<String>,
    pub smtp_msa: Option<serde_json::Value>,
    pub treat_as_alias: Option<bool>,
    pub user_id: String,
    pub verification_status: Option<String>,
}
impl SettingsSendAsUpdateRequest {}
impl FluentRequest<'_, SettingsSendAsUpdateRequest> {
    pub fn display_name(mut self, display_name: &str) -> Self {
        self.params.display_name = Some(display_name.to_owned());
        self
    }
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.params.is_default = Some(is_default);
        self
    }
    pub fn is_primary(mut self, is_primary: bool) -> Self {
        self.params.is_primary = Some(is_primary);
        self
    }
    pub fn reply_to_address(mut self, reply_to_address: &str) -> Self {
        self.params.reply_to_address = Some(reply_to_address.to_owned());
        self
    }
    pub fn signature(mut self, signature: &str) -> Self {
        self.params.signature = Some(signature.to_owned());
        self
    }
    pub fn smtp_msa(mut self, smtp_msa: serde_json::Value) -> Self {
        self.params.smtp_msa = Some(smtp_msa);
        self
    }
    pub fn treat_as_alias(mut self, treat_as_alias: bool) -> Self {
        self.params.treat_as_alias = Some(treat_as_alias);
        self
    }
    pub fn verification_status(mut self, verification_status: &str) -> Self {
        self.params.verification_status = Some(verification_status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SettingsSendAsUpdateRequest> {
    type Output = httpclient::InMemoryResult<SettingsSendAsUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/gmail/v1/users/{user_id}/settings/sendAs/{send_as_email}",
                send_as_email = self.params.send_as_email, user_id = self.params.user_id
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.display_name {
                r = r.json(json!({ "displayName" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.is_default {
                r = r.json(json!({ "isDefault" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.is_primary {
                r = r.json(json!({ "isPrimary" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.reply_to_address {
                r = r.json(json!({ "replyToAddress" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.signature {
                r = r.json(json!({ "signature" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.smtp_msa {
                r = r.json(json!({ "smtpMsa" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.treat_as_alias {
                r = r.json(json!({ "treatAsAlias" : unwrapped }));
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
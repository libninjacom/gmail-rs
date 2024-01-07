use serde::{Serialize, Deserialize};
///Auto-forwarding settings for an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsGetAutoForwardingResponse {
    ///The state that a message should be left in after it has been forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
    ///Email address to which all incoming messages are forwarded. This email address must be a verified member of the forwarding addresses.
    #[serde(rename = "emailAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///Whether all incoming mail is automatically forwarded to another address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl std::fmt::Display for SettingsGetAutoForwardingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
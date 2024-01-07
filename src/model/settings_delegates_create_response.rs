use serde::{Serialize, Deserialize};
///Settings for a delegate. Delegates can read, send, and delete messages, as well as view and add contacts, for the delegator's account. See "Set up mail delegation" for more information about delegates.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsDelegatesCreateResponse {
    ///The email address of the delegate.
    #[serde(rename = "delegateEmail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate_email: Option<String>,
    ///Indicates whether this address has been verified and can act as a delegate for the account. Read-only.
    #[serde(rename = "verificationStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for SettingsDelegatesCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
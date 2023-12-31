use serde::{Serialize, Deserialize};
///Settings for a forwarding address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsForwardingAddressesGetResponse {
    ///An email address to which messages can be forwarded.
    #[serde(rename = "forwardingEmail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forwarding_email: Option<String>,
    ///Indicates whether this address has been verified and is usable for forwarding. Read-only.
    #[serde(rename = "verificationStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for SettingsForwardingAddressesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
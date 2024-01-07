use serde::{Serialize, Deserialize};
///Response for the ListForwardingAddresses method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsForwardingAddressesListResponse {
    ///List of addresses that may be used for forwarding.
    #[serde(rename = "forwardingAddresses")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forwarding_addresses: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SettingsForwardingAddressesListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
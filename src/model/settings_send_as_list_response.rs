use serde::{Serialize, Deserialize};
///Response for the ListSendAs method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsSendAsListResponse {
    ///List of send-as aliases.
    #[serde(rename = "sendAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_as: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SettingsSendAsListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
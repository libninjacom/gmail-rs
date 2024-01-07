use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsSendAsSmimeInfoListResponse {
    ///List of SmimeInfo.
    #[serde(rename = "smimeInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smime_info: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SettingsSendAsSmimeInfoListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
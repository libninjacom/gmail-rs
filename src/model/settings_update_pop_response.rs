use serde::{Serialize, Deserialize};
///POP settings for an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsUpdatePopResponse {
    ///The range of messages which are accessible via POP.
    #[serde(rename = "accessWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_window: Option<String>,
    ///The action that will be executed on a message after it has been fetched via POP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
}
impl std::fmt::Display for SettingsUpdatePopResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
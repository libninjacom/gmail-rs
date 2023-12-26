use serde::{Serialize, Deserialize};
///Response for the ListDelegates method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsDelegatesListResponse {
    ///List of the user's delegates (with any verification status). If an account doesn't have delegates, this field doesn't appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegates: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SettingsDelegatesListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
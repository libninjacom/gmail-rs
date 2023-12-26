use serde::{Serialize, Deserialize};
///Resource definition for Gmail filters. Filters apply to specific messages instead of an entire email thread.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsFiltersCreateResponse {
    ///Action that the filter performs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<serde_json::Value>,
    ///Matching criteria for the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<serde_json::Value>,
    ///The server assigned ID of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for SettingsFiltersCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
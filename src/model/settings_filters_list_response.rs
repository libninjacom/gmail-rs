use serde::{Serialize, Deserialize};
///Response for the ListFilters method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsFiltersListResponse {
    ///List of a user's filters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for SettingsFiltersListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
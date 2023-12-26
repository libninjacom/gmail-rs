use serde::{Serialize, Deserialize};
///Language settings for an account. These settings correspond to the "Language settings" feature in the web interface.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsUpdateLanguageResponse {
    ///The language to display Gmail in, formatted as an RFC 3066 Language Tag (for example `en-GB`, `fr` or `ja` for British English, French, or Japanese respectively). The set of languages supported by Gmail evolves over time, so please refer to the "Language" dropdown in the Gmail settings for all available options, as described in the language settings help article. A table of sample values is also provided in the Managing Language Settings guide Not all Gmail clients can display the same set of languages. In the case that a user's display language is not available for use on a particular client, said client automatically chooses to display in the closest supported variant (or a reasonable default).
    #[serde(rename = "displayLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
}
impl std::fmt::Display for SettingsUpdateLanguageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
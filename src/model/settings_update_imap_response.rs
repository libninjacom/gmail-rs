use serde::{Serialize, Deserialize};
///IMAP settings for an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsUpdateImapResponse {
    ///If this value is true, Gmail will immediately expunge a message when it is marked as deleted in IMAP. Otherwise, Gmail will wait for an update from the client before expunging messages marked as deleted.
    #[serde(rename = "autoExpunge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_expunge: Option<bool>,
    ///Whether IMAP is enabled for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder.
    #[serde(rename = "expungeBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expunge_behavior: Option<String>,
    ///An optional limit on the number of messages that an IMAP folder may contain. Legal values are 0, 1000, 2000, 5000 or 10000. A value of zero is interpreted to mean that there is no limit.
    #[serde(rename = "maxFolderSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_folder_size: Option<i64>,
}
impl std::fmt::Display for SettingsUpdateImapResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
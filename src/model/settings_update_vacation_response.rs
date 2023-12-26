use serde::{Serialize, Deserialize};
///Vacation auto-reply settings for an account. These settings correspond to the "Vacation responder" feature in the web interface.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsUpdateVacationResponse {
    ///Flag that controls whether Gmail automatically replies to messages.
    #[serde(rename = "enableAutoReply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_reply: Option<bool>,
    ///An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`.
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    ///Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used.
    #[serde(rename = "responseBodyHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body_html: Option<String>,
    ///Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used.
    #[serde(rename = "responseBodyPlainText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body_plain_text: Option<String>,
    ///Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty.
    #[serde(rename = "responseSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_subject: Option<String>,
    ///Flag that determines whether responses are sent to recipients who are not in the user's list of contacts.
    #[serde(rename = "restrictToContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_to_contacts: Option<bool>,
    ///Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for G Suite users.
    #[serde(rename = "restrictToDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_to_domain: Option<bool>,
    ///An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
impl std::fmt::Display for SettingsUpdateVacationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
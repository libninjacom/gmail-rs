use serde::{Serialize, Deserialize};
///Labels are used to categorize messages and threads within the user's mailbox. The maximum number of labels supported for a user's mailbox is 10,000.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LabelsUpdateResponse {
    ///The color to assign to the label. Color is only available for labels that have their `type` set to `user`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<serde_json::Value>,
    ///The immutable ID of the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The visibility of the label in the label list in the Gmail web interface.
    #[serde(rename = "labelListVisibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_list_visibility: Option<String>,
    ///The visibility of messages with this label in the message list in the Gmail web interface.
    #[serde(rename = "messageListVisibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_list_visibility: Option<String>,
    ///The total number of messages with the label.
    #[serde(rename = "messagesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_total: Option<i64>,
    ///The number of unread messages with the label.
    #[serde(rename = "messagesUnread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_unread: Option<i64>,
    ///The display name of the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The total number of threads with the label.
    #[serde(rename = "threadsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_total: Option<i64>,
    ///The number of unread threads with the label.
    #[serde(rename = "threadsUnread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_unread: Option<i64>,
    ///The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for LabelsUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
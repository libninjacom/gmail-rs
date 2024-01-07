use serde::{Serialize, Deserialize};
///Settings associated with a send-as alias, which can be either the primary login address associated with the account or a custom "from" address. Send-as aliases correspond to the "Send Mail As" feature in the web interface.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsSendAsCreateResponse {
    ///A name that appears in the "From:" header for mail sent using this alias. For custom "from" addresses, when this is empty, Gmail will populate the "From:" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail.
    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address.
    #[serde(rename = "isDefault")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    ///Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only.
    #[serde(rename = "isPrimary")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    ///An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header.
    #[serde(rename = "replyToAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_address: Option<String>,
    ///The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create.
    #[serde(rename = "sendAsEmail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_as_email: Option<String>,
    ///An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    ///An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom "from" aliases.
    #[serde(rename = "smtpMsa")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_msa: Option<serde_json::Value>,
    ///Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases.
    #[serde(rename = "treatAsAlias")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub treat_as_alias: Option<bool>,
    ///Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases.
    #[serde(rename = "verificationStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}
impl std::fmt::Display for SettingsSendAsCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
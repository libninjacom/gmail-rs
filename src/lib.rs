//! [`GmailClient`](struct.GmailClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
mod batch;

pub use httpclient::{Error, Result, InMemoryResponseExt};
use std::sync::{Arc, OnceLock};
use std::borrow::Cow;
use httpclient::{InMemoryRequest, InMemoryResponse, ProtocolError};
use httpclient::multipart::Form;
use httpclient_oauth2::RefreshData;
use crate::batch::Batch;
use crate::model::*;
static SHARED_HTTPCLIENT: OnceLock<httpclient::Client> = OnceLock::new();
pub fn default_http_client() -> httpclient::Client {
    httpclient::Client::new().base_url("https://gmail.googleapis.com")
}
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: httpclient::Client) {
    SHARED_HTTPCLIENT.set(init).expect("Failed to manually init httpclient");
}
fn shared_http_client() -> Cow<'static, httpclient::Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
static SHARED_OAUTH2FLOW: OnceLock<httpclient_oauth2::OAuth2Flow> = OnceLock::new();
pub fn init_oauth2_flow(init: httpclient_oauth2::OAuth2Flow) {
    let _ = SHARED_OAUTH2FLOW.set(init);
}
pub fn shared_oauth2_flow() -> &'static httpclient_oauth2::OAuth2Flow {
    SHARED_OAUTH2FLOW
        .get_or_init(|| httpclient_oauth2::OAuth2Flow {
            client_id: std::env::var("GMAIL_CLIENT_ID")
                .expect("GMAIL_CLIENT_ID must be set"),
            client_secret: std::env::var("GMAIL_CLIENT_SECRET")
                .expect("GMAIL_CLIENT_SECRET must be set"),
            init_endpoint: "https://accounts.google.com/o/oauth2/auth".to_string(),
            exchange_endpoint: "https://accounts.google.com/o/oauth2/token".to_string(),
            refresh_endpoint: "https://accounts.google.com/o/oauth2/token".to_string(),
            redirect_uri: std::env::var("GMAIL_REDIRECT_URI")
                .expect("GMAIL_REDIRECT_URI must be set"),
        })
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a GmailClient,
    pub params: T,
}
pub struct GmailClient {
    client: Cow<'static, httpclient::Client>,
    authentication: GmailAuth,
}
impl GmailClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
            authentication: GmailAuth::from_env(),
        }
    }
    pub fn with_auth(authentication: GmailAuth) -> Self {
        Self {
            client: shared_http_client(),
            authentication,
        }
    }
    pub fn new_with(client: httpclient::Client, authentication: GmailAuth) -> Self {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    }
}
impl GmailClient {
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            GmailAuth::OAuth2 { middleware } => {
                r.middlewares.insert(0, middleware.clone());
            }
        }
        r
    }
    ///Lists the drafts in the user's mailbox.
    pub fn drafts_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::DraftsListRequest> {
        FluentRequest {
            client: self,
            params: request::DraftsListRequest {
                include_spam_trash: None,
                max_results: None,
                page_token: None,
                q: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Creates a new draft with the `DRAFT` label.
    pub fn drafts_create(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::DraftsCreateRequest> {
        FluentRequest {
            client: self,
            params: request::DraftsCreateRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers.
    pub fn drafts_send(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::DraftsSendRequest> {
        FluentRequest {
            client: self,
            params: request::DraftsSendRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the specified draft.
    pub fn drafts_get(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::DraftsGetRequest> {
        FluentRequest {
            client: self,
            params: request::DraftsGetRequest {
                format: None,
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Replaces a draft's content.
    pub fn drafts_update(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::DraftsUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::DraftsUpdateRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Immediately and permanently deletes the specified draft. Does not simply trash it.
    pub fn drafts_delete(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::DraftsDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::DraftsDeleteRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`).
    pub fn history_list(
        &self,
        user_id: &str,
        start_history_id: &str,
    ) -> FluentRequest<'_, request::HistoryListRequest> {
        FluentRequest {
            client: self,
            params: request::HistoryListRequest {
                history_types: None,
                label_id: None,
                max_results: None,
                page_token: None,
                start_history_id: start_history_id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists all labels in the user's mailbox.
    pub fn labels_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::LabelsListRequest> {
        FluentRequest {
            client: self,
            params: request::LabelsListRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Creates a new label.
    pub fn labels_create(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::LabelsCreateRequest> {
        FluentRequest {
            client: self,
            params: request::LabelsCreateRequest {
                color: None,
                id: None,
                label_list_visibility: None,
                message_list_visibility: None,
                messages_total: None,
                messages_unread: None,
                name: None,
                threads_total: None,
                threads_unread: None,
                type_: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the specified label.
    pub fn labels_get(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::LabelsGetRequest> {
        FluentRequest {
            client: self,
            params: request::LabelsGetRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates the specified label.
    pub fn labels_update(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::LabelsUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::LabelsUpdateRequest {
                color: None,
                id: id.to_owned(),
                label_list_visibility: None,
                message_list_visibility: None,
                messages_total: None,
                messages_unread: None,
                name: None,
                threads_total: None,
                threads_unread: None,
                type_: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Immediately and permanently deletes the specified label and removes it from any messages and threads that it is applied to.
    pub fn labels_delete(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::LabelsDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::LabelsDeleteRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Patch the specified label.
    pub fn labels_patch(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::LabelsPatchRequest> {
        FluentRequest {
            client: self,
            params: request::LabelsPatchRequest {
                color: None,
                id: id.to_owned(),
                label_list_visibility: None,
                message_list_visibility: None,
                messages_total: None,
                messages_unread: None,
                name: None,
                threads_total: None,
                threads_unread: None,
                type_: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the messages in the user's mailbox.
    pub fn messages_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesListRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesListRequest {
                include_spam_trash: None,
                label_ids: None,
                max_results: None,
                page_token: None,
                q: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message.
    pub fn messages_insert(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesInsertRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesInsertRequest {
                deleted: None,
                internal_date_source: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Deletes many messages by message ID. Provides no guarantees that messages were not already deleted or even existed at all.
    pub fn messages_batch_delete(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesBatchDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesBatchDeleteRequest {
                ids: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Modifies the labels on the specified messages.
    pub fn messages_batch_modify(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesBatchModifyRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesBatchModifyRequest {
                add_label_ids: None,
                ids: None,
                remove_label_ids: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. This method doesn't perform SPF checks, so it might not work for some spam messages, such as those attempting to perform domain spoofing. This method does not send a message. Note: This function doesn't trigger forwarding rules or filters set up by the user.
    pub fn messages_import(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesImportRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesImportRequest {
                deleted: None,
                internal_date_source: None,
                never_mark_spam: None,
                process_for_calendar: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers.
    pub fn messages_send(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesSendRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesSendRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the specified message.
    pub fn messages_get(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesGetRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesGetRequest {
                format: None,
                id: id.to_owned(),
                metadata_headers: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer `messages.trash` instead.
    pub fn messages_delete(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesDeleteRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Modifies the labels on the specified message.
    pub fn messages_modify(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesModifyRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesModifyRequest {
                add_label_ids: None,
                id: id.to_owned(),
                remove_label_ids: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Moves the specified message to the trash.
    pub fn messages_trash(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesTrashRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesTrashRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Removes the specified message from the trash.
    pub fn messages_untrash(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesUntrashRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesUntrashRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the specified message attachment.
    pub fn messages_attachments_get(
        &self,
        id: &str,
        message_id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::MessagesAttachmentsGetRequest> {
        FluentRequest {
            client: self,
            params: request::MessagesAttachmentsGetRequest {
                id: id.to_owned(),
                message_id: message_id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the current user's Gmail profile.
    pub fn get_profile(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::GetProfileRequest> {
        FluentRequest {
            client: self,
            params: request::GetProfileRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the auto-forwarding setting for the specified account.
    pub fn settings_get_auto_forwarding(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsGetAutoForwardingRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsGetAutoForwardingRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates the auto-forwarding setting for the specified account. A verified forwarding address must be specified when auto-forwarding is enabled. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_update_auto_forwarding(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsUpdateAutoForwardingRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsUpdateAutoForwardingRequest {
                disposition: None,
                email_address: None,
                enabled: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the delegates for the specified account. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_delegates_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsDelegatesListRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsDelegatesListRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Adds a delegate with its verification status set directly to `accepted`, without sending any verification email. The delegate user must be a member of the same G Suite organization as the delegator user. Gmail imposes limitations on the number of delegates and delegators each user in a G Suite organization can have. These limits depend on your organization, but in general each user can have up to 25 delegates and up to 10 delegators. Note that a delegate user must be referred to by their primary email address, and not an email alias. Also note that when a new delegate is created, there may be up to a one minute delay before the new delegate is available for use. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_delegates_create(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsDelegatesCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsDelegatesCreateRequest {
                delegate_email: None,
                user_id: user_id.to_owned(),
                verification_status: None,
            },
        }
    }
    ///Gets the specified delegate. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_delegates_get(
        &self,
        delegate_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsDelegatesGetRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsDelegatesGetRequest {
                delegate_email: delegate_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Removes the specified delegate (which can be of any verification status), and revokes any verification that may have been required for using it. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_delegates_delete(
        &self,
        delegate_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsDelegatesDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsDelegatesDeleteRequest {
                delegate_email: delegate_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the message filters of a Gmail user.
    pub fn settings_filters_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsFiltersListRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsFiltersListRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Creates a filter. Note: you can only create a maximum of 1,000 filters.
    pub fn settings_filters_create(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsFiltersCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsFiltersCreateRequest {
                action: None,
                criteria: None,
                id: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets a filter.
    pub fn settings_filters_get(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsFiltersGetRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsFiltersGetRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Deletes a filter.
    pub fn settings_filters_delete(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsFiltersDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsFiltersDeleteRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the forwarding addresses for the specified account.
    pub fn settings_forwarding_addresses_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsForwardingAddressesListRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsForwardingAddressesListRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_forwarding_addresses_create(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsForwardingAddressesCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsForwardingAddressesCreateRequest {
                forwarding_email: None,
                user_id: user_id.to_owned(),
                verification_status: None,
            },
        }
    }
    ///Gets the specified forwarding address.
    pub fn settings_forwarding_addresses_get(
        &self,
        forwarding_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsForwardingAddressesGetRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsForwardingAddressesGetRequest {
                forwarding_email: forwarding_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Deletes the specified forwarding address and revokes any verification that may have been required. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_forwarding_addresses_delete(
        &self,
        forwarding_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsForwardingAddressesDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsForwardingAddressesDeleteRequest {
                forwarding_email: forwarding_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets IMAP settings.
    pub fn settings_get_imap(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsGetImapRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsGetImapRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates IMAP settings.
    pub fn settings_update_imap(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsUpdateImapRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsUpdateImapRequest {
                auto_expunge: None,
                enabled: None,
                expunge_behavior: None,
                max_folder_size: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets language settings.
    pub fn settings_get_language(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsGetLanguageRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsGetLanguageRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates language settings. If successful, the return object contains the `displayLanguage` that was saved for the user, which may differ from the value passed into the request. This is because the requested `displayLanguage` may not be directly supported by Gmail but have a close variant that is, and so the variant may be chosen and saved instead.
    pub fn settings_update_language(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsUpdateLanguageRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsUpdateLanguageRequest {
                display_language: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets POP settings.
    pub fn settings_get_pop(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsGetPopRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsGetPopRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates POP settings.
    pub fn settings_update_pop(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsUpdatePopRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsUpdatePopRequest {
                access_window: None,
                disposition: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the send-as aliases for the specified account. The result includes the primary send-as address associated with the account as well as any custom "from" aliases.
    pub fn settings_send_as_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsListRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsListRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Creates a custom "from" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_send_as_create(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsCreateRequest {
                display_name: None,
                is_default: None,
                is_primary: None,
                reply_to_address: None,
                send_as_email: None,
                signature: None,
                smtp_msa: None,
                treat_as_alias: None,
                user_id: user_id.to_owned(),
                verification_status: None,
            },
        }
    }
    ///Gets the specified send-as alias. Fails with an HTTP 404 error if the specified address is not a member of the collection.
    pub fn settings_send_as_get(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsGetRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsGetRequest {
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates a send-as alias. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. Addresses other than the primary address for the account can only be updated by service account clients that have been delegated domain-wide authority.
    pub fn settings_send_as_update(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsUpdateRequest {
                display_name: None,
                is_default: None,
                is_primary: None,
                reply_to_address: None,
                send_as_email: send_as_email.to_owned(),
                signature: None,
                smtp_msa: None,
                treat_as_alias: None,
                user_id: user_id.to_owned(),
                verification_status: None,
            },
        }
    }
    ///Deletes the specified send-as alias. Revokes any verification that may have been required for using it. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_send_as_delete(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsDeleteRequest {
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Patch the specified send-as alias.
    pub fn settings_send_as_patch(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsPatchRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsPatchRequest {
                display_name: None,
                is_default: None,
                is_primary: None,
                reply_to_address: None,
                send_as_email: send_as_email.to_owned(),
                signature: None,
                smtp_msa: None,
                treat_as_alias: None,
                user_id: user_id.to_owned(),
                verification_status: None,
            },
        }
    }
    ///Lists S/MIME configs for the specified send-as alias.
    pub fn settings_send_as_smime_info_list(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsSmimeInfoListRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsSmimeInfoListRequest {
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Insert (upload) the given S/MIME config for the specified send-as alias. Note that pkcs12 format is required for the key.
    pub fn settings_send_as_smime_info_insert(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsSmimeInfoInsertRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsSmimeInfoInsertRequest {
                encrypted_key_password: None,
                expiration: None,
                id: None,
                is_default: None,
                issuer_cn: None,
                pem: None,
                pkcs12: None,
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the specified S/MIME config for the specified send-as alias.
    pub fn settings_send_as_smime_info_get(
        &self,
        id: &str,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsSmimeInfoGetRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsSmimeInfoGetRequest {
                id: id.to_owned(),
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Deletes the specified S/MIME config for the specified send-as alias.
    pub fn settings_send_as_smime_info_delete(
        &self,
        id: &str,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsSmimeInfoDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsSmimeInfoDeleteRequest {
                id: id.to_owned(),
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Sets the default S/MIME config for the specified send-as alias.
    pub fn settings_send_as_smime_info_set_default(
        &self,
        id: &str,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsSmimeInfoSetDefaultRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsSmimeInfoSetDefaultRequest {
                id: id.to_owned(),
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Sends a verification email to the specified send-as alias address. The verification status must be `pending`. This method is only available to service account clients that have been delegated domain-wide authority.
    pub fn settings_send_as_verify(
        &self,
        send_as_email: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsSendAsVerifyRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsSendAsVerifyRequest {
                send_as_email: send_as_email.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets vacation responder settings.
    pub fn settings_get_vacation(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsGetVacationRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsGetVacationRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Updates vacation responder settings.
    pub fn settings_update_vacation(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::SettingsUpdateVacationRequest> {
        FluentRequest {
            client: self,
            params: request::SettingsUpdateVacationRequest {
                enable_auto_reply: None,
                end_time: None,
                response_body_html: None,
                response_body_plain_text: None,
                response_subject: None,
                restrict_to_contacts: None,
                restrict_to_domain: None,
                start_time: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Stop receiving push notifications for the given user mailbox.
    pub fn stop(&self, user_id: &str) -> FluentRequest<'_, request::StopRequest> {
        FluentRequest {
            client: self,
            params: request::StopRequest {
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Lists the threads in the user's mailbox.
    pub fn threads_list(
        &self,
        user_id: &str,
    ) -> FluentRequest<'_, request::ThreadsListRequest> {
        FluentRequest {
            client: self,
            params: request::ThreadsListRequest {
                include_spam_trash: None,
                label_ids: None,
                max_results: None,
                page_token: None,
                q: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Gets the specified thread.
    pub fn threads_get(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::ThreadsGetRequest> {
        FluentRequest {
            client: self,
            params: request::ThreadsGetRequest {
                format: None,
                id: id.to_owned(),
                metadata_headers: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Immediately and permanently deletes the specified thread. This operation cannot be undone. Prefer `threads.trash` instead.
    pub fn threads_delete(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::ThreadsDeleteRequest> {
        FluentRequest {
            client: self,
            params: request::ThreadsDeleteRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Modifies the labels applied to the thread. This applies to all messages in the thread.
    pub fn threads_modify(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::ThreadsModifyRequest> {
        FluentRequest {
            client: self,
            params: request::ThreadsModifyRequest {
                add_label_ids: None,
                id: id.to_owned(),
                remove_label_ids: None,
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Moves the specified thread to the trash.
    pub fn threads_trash(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::ThreadsTrashRequest> {
        FluentRequest {
            client: self,
            params: request::ThreadsTrashRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Removes the specified thread from the trash.
    pub fn threads_untrash(
        &self,
        id: &str,
        user_id: &str,
    ) -> FluentRequest<'_, request::ThreadsUntrashRequest> {
        FluentRequest {
            client: self,
            params: request::ThreadsUntrashRequest {
                id: id.to_owned(),
                user_id: user_id.to_owned(),
            },
        }
    }
    ///Set up or update a push notification watch on the given user mailbox.
    pub fn watch(&self, user_id: &str) -> FluentRequest<'_, request::WatchRequest> {
        FluentRequest {
            client: self,
            params: request::WatchRequest {
                label_filter_action: None,
                label_ids: None,
                topic_name: None,
                user_id: user_id.to_owned(),
            },
        }
    }

    pub async fn batch(&self, form: Form<InMemoryRequest>) -> Result<Form<InMemoryResponse>> {
        use serde::de::Error as SerdeError;
        let r = self.client.post("/batch");
        let r = self.authenticate(r);
        let res = r.multipart(form).await?;
        let Some(form) = Form::from_response(res) else {
            // this shouldn't really be a json error
            let err = serde_json::Error::custom("Failed to parse multipart");
            return Err(Error::Protocol(ProtocolError::JsonError(err)));
        };
        Ok(form)
    }
}
pub enum GmailAuth {
    OAuth2 { middleware: Arc<httpclient_oauth2::OAuth2> },
}
impl GmailAuth {
    pub fn from_env() -> Self {
        let access = std::env::var("GMAIL_ACCESS_TOKEN").unwrap();
        let refresh = std::env::var("GMAIL_REFRESH_TOKEN").unwrap();
        let mw = shared_oauth2_flow().bearer_middleware(access, refresh);
        Self::OAuth2 {
            middleware: Arc::new(mw),
        }
    }
    pub fn oauth2(access: impl Into<String>, refresh: impl Into<String>, callback: Option<Box<dyn Fn(RefreshData) + Send + Sync + 'static>>) -> Self {
        let mut mw = shared_oauth2_flow().bearer_middleware(access.into(), refresh.into());
        if let Some(cb) = callback {
            mw.callback(cb);
        }
        Self::OAuth2 {
            middleware: Arc::new(mw),
        }
    }
}
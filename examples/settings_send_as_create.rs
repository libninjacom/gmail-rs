#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_send_as_create(user_id)
        .display_name("your display name")
        .is_default(true)
        .is_primary(true)
        .reply_to_address("your reply to address")
        .send_as_email("your send as email")
        .signature("your signature")
        .smtp_msa(serde_json::json!({}))
        .treat_as_alias(true)
        .verification_status("your verification status")
        .await
        .unwrap();
    println!("{:#?}", response);
}
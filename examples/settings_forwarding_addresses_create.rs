#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_forwarding_addresses_create(user_id)
        .forwarding_email("your forwarding email")
        .verification_status("your verification status")
        .await
        .unwrap();
    println!("{:#?}", response);
}
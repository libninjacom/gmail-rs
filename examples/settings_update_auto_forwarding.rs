#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_update_auto_forwarding(user_id)
        .disposition("your disposition")
        .email_address("your email address")
        .enabled(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
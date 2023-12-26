#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_update_language(user_id)
        .display_language("your display language")
        .await
        .unwrap();
    println!("{:#?}", response);
}
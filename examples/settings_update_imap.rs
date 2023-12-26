#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_update_imap(user_id)
        .auto_expunge(true)
        .enabled(true)
        .expunge_behavior("your expunge behavior")
        .max_folder_size(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}
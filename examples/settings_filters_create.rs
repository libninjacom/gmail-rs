#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_filters_create(user_id)
        .action(serde_json::json!({}))
        .criteria(serde_json::json!({}))
        .id("your id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
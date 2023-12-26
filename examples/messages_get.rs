#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let id = "your id";
    let user_id = "your user id";
    let response = client
        .messages_get(id, user_id)
        .format("your format")
        .metadata_headers(&["your metadata headers"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
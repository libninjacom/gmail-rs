#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .messages_batch_delete(user_id)
        .ids(&["your ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
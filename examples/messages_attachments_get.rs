#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let id = "your id";
    let message_id = "your message id";
    let user_id = "your user id";
    let response = client
        .messages_attachments_get(id, message_id, user_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
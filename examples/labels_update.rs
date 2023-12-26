#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let id = "your id";
    let user_id = "your user id";
    let response = client
        .labels_update(id, user_id)
        .color(serde_json::json!({}))
        .label_list_visibility("your label list visibility")
        .message_list_visibility("your message list visibility")
        .messages_total(1)
        .messages_unread(1)
        .name("your name")
        .threads_total(1)
        .threads_unread(1)
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}
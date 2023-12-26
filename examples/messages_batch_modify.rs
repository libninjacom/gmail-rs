#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .messages_batch_modify(user_id)
        .add_label_ids(&["your add label ids"])
        .ids(&["your ids"])
        .remove_label_ids(&["your remove label ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
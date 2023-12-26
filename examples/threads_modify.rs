#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let id = "your id";
    let user_id = "your user id";
    let response = client
        .threads_modify(id, user_id)
        .add_label_ids(&["your add label ids"])
        .remove_label_ids(&["your remove label ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}
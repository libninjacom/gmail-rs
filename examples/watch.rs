#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .watch(user_id)
        .label_filter_action("your label filter action")
        .label_ids(&["your label ids"])
        .topic_name("your topic name")
        .await
        .unwrap();
    println!("{:#?}", response);
}
#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .messages_insert(user_id)
        .deleted(true)
        .internal_date_source("your internal date source")
        .await
        .unwrap();
    println!("{:#?}", response);
}
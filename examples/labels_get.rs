#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let id = "your id";
    let user_id = "your user id";
    let response = client.labels_get(id, user_id).await.unwrap();
    println!("{:#?}", response);
}
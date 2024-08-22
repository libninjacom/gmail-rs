#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client.messages_send(user_id, httpclient::InMemoryBody::Text("test".to_string()), None).await.unwrap();
    println!("{:#?}", response);
}
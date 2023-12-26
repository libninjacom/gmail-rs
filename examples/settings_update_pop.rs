#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_update_pop(user_id)
        .access_window("your access window")
        .disposition("your disposition")
        .await
        .unwrap();
    println!("{:#?}", response);
}
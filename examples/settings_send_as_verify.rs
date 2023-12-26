#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let send_as_email = "your send as email";
    let user_id = "your user id";
    let response = client.settings_send_as_verify(send_as_email, user_id).await.unwrap();
    println!("{:#?}", response);
}
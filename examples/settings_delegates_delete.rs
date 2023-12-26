#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let delegate_email = "your delegate email";
    let user_id = "your user id";
    let response = client
        .settings_delegates_delete(delegate_email, user_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let forwarding_email = "your forwarding email";
    let user_id = "your user id";
    let response = client
        .settings_forwarding_addresses_delete(forwarding_email, user_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}
#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .history_list(user_id, "history id")
        .history_types(&["your history types"])
        .label_id("your label id")
        .max_results(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
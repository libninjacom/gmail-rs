#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .drafts_list(user_id)
        .include_spam_trash(true)
        .max_results(1)
        .page_token("your page token")
        .q("your q")
        .await
        .unwrap();
    println!("{:#?}", response);
}
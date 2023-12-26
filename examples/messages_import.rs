#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .messages_import(user_id)
        .deleted(true)
        .internal_date_source("your internal date source")
        .never_mark_spam(true)
        .process_for_calendar(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
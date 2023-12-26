#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let user_id = "your user id";
    let response = client
        .settings_update_vacation(user_id)
        .enable_auto_reply(true)
        .end_time("your end time")
        .response_body_html("your response body html")
        .response_body_plain_text("your response body plain text")
        .response_subject("your response subject")
        .restrict_to_contacts(true)
        .restrict_to_domain(true)
        .start_time("your start time")
        .await
        .unwrap();
    println!("{:#?}", response);
}
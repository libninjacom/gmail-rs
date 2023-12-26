#![allow(unused_imports)]
use gmail::GmailClient;
use gmail::model::*;
#[tokio::main]
async fn main() {
    let client = GmailClient::from_env();
    let send_as_email = "your send as email";
    let user_id = "your user id";
    let response = client
        .settings_send_as_smime_info_insert(send_as_email, user_id)
        .encrypted_key_password("your encrypted key password")
        .expiration("your expiration")
        .id("your id")
        .is_default(true)
        .issuer_cn("your issuer cn")
        .pem("your pem")
        .pkcs12("your pkcs 12")
        .await
        .unwrap();
    println!("{:#?}", response);
}
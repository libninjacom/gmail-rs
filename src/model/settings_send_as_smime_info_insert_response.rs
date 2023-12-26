use serde::{Serialize, Deserialize};
///An S/MIME email config.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsSendAsSmimeInfoInsertResponse {
    ///Encrypted key password, when key is encrypted.
    #[serde(rename = "encryptedKeyPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_key_password: Option<String>,
    ///When the certificate expires (in milliseconds since epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    ///The immutable ID for the SmimeInfo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Whether this SmimeInfo is the default one for this user's send-as address.
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    ///The S/MIME certificate issuer's common name.
    #[serde(rename = "issuerCn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_cn: Option<String>,
    ///PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
    ///PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs12: Option<String>,
}
impl std::fmt::Display for SettingsSendAsSmimeInfoInsertResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
use serde::{Serialize, Deserialize};
use super::Header;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessagePartBody {
    Attachment {
        ///The ID of the attachment.
        #[serde(rename = "attachmentId")]
        attachment_id: String,
        ///The file size in bytes of the attachment.
        size: i64,
    },
    Body {
        ///The body data of a MIME message part as a base64url encoded string.
        data: String,
        ///Total number of bytes in the body of the message part after decoding.
        size: i64,
    },
    Empty {
        ///The empty message part body.
        size: i64,
    },
}

impl std::fmt::Display for MessagePartBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
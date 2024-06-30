use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct RequestMail {
    pub from: String,
    pub to: String,
    pub cc: String,
    pub bcc: String,
    pub subject: String,
    pub content: String,
}

// impl Default for RequestMail {
//     fn default() -> RequestMail {
//         Self {
//             from: "default".to_string(),
//             to: "default".to_string(),
//             cc: "default".to_string(),
//             bcc: "default".to_string(),
//             subject: "default".to_string(),
//             content: "default".to_string(),
//         }
//     }
// }

impl RequestMail {
    pub fn new(
        from: String,
        to: String,
        cc: String,
        bcc: String,
        subject: String,
        content: String,
    ) -> Self {
        Self {
            from,
            to,
            cc,
            bcc,
            subject,
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ResultMail {
    pub success: bool,
    pub message: String,
}

impl ResultMail {
    pub fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }
}

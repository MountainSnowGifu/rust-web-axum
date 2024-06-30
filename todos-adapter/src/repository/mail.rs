use async_trait::async_trait;
use todos_domain::{
    model::mail::{RequestMail, ResultMail},
    repository::mail::MailRepository,
};

#[derive(Debug, Clone, Default)]
pub struct MailRepositoryForMemory {}

impl MailRepositoryForMemory {
    pub fn new() -> Self {
        MailRepositoryForMemory {}
    }
}

#[async_trait]
impl MailRepository for MailRepositoryForMemory {
    async fn mail_send(&self, mail: RequestMail) -> anyhow::Result<ResultMail> {
        println!("mail send:MailRepositoryForMemory:{}", mail.content);
        let result_mail = ResultMail::new(true, "mail sent:MailRepositoryForMemory".to_owned());
        Ok(result_mail)
    }
}

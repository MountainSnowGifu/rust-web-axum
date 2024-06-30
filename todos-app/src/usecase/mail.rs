use std::sync::Arc;

use todos_domain::{
    model::mail::{RequestMail, ResultMail},
    repository::mail::MailRepository,
};

pub struct MailUseCase<T: MailRepository> {
    mail_repository: Arc<T>,
}

impl<T: MailRepository> MailUseCase<T> {
    pub fn new(mail_repository: Arc<T>) -> Self {
        Self { mail_repository }
    }

    pub async fn mail_send(&self, mail: RequestMail) -> anyhow::Result<ResultMail> {
        self.mail_repository.mail_send(mail).await
    }
}

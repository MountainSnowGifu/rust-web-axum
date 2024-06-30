use async_trait::async_trait;

use crate::model::mail::{RequestMail, ResultMail};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailRepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait MailRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn mail_send(&self, mail: RequestMail) -> anyhow::Result<ResultMail>;
}

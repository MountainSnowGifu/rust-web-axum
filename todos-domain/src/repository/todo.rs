use async_trait::async_trait;
use thiserror::Error;

use crate::model::todo::Todo;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn all(&self) -> Vec<Todo>;
}

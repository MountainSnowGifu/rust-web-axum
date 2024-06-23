use std::sync::Arc;

use todos_domain::{model::todo::Todo, repository::todo::TodoRepository};

pub struct TodoUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> TodoUseCase<T> {
    pub fn new(todo_repository: Arc<T>) -> Self {
        Self { todo_repository }
    }

    pub async fn all(&self) -> Vec<Todo> {
        self.todo_repository.all().await
    }
}

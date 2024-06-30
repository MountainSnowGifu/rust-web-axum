use async_trait::async_trait;
use todos_domain::{model::todo::Todo, repository::todo::TodoRepository};

#[derive(Debug, Clone, Default)]
pub struct TodoRepositoryForMemory {}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {}
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForMemory {
    async fn all(&self) -> Vec<Todo> {
        let mut v: Vec<Todo> = Vec::new();
        v.push(Todo {
            id: 1,
            text: "test".to_string(),
            completed: false,
        });
        v
    }
}

use std::sync::Arc;

//use todos_adapter::repository::todo::TodoRepositoryForMemory;
use todos_adapter::repository::todo_redis::TodoRepositoryForRedis;
use todos_app::usecase::todo::TodoUseCase;

pub struct Modules {
    todo_use_case: TodoUseCase<TodoRepositoryForRedis>,
    //todo_use_case: TodoUseCase<TodoRepositoryForMemory>,
}

impl Modules {
    pub async fn new() -> Self {
        let todo_use_case = TodoUseCase::new(Arc::new(TodoRepositoryForRedis::new()));
        Self { todo_use_case }
    }

    pub fn todo_use_case(&self) -> &TodoUseCase<TodoRepositoryForRedis> {
        &self.todo_use_case
    }
}

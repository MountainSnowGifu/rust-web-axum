use std::{fmt, sync::Arc};

use todos_adapter::repository::{
    mail_grpc::MailRepositoryForGRPC, todo_redis::TodoRepositoryForRedis,
};
use todos_app::usecase::{mail::MailUseCase, todo::TodoUseCase};

impl Default for Modules {
    fn default() -> Modules {
        let todo_use_case = TodoUseCase::new(Arc::new(TodoRepositoryForRedis::new()));
        let mail_use_case = MailUseCase::new(Arc::new(MailRepositoryForGRPC::new()));
        Self {
            todo_use_case,
            mail_use_case,
        }
    }
}

impl fmt::Debug for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Modules")
    }
}

pub struct Modules {
    todo_use_case: TodoUseCase<TodoRepositoryForRedis>,
    mail_use_case: MailUseCase<MailRepositoryForGRPC>,
}

impl Modules {
    pub async fn new() -> Self {
        let todo_use_case = TodoUseCase::new(Arc::new(TodoRepositoryForRedis::new()));
        let mail_use_case = MailUseCase::new(Arc::new(MailRepositoryForGRPC::new()));
        Self {
            todo_use_case,
            mail_use_case,
        }
    }

    pub fn todo_use_case(&self) -> &TodoUseCase<TodoRepositoryForRedis> {
        &self.todo_use_case
    }
    pub fn mail_use_case(&self) -> &MailUseCase<MailRepositoryForGRPC> {
        &self.mail_use_case
    }
}

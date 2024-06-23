use async_trait::async_trait;
use redis::Value;
use todos_domain::{model::todo::Todo, repository::todo::TodoRepository};

const REDIS_URL: &str = "redis://127.0.0.1/";
const REDIS_TEST_KEY: &str = "REDIS_TEST_KEY";

pub trait IntoRedisValue {
    fn into_redis_value(self) -> Value;
}

#[derive(Debug, Clone, Default)]
pub struct TodoRepositoryForRedis {}

impl TodoRepositoryForRedis {
    pub fn new() -> Self {
        TodoRepositoryForRedis {}
    }

    pub async fn redis_test(&self, todos: Vec<Todo>) -> redis::RedisResult<Vec<Todo>> {
        let client = redis::Client::open(REDIS_URL).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await?;

        let json = serde_json::to_string(&todos)?;

        redis::cmd("SET")
            .arg(REDIS_TEST_KEY)
            .arg(json)
            .query_async::<_, ()>(&mut conn)
            .await
            .unwrap();
        let result: Result<String, _> = redis::cmd("GET")
            .arg(REDIS_TEST_KEY)
            .query_async(&mut conn)
            .await;
        let result_string = result.unwrap();
        let retrieved_todos: Vec<Todo> = serde_json::from_str(&result_string)?;

        Ok(retrieved_todos)
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForRedis {
    async fn all(&self) -> Vec<Todo> {
        let mut v: Vec<Todo> = Vec::new();
        v.push(Todo {
            id: 1,
            text: "test_1".to_string(),
            completed: false,
        });
        v.push(Todo {
            id: 2,
            text: "test_2".to_string(),
            completed: true,
        });

        self.redis_test(v.clone()).await.unwrap();
        v
    }
}

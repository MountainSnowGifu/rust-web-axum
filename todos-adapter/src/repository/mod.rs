pub mod mail;
pub mod mail_grpc;
pub mod test_grpc;
pub mod todo_memory;
pub mod todo_redis;

pub const GRPC_ADRESS: &str = "http://[::1]:50051";

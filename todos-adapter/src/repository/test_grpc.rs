use todos_domain::grpc_model::mail::hello_world::{
    greeter2_server::Greeter2, HelloReply, HelloRequest,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter2 {}

#[tonic::async_trait]
impl Greeter2 for MyGreeter2 {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello Rust Axum {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

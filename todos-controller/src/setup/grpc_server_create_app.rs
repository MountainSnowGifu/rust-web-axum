use std::sync::Arc;

use todos_adapter::repository::test_grpc::MyGreeter2;
use todos_domain::grpc_model::mail::{
    hello_world::greeter2_server::Greeter2Server, mail_sender::mail_sender_server::MailSenderServer,
};
use tonic::{service::interceptor::InterceptedService, transport::Server};

use crate::{
    grpc::{mail_interceptor::mail_interceptor, mail_sender::OneKitMailSender},
    module::modules::Modules,
};

pub async fn start_grpc_server(modules: Arc<Modules>) -> Result<(), Box<dyn std::error::Error>> {
    println!("start_grpc_server");
    let addr = "[::1]:50051".parse()?;
    let greeter2 = MyGreeter2::default();
    let mail_sender = OneKitMailSender::new(modules);
    let mail_sender_server = MailSenderServer::new(mail_sender);
    let mail_sender_server = InterceptedService::new(mail_sender_server, mail_interceptor);

    Server::builder()
        .add_service(Greeter2Server::new(greeter2))
        .add_service(mail_sender_server)
        .serve(addr)
        .await?;

    Ok(())
}

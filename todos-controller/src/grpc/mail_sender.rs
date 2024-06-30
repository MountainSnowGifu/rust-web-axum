use std::sync::Arc;

use todos_domain::grpc_model::mail::mail_sender::{
    mail_sender_server::MailSender, MailReply, MailRequest,
};
use tonic::{Request, Response, Status};

use crate::module::modules::Modules;

#[derive(Debug, Default)]
pub struct OneKitMailSender {
    modules: Arc<Modules>,
}

impl OneKitMailSender {
    pub fn new(modules: Arc<Modules>) -> Self {
        OneKitMailSender { modules }
    }
}

#[tonic::async_trait]
impl MailSender for OneKitMailSender {
    async fn mail_send(
        &self,
        request: Request<MailRequest>,
    ) -> Result<Response<MailReply>, Status> {
        let reply = MailReply {
            message: format!("mail {}!", request.into_inner().subject),
            success: true,
        };

        let todo = self.modules.todo_use_case().all().await;
        tracing::debug!("{:#?}", todo);

        Ok(Response::new(reply))
    }
}

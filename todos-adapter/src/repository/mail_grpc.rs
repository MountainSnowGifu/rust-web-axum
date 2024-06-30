use anyhow::Ok;
use async_trait::async_trait;
use todos_domain::{
    grpc_model::mail::mail_sender::{mail_sender_client::MailSenderClient, MailRequest},
    model::mail::{RequestMail, ResultMail},
    repository::mail::MailRepository,
};
use tonic::metadata::MetadataValue;

use crate::repository::GRPC_ADRESS;

#[derive(Debug, Clone, Default)]
pub struct MailRepositoryForGRPC {}

impl MailRepositoryForGRPC {
    pub fn new() -> Self {
        MailRepositoryForGRPC {}
    }
}

#[async_trait]
impl MailRepository for MailRepositoryForGRPC {
    async fn mail_send(&self, mail: RequestMail) -> anyhow::Result<ResultMail> {
        println!("mail send:MailRepositoryForGRPC");

        let mut client = MailSenderClient::connect(GRPC_ADRESS).await.unwrap();

        let mut request = tonic::Request::new(MailRequest {
            from: mail.from,
            to: mail.to,
            cc: mail.cc,
            bcc: mail.bcc,
            subject: mail.subject,
            content: mail.content,
        });

        let token: MetadataValue<_> = "Bearer some-auth-token".parse()?;
        request
            .metadata_mut()
            .insert("custom-header", token.clone());

        let response = client.mail_send(request).await?;

        println!("mail send:RESPONSE={:?}", response);
        let result = ResultMail::new(
            response.get_ref().success,
            response.get_ref().message.clone(),
        );

        Ok(result)
    }
}

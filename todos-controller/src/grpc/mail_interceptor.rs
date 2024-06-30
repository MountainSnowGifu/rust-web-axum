use tonic::{Request, Status};

pub fn mail_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    tracing::debug!("mail_interceptor:{:#?}", req);

    let metadata = req.metadata();

    // メタデータを取り出して処理する
    if let Some(value) = metadata.get("custom-header") {
        tracing::debug!("mail_interceptor Received custom-header:{:#?}", value);
    }

    Ok(req)
}

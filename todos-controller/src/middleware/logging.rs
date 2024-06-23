use axum::{body::Body, middleware::Next, response::Response};
use http::{Request, StatusCode};
use tokio::time::Instant;

pub async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, std::convert::Infallible> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = Instant::now() - start;
    tracing::debug!("{} {} took {} ms", method, uri, duration.as_millis());

    Ok(response)
}

pub async fn access_log_on_request(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    tracing::debug!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}

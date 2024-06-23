use axum::{http::StatusCode, response::IntoResponse};
use tokio::time::{sleep, Duration};

pub async fn health_check() -> impl IntoResponse {
    tracing::debug!("Access health check endpoint from user!");
    sleep(Duration::from_secs(5)).await;
    StatusCode::OK
}

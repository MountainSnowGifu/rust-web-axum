use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{self, header::SET_COOKIE, request::Parts, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};

use serde::{Deserialize, Serialize};
use tower_sessions::Session;
const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
pub struct Counter(usize);

#[async_trait]
impl<S> FromRequestParts<S> for Counter
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await?;
        let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
        session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
        Ok(counter)
    }
}

pub async fn handler(counter: Counter) -> impl IntoResponse {
    tracing::debug!("handler");
    tracing::debug!("{}", format!("Current count: {}", counter.0));
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_static("key=value2; Path=/; SameSite=Strict; Max-Age=300"),
    );
    (StatusCode::OK, headers, "クッキーが設定されました")
}

use axum::{response::IntoResponse, Json};
use http::StatusCode;

pub type ApiResult<T> = anyhow::Result<T, ApiError>;
pub struct ApiError {
    status: StatusCode,
    response: Json<serde_json::Value>,
}

//ApiErrorには、Into<anyhow::Error>を実装しているエラー（基本的に全てのエラー）から変換できるようにしておく
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(original_error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            response: axum::Json(serde_json::json!({
                "error": format!("{:#?}", original_error.into())
            })),
        }
    }
}

//ApiErrorは、Responseへの変換を行えるようにIntoResponseを実装しておく
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.response).into_response()
    }
}

pub async fn hello() -> ApiResult<impl IntoResponse> {
    Ok((
        StatusCode::OK,
        axum::Json(serde_json::json!({"message": "Hello, World!"})),
    ))
}

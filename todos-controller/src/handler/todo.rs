use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension};
use std::sync::Arc;

use crate::module::{app_state::AppState, modules::Modules};

pub async fn all_todo(Extension(module): Extension<Arc<Modules>>) -> impl IntoResponse {
    let todo = module.todo_use_case().all().await;
    tracing::debug!("{:#?}", todo);
    StatusCode::OK
}

pub async fn get_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    //let mut counter1 = state.counter1.lock().unwrap();
    //let mut counter2 = state.counter2.lock().unwrap();

    let tx = state.tx.lock().await;
    tx.send("test".to_string()).await.unwrap();

    //*counter1 += 1;
    //*counter2 += 10;
}

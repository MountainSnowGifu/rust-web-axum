use crate::{
    authenticate::basic_authenticate::{authenticate, login},
    handler::{
        cookie::handler,
        health::health_check,
        request_id::{OneKitRequestId, ONE_KIT_REQUEST_ID},
        todo::{all_todo, get_state},
    },
    middleware::logging::logging_middleware,
    module::{app_state::AppState, modules::Modules},
};
use axum::{routing::get, Extension, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};
use tracing_subscriber;

//https://zenn.dev/scirexs/articles/c467a911218593

pub async fn create_app(modules: Arc<Modules>, app_state: Arc<AppState>) {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(300)));

    let hc_router = Router::new().route("/health", get(health_check));
    let todo_router = Router::new().route("/one-kit", get(all_todo));
    let handler_router = Router::new().route("/handler", get(handler));
    let get_state_router = Router::new().route("/get-state", get(get_state));
    let auth_login_router = Router::new().route("/login", get(login));
    let auth_user_router = Router::new().route("/user", get(login));

    let app = Router::new()
        .nest("/auth", auth_login_router)
        .nest("/auth", auth_user_router)
        .nest("/api", hc_router)
        .nest("/api", todo_router)
        .nest("/api", handler_router)
        .nest("/api", get_state_router)
        .with_state(app_state)
        .layer(Extension(modules))
        .layer(session_layer)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    ONE_KIT_REQUEST_ID.clone(),
                    OneKitRequestId::new(),
                ))
                .layer(PropagateRequestIdLayer::new(ONE_KIT_REQUEST_ID.clone()))
                .layer(axum::middleware::from_fn(logging_middleware))
                .layer(axum::middleware::from_fn(authenticate)),
        );

    //let error_log = std::sync::Arc::new(std::fs::File::create("./log/one_kit_error.log").unwrap());
    //let writer = || std::io::stderr();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_names(true) // default=false
        .with_thread_ids(true) // default=false
        .with_file(true) // default=false
        .with_line_number(true) // default=false
        .with_ansi(false) // default=true; to avoid text garbling
        //.with_writer(error_log)
        //.json()
        //.with_writer(writer)
        .init();

    tracing::debug!("Starting server on");
    tracing::debug!("http://localhost:8080");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

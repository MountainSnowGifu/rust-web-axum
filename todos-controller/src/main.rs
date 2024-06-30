use std::sync::Arc;
use todos_controller::{
    module::{app_state::AppState, modules::Modules},
    setup::{axum_server_create_app::create_app, grpc_server_create_app::start_grpc_server},
    worker::one_kit_worker::worker_start,
};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let grpc_module = Modules::new().await;
    tokio::spawn(async move {
        start_grpc_server(Arc::new(grpc_module)).await.unwrap();
    });

    let (mpsc_channel_tx, mpsc_channel_rx) = mpsc::channel(64);
    let (oneshot_channel_tx, oneshot_channel_rx) = oneshot::channel();

    worker_start(mpsc_channel_rx, oneshot_channel_rx);
    let _ = oneshot_channel_tx.send("test2".to_string()).unwrap();

    let web_app_state = AppState::new(mpsc_channel_tx).await;
    let web_app_module = Modules::new().await;
    let _ = create_app(Arc::new(web_app_module), Arc::new(web_app_state)).await;

    Ok(())
}

use std::sync::Arc;
use todos_controller::{
    module::{app_state::AppState, modules::Modules},
    setup::create_app,
    worker::one_kit_worker::worker_start,
};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (mpsc_channel_tx, mpsc_channel_rx) = mpsc::channel(64);
    let (oneshot_channel_tx, oneshot_channel_rx) = oneshot::channel();

    worker_start(mpsc_channel_rx, oneshot_channel_rx);
    let _ = oneshot_channel_tx.send("test2".to_string()).unwrap();

    let app_state = AppState::new(mpsc_channel_tx).await;
    let module = Modules::new().await;
    let _ = create_app(Arc::new(module), Arc::new(app_state)).await;
    Ok(())
}

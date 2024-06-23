use std::sync::Arc;

use axum::extract::FromRef;
use tokio::sync::{mpsc::Sender, Mutex};

#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub counter1: Arc<Mutex<i32>>,
    pub tx: Arc<Mutex<Sender<String>>>,
}

impl AppState {
    pub async fn new(tx: Sender<String>) -> Self {
        AppState {
            counter1: Arc::new(Mutex::new(0)),
            tx: Arc::new(Mutex::new(tx)),
        }
    }
}

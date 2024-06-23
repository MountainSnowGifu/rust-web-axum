use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

pub fn worker_start(mut mpsc_rx: mpsc::Receiver<String>, oneshot_rx: oneshot::Receiver<String>) {
    tokio::spawn(async move {
        while let Some(msg) = mpsc_rx.recv().await {
            println!("Received: {}", msg);
            sleep(Duration::from_secs(60)).await;
        }
    });

    tokio::spawn(async move {
        match oneshot_rx.await {
            Ok(v) => println!("got = {:?}", v),
            Err(_) => println!("the sender dropped"),
        }
    });
}

use axum::Router;
use tokio::{
    net::TcpListener,
    sync::mpsc::{self, Sender},
    task::JoinHandle,
};

pub fn start_server_with_graceful_shutdown_channel(
    tcp_listener: TcpListener,
    router: Router,
) -> (JoinHandle<()>, Sender<()>) {
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let sender = tx.clone();

    let handler = tokio::task::spawn(async move {
        let tx = tx.clone();

        axum::serve(tcp_listener, router)
            .with_graceful_shutdown(async move {
                rx.recv().await;
            })
            .await
            .expect("start axum server failed");

        tx.send(()).await.expect("send message failed");
    });

    (handler, sender)
}

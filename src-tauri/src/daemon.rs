use axum::{routing::get, Router};

async fn pong() -> &'static str {
    "pong"
}

pub async fn start() {
    let app = Router::new().route("/ping", get(pong));

    let addr = "127.0.0.1:19750";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("daemon http server started at {}", addr);
    axum::serve(listener, app).await.unwrap();
}

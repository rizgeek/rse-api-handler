use tokio::{net::TcpListener, signal};

use crate::routes::routes::routes;

pub async fn server() {
    let app = routes().await;

    let listener = TcpListener::bind("127.0.0.1:4444")
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    println!("shutdown signal received. Cleaning up..")
}
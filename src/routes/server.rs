use tokio::net::TcpListener;

use crate::routes::routes::routes;

pub async fn server() {
    let app = routes().await;

    let listener = TcpListener::bind("127.0.0.1:4444")
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server Error");
}
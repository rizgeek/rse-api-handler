use std::env;

use dotenv::dotenv;
use tokio::{net::TcpListener, signal};

use crate::routes::routes::routes;

struct ApiConfig {
    port: String,
    host: String
}

impl ApiConfig {
    fn new(port: String, host: String) -> Self {
        Self { port, host }
    }

    fn bind(self) -> String  {
        format!("{}:{}",self.host, self.port)
    }
}


pub async fn server() {
    dotenv().ok();

    let app = routes().await;

    let port = env::var("PORT").expect("PORT must be set in .env");
    let host = env::var("HOST").expect("HOST must be set in .env");
    
    let bind = ApiConfig::new(port, host).bind();

    let listener = TcpListener::bind(bind)
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
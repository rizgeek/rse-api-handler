use std::env;
use std::sync::Arc;

use tokio::{net::TcpListener, signal};

use crate::routes::routes::routes;
use crate::helper::notif;
use crate::server::app_state::AppState;
use crate::server::build_state::build_state;

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
    let state: Arc<AppState> = build_state();

    let app= routes().with_state(state);

    let port: String = env::var("PORT").expect("PORT must be set in .env");
    let host: String = env::var("HOST").expect("HOST must be set in .env");
    
    notif::info("starting server host {?}",Some(host.as_str()));
    notif::info("opening port {?}",Some(port.as_str()));

    let bind: String = ApiConfig::new(port, host).bind();
    
    let listener: TcpListener = TcpListener::bind(bind)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    notif::success("shutdown signal received. Cleaning up..", None)
}
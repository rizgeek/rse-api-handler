use std::sync::Arc;

use axum::{routing::get, Router};
use crate::{handler::global, server::app_state::AppState};

pub fn global() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(global::hdl_health))
}
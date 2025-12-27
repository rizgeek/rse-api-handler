use axum::{routing::get, Router};
use crate::handler::global;

pub fn global() -> Router {
    Router::new()
        .route("/health", get(global::hdl_health))
}
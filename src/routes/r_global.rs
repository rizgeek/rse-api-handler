use axum::{routing::get, Router};
use crate::handler::global;

pub async fn global() -> Router {
    let app: Router = Router::new()
        .route("/health", get(global::hdl_health));

    app
}
use axum::{routing::get, Router};

use crate::handler::global::handler_hello_world;

pub async fn routes() -> Router {
    let app: Router = Router::new()
        .route("/", get(handler_hello_world));

    app
}
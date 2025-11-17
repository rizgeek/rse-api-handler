use axum::{routing::get, Router};

use crate::handler::global::handler_hello_world;

async fn global() -> Router {
    let app: Router = Router::new()
        .route("/hello", get(handler_hello_world));

    app
}

pub async fn routes() -> Router {
    let global = global().await;

    let app: Router = Router::new()
        .nest("/global", global);

    app
}
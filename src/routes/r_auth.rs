use axum::{routing::post, Router};
use crate::handler::auth;

pub async fn auth() -> Router<()> {
    Router::new()
        .route("/register", post(auth::hdl_register))
}

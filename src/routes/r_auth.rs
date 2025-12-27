
use std::sync::Arc;

use axum::{routing::post, Router};
use crate::{handler::auth, server::app_state::AppState};

pub fn auth() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth::hdl_register))
}

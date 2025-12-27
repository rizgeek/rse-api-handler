use std::sync::Arc;

use axum::Router;
use crate::{routes::{r_auth, r_global}, server::app_state::AppState};

pub  fn routes() -> Router<Arc<AppState>> {
    let global = r_global::global();
    let auth = r_auth::auth();

    Router::new()
    .nest("/auth", auth)
    .nest("/global", global)
}
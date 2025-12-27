use axum::Router;
use crate::routes::{r_auth, r_global};

pub  fn routes() -> Router {
    let global = r_global::global();
    let auth = r_auth::auth();

    Router::new()
    .nest("/auth", auth)
    .nest("/global", global)
}
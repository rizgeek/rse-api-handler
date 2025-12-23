use axum::Router;
use crate::routes::{r_global, r_auth};

pub async fn routes() -> Router {
    let global = r_global::global().await;
    let auth = r_auth::auth().await;

    let app: Router = Router::new()
        .nest("/global", global)
        .nest("/auth", auth);

    app
}
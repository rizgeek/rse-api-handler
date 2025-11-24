use axum::Router;
use crate::routes;

pub async fn routes() -> Router {
    let global = routes::r_global::global().await;

    let app: Router = Router::new()
        .nest("/global", global);

    app
}
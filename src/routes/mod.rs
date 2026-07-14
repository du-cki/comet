use std::sync::Arc;

use axum::{Router, routing::get};

use sqlx::{Pool, Sqlite};

pub fn new(pool: Arc<Pool<Sqlite>>) -> Router {
    let app = Router::new()
        .route("/ping", get(|| async {}))
        .with_state(pool);

    return app;
}

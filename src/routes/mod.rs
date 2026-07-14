use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

mod does_any_user_exist;
mod login;
mod register;

pub fn with_state(pool: Arc<super::models::AppState>) -> Router {
    let app = Router::new()
        .route("/ping", get(|| async {}))
        .route("/does-any-user-exist", get(does_any_user_exist::route))
        .route("/login", post(login::route))
        .route("/register", post(register::route))
        .with_state(pool);

    return app;
}

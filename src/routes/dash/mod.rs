use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::models::AppState;
use crate::route;

mod pineapple;

pub fn new(state: AppState) -> Router {
    return Router::new()
        .route(&route!("/api/ws/pineapple"), get(pineapple::route))
        .nest_service(
            &route!(&state.config.dashboard.path),
            ServeFile::new("ui/dist/index.html"),
        )
        .nest_service("/assets/", ServeDir::new("ui/dist/assets"))
        // .nest_service("/*", ServeDir::new("ui/dist/public"))
        .with_state(state);
}

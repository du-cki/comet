use std::sync::Arc;

use axum::{
    body::BoxBody,
    extract::{State, DefaultBodyLimit},
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use sqlx::{Pool, Sqlite};

use crate::models::AppState;
use crate::settings::Settings;

mod delete_file;
mod get_file;
mod upload_file;
mod ping;

async fn authenticated_routes<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Response<BoxBody> {
    if let Some(raw_token) = request.headers().get("Authorization") {
        if let Ok(token) = raw_token.to_str() {
            if state.config.password == token {
                return next.run(request).await.into_response();
            }
        }
    }

    Response::builder()
        .status(401)
        .body(BoxBody::default())
        .unwrap()
}

pub fn create(pool: Arc<Pool<Sqlite>>, config: &Settings) -> Router {
    let state = AppState {
        pool,
        config: config.clone(),
    };

    let file_size_limit = {
        if config.file_size_limit > 0 {
            DefaultBodyLimit::max(config.file_size_limit)
        } else {
            DefaultBodyLimit::disable()
        }
    };

    let inner = Router::new() // these will be authenticated routes.
        .route(
            &format!("{}:media_id", state.config.endpoints.delete_file),
            delete(delete_file::route),
        )
        .route(&state.config.endpoints.upload_file, post(upload_file::route))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authenticated_routes,
        ))
        .with_state(state.clone());

    Router::new()
        .merge(inner)
        .layer(file_size_limit)
        .route(
            &format!("{}:media_id", state.config.endpoints.get_file),
            get(get_file::route),
        )
        .route(
            &state.config.endpoints.ping,
            get(ping::route)
        )
        .with_state(state)
}

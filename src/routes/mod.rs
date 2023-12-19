use std::sync::Arc;

use axum::{
    body::BoxBody,
    extract::{DefaultBodyLimit, State},
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use sqlx::{Pool, Sqlite};

use tokio::sync::broadcast;
use tower_http::services::{ServeDir, ServeFile};

use crate::models::{AppState, FileRecord};
use crate::settings::Settings;

mod delete_file;
mod get_file;
mod pineapple;
mod ping;
mod upload_file;

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
    let (sx, _) = broadcast::channel::<FileRecord>(1);

    let state = AppState {
        pool,
        config: config.clone(),
        sx,
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
            &format!("{}:file_name", state.config.endpoints.delete_file),
            delete(delete_file::route),
        )
        .route(
            &state.config.endpoints.upload_file,
            post(upload_file::route),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authenticated_routes,
        ))
        .with_state(state.clone());

    Router::new()
        .merge(inner)
        .layer(file_size_limit)
        .route(
            &format!("{}:file_name", state.config.endpoints.get_file),
            get(get_file::route),
        )
        .route(&state.config.endpoints.ping, get(ping::route))
        .route("/api/ws/pineapple", get(pineapple::route))
        .nest_service("/assets/", ServeDir::new("ui/dist/assets"))
        .fallback_service(ServeFile::new("ui/dist/index.html"))
        .with_state(state)
}

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

use crate::models::{AppState, FileRecord};
use crate::settings::Settings;

use crate::route;

mod dash;
mod delete_file;
mod get_file;
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

pub fn new(pool: Arc<Pool<Sqlite>>, config: &Settings) -> Router {
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

    let inner = Router::new() // authenticated routes.
        .route(
            &route!(&state.config.api_endpoints.delete, "/:file_name"),
            delete(delete_file::route),
        )
        .route(
            &route!(&state.config.api_endpoints.upload),
            post(upload_file::route),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authenticated_routes,
        ))
        .with_state(state.clone());

    let base = Router::new()
        .merge(inner)
        .layer(file_size_limit)
        .route(
            &route!(&state.config.api_endpoints.get, "/*file"),
            get(get_file::route),
        )
        .route(&route!(&state.config.api_endpoints.ping), get(ping::route))
        .with_state(state.clone());

    if state.config.dashboard.enabled {
        return base.merge(dash::new(state.clone()));
    }

    base
}

use axum::{
    Router,
    http::Request,
    extract::State,
    response::{Response, IntoResponse},
    body::BoxBody,
    middleware::{self, Next},
    routing::{delete, get, post},
};
use sqlx::{Pool, Sqlite};

use crate::models::AppState;
use crate::settings::Settings;

mod delete_file;
mod get_file;
mod upload_file;

async fn authenticated_routes<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>
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

pub fn create(pool: Pool<Sqlite>, config: &Settings) -> Router {
    let state = AppState {
        pool,
        config: config.clone(),
    };

    let inner = Router::new() // these will be authenticated routes.
        .route(
            &format!("{}:media_id", config.endpoints.delete_file),
            delete(delete_file::route),
        )
        .route(
            &config.endpoints.upload_file,
            post(upload_file::route)
        )
        .layer(
            middleware::from_fn_with_state(
                state.clone(),
                authenticated_routes
            )
        )
        .with_state(
            state.clone()
        );

    Router::new()
        .route(
            &format!("{}:media_id", config.endpoints.get_file),
            get(get_file::route),
        )
        .merge(inner)
        .with_state(state)
}

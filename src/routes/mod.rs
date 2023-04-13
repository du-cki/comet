use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::{Pool, Sqlite};

use crate::models::AppState;
use crate::settings::Settings;

mod delete_file;
mod get_file;
mod upload_file;

use delete_file::delete_file;
use get_file::get_file;
use upload_file::upload_file;

pub fn create_routes(pool: Pool<Sqlite>, config: &Settings) -> Router {
    let state = AppState {
        pool,
        config: config.clone(),
    };

    Router::new()
        .route(
            &format!("{}:media_id", config.endpoints.get_file),
            get(get_file),
        )
        .route(
            &format!("{}:media_id", config.endpoints.delete_file),
            delete(delete_file),
        )
        .route(&config.endpoints.upload_file, post(upload_file))
        .with_state(state)
}

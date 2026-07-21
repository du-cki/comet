use std::sync::Arc;

use axum::{Json, extract::State};

use crate::models::AppState;

pub(crate) async fn route(State(state): State<Arc<AppState>>) -> Json<bool> {
    let user_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);

    Json(user_count > 0)
}

use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::QueryBuilder;

use crate::{
    models::{AppState, ErrorResponse, Role, Settings},
    utils::internal_error,
};

pub async fn get_settings(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<Settings>, (StatusCode, Json<ErrorResponse>)> {
    let role: i64 = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?;

    if role != Role::Admin.weight() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "You cannot access this route.".to_string(),
            }),
        ));
    }

    let settings = sqlx::query_as::<_, Settings>("SELECT * FROM settings WHERE id = 1")
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(Json(settings))
}

#[derive(Deserialize)]
pub struct UpdateSettingsPayload {
    allow_public_signups: Option<bool>,
    require_2fa: Option<bool>,
    maintenance_mode: Option<bool>,
    max_upload_size_mb: Option<i64>,
    enforce_file_extensions: Option<bool>,
    file_name_length: Option<i64>,
}

pub async fn patch_settings(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
    Json(payload): Json<UpdateSettingsPayload>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let role: i64 = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(internal_error)?;

    if role != Role::Admin.weight() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "You cannot access this route.".to_string(),
            }),
        ));
    }

    let mut query = QueryBuilder::new("UPDATE settings SET ");
    let mut separated = query.separated(", ");
    let mut has_updates = false;

    if let Some(val) = payload.allow_public_signups {
        separated.push("allow_public_signups = ");
        separated.push_bind_unseparated(val);
        has_updates = true;
    }

    if let Some(val) = payload.require_2fa {
        separated.push("require_2fa = ");
        separated.push_bind_unseparated(val);
        has_updates = true;
    }

    if let Some(val) = payload.maintenance_mode {
        separated.push("maintenance_mode = ");
        separated.push_bind_unseparated(val);
        has_updates = true;
    }

    if let Some(val) = payload.max_upload_size_mb {
        let db_val = if val <= 0 { None } else { Some(val) };

        separated.push("max_upload_size_mb = ");
        separated.push_bind_unseparated(db_val);
        has_updates = true;
    }

    if let Some(val) = payload.enforce_file_extensions {
        separated.push("enforce_file_extensions = ");
        separated.push_bind_unseparated(val);
        has_updates = true;
    }

    if let Some(val) = payload.file_name_length {
        separated.push("file_name_length = ");
        separated.push_bind_unseparated(val);
        has_updates = true;
    }

    if !has_updates {
        return Ok(StatusCode::OK);
    }

    query.push(" WHERE id = 1");

    query
        .build()
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

    Ok(StatusCode::OK)
}

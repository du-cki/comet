use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::{Extension, Json, extract::State, http::StatusCode};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::{
    jwt::generate_api_token,
    models::{AppState, ErrorResponse},
    utils::internal_error,
};

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let record = sqlx::query!(
        r#"
            SELECT 
                password 
            FROM users
                WHERE id = ?
        "#,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?
    .ok_or((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: "User not found".to_string(),
        }),
    ))?;

    let parsed_hash = PasswordHash::new(&record.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Invalid hash format in database".to_string(),
            }),
        )
    })?;

    let is_valid = Argon2::default()
        .verify_password(payload.current_password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Incorrect current password".to_string(),
            }),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let new_hashed_password = Argon2::default()
        .hash_password(payload.new_password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to hash new password".to_string(),
                }),
            )
        })?
        .to_string();

    sqlx::query!(
        "UPDATE users SET password = ? WHERE id = ?",
        new_hashed_password,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update database".to_string(),
            }),
        )
    })?;

    Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    api_key: String,
}

pub async fn reset_api_key(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<ApiKeyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let new_api_key = generate_api_token();

    sqlx::query!(
        "UPDATE users SET api_key = ? WHERE id = ?",
        new_api_key,
        user_id
    )
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(ApiKeyResponse {
        api_key: new_api_key,
    }))
}

use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{
    jwt::create_jwt,
    models::{AppState, AuthResponse, ErrorResponse},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub(crate) async fn route(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query!(
        "SELECT id, password, role FROM users WHERE email = ?",
        &payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database error".into(),
            }),
        )
    })?;

    let user = user.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: "Invalid credentials".into(),
        }),
    ))?;

    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Invalid hash".into(),
            }),
        )
    })?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid credentials".into(),
                }),
            )
        })?;

    let token = create_jwt(user.id, user.role, &state.jwt_secret).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Token creation failed".into(),
            }),
        )
    })?;

    Ok(Json(AuthResponse { token }))
}

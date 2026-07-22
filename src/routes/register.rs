use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use axum::{Json, extract::State, http::StatusCode};
use rand::{Rng, distributions::Alphanumeric, rngs::OsRng};
use serde::Deserialize;

use crate::{
    jwt::{create_jwt, current_timestamp},
    models::{AppState, AuthResponse, ErrorResponse, Role},
    utils::internal_error,
};

#[derive(Deserialize)]
pub struct SignupRequest {
    name: String,
    email: String,
    password: String,
}

pub(crate) async fn route(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let config =
        sqlx::query!("SELECT allow_public_signups, maintenance_mode FROM settings WHERE id = 1")
            .fetch_one(&state.db)
            .await
            .map_err(internal_error)?;

    if config.maintenance_mode {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "This app is in maintenance mode.".to_string(),
            }),
        ));
    }

    let user_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);

    let role = if user_count == 0 {
        Role::Admin
    } else {
        if !config.allow_public_signups {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Public signups are disabled".into(),
                }),
            ));
        }

        Role::User
    }
    .weight();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Hashing failed".into(),
                }),
            )
        })?
        .to_string();

    let api_key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let created_at = current_timestamp();
    let insert_result = sqlx::query!(
        "INSERT INTO users (name, email, password, api_key, role, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        payload.name,
        payload.email,
        password_hash,
        api_key,
        role,
        created_at
    )
    .execute(&state.db)
    .await;

    match insert_result {
        Ok(result) => {
            let user_id = result.last_insert_rowid();

            let token = create_jwt(user_id, role, &state.jwt_secret).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Token creation failed".into(),
                    }),
                )
            })?;

            Ok(Json(AuthResponse { token }))
        }
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Email already exists".into(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database error".into(),
            }),
        )),
    }
}

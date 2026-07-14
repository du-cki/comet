use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

pub enum Role {
    Admin = 1,
    User = 2,
}

pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(FromRow)]
pub struct DbUser {
    pub id: i64,
    pub password: String,
    pub role: i64,
}

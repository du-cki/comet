use serde::Serialize;
use sqlx::{Pool, Sqlite};

use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub config: Settings,
}

#[derive(Serialize, Debug)]
pub struct APIError {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UploadResponse {
    pub file: String,
    pub file_url: String,
    pub file_size: usize,
}

#[derive(Serialize, Debug)]
pub struct GenericResponse {
    // a generic response, i don't want to type 30 different payloads for each endpoints.
    pub message: String,
}

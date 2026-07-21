use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};
use tokio::sync::broadcast;

pub enum Role {
    Admin = 1,
    User = 2,
}

pub struct Config {
    pub file_save_path: String,
    pub file_name_length: usize,
    pub enforce_file_extensions: bool,
}

pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
    pub config: Config,
    pub tx: broadcast::Sender<UserBroadcast>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum WsEvent {
    DashboardStats {
        total_files: i64,
        total_files_trend: i64,

        storage_used_bytes: i64,
        storage_used_bytes_trend: i64,

        average_file_size_bytes: i64,
        views: i64,
    },
    UploadsList {
        items: Vec<MediaItem>,
        next_cursor: Option<String>,
    },
    FileUpload(MediaItem),
    FileDelete(String),
    Error(String),
}

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum WSClientCommand {
    GetStats,
    GetUploads {
        cursor: Option<String>,
        limit: Option<i64>,
    },
}

#[derive(Clone, Debug)]
pub struct UserBroadcast {
    pub user_id: i64,
    pub event: WsEvent,
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

#[derive(Clone, Debug, Serialize)]
pub struct MediaItem {
    pub media_id: String,
    pub file_url: String,
    pub file_size: i64,
    pub uploaded_at: i64,
    pub content_type: String,
    pub original_file_name: Option<String>,
}

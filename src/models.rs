use std::{sync::Arc, fmt};

use tokio::sync::broadcast::Sender;
use serde::{Deserialize, Serialize};

use sqlx::{Pool, Sqlite};

use crate::settings::Settings;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Events {
    QueryFolder,
    FileUpload,

    // Future "protocols" I would like to implement.
    UserJoinedMesh,
    UserLeftMesh,
    FileSendRequest,
    FileRecieveRequest,
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GenericRequest {
    pub event: Events,
    pub request_id: String,
    pub data: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Folder {
    pub id: Option<i64>,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    pub file_id: i64,
    pub file_name: String,
    pub file_ext: Option<String>,
    pub last_updated: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct FileRecord {
    pub original_file_name: Option<String>,
    pub file_name: Option<String>,
    pub file_ext: Option<String>,
    pub folder_id: Option<i64>,
    pub file_id: i64,
    pub last_updated: i64,
}

#[derive(Deserialize)]
pub struct UploadQuery {
    pub public: Option<bool>,
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

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Arc<Pool<Sqlite>>,
    pub config: Settings,
    pub sx: Sender<FileRecord>,
}

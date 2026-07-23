use axum::{Json, http::StatusCode};

use nanoid::nanoid;
use std::{ffi::OsStr, path::Path};
use tokio::fs;

use tracing::*;

use crate::models::ErrorResponse;

pub fn internal_error<E>(err: E) -> (StatusCode, Json<ErrorResponse>)
where
    E: std::error::Error,
{
    error!("Something went wrong: {:#?}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: "Something went wrong.".to_string(),
        }),
    )
}

pub fn generate_file_path(
    length: usize,
    base_path: &str,
    file_hash: &str,
    raw_file_ext: &Option<&str>,
) -> (String, String) {
    let file_name = nanoid!(length);
    let mut fp = format!("{}{}", base_path, file_hash);

    if let Some(file_ext) = raw_file_ext {
        fp = fp + "." + file_ext;
    }

    (file_name, fp)
}

pub fn parse_filename(filename: &String) -> (Option<&str>, Option<&str>) {
    let path = Path::new(filename);

    (
        path.file_stem().and_then(OsStr::to_str),
        path.extension().and_then(OsStr::to_str),
    )
}

pub struct TempFileGuard {
    path: String,
    armed: bool,
}

impl TempFileGuard {
    pub fn new(path: String) -> Self {
        Self { path, armed: true }
    }

    pub fn disarm(&mut self) {
        self.armed = false
    }
}

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        if self.armed {
            let path = std::mem::take(&mut self.path);

            tokio::spawn(async move {
                if let Err(e) = fs::remove_file(&path).await {
                    error!("failed to clean up temp file {path}: {e}");
                };
            });
        }
    }
}

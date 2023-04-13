use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use nanoid::nanoid;
use std::{ffi::OsStr, path::Path};

use tracing::*;

use crate::{models::APIError, settings::Settings};

pub fn auth(headers: &HeaderMap, settings: &Settings) -> Result<(), (StatusCode, Json<APIError>)> {
    if let Some(raw_token) = headers.get("Authorization") {
        if let Ok(token) = raw_token.to_str() {
            if settings.password == token {
                return Ok(());
            }
        }
    }

    Err((
        StatusCode::UNAUTHORIZED,
        Json(APIError {
            message: "Unauthorised".to_string(),
        }),
    ))
}

pub fn path_exists(path: &String) -> bool {
    if let Ok(_) = std::fs::metadata(path) {
        // i doub't this would block much.
        return true;
    };

    false
}

pub fn internal_error<E>(err: E) -> (StatusCode, Json<APIError>)
where
    E: std::error::Error,
{
    error!("Something went wrong: {:#?}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(APIError {
            message: "Something went wrong.".to_string(),
        }),
    )
}

pub fn generate_file_path(
    length: usize,
    base_path: String,
    raw_file_ext: &Option<&str>,
) -> (String, String) {
    let file_name = nanoid!(length);
    let mut fp = format!("{}{}", base_path, file_name);

    if let Some(file_ext) = raw_file_ext {
        fp = fp + "." + file_ext; // in cases of file not having a file extension, we can safely handle it like this.
    }

    if path_exists(&fp) {
        return generate_file_path(length, base_path, raw_file_ext);
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

use nanoid::nanoid;
use axum::{http::StatusCode, Json};

use std::{ffi::OsStr, path::Path};

use tracing::*;

use crate::models::APIError;

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

pub fn strip_first_and_last(target: String) -> String {
    let mut chars = target.chars();
    chars.next();
    chars.next_back();

    chars.collect()
}

pub fn generate_file_path(
    length: usize,
    base_path: String,
    file_hash: &String,
    raw_file_ext: &Option<&str>,
) -> (String, String) {
    let file_name = nanoid!(length);
    let mut fp = format!("{}{}", base_path, file_hash);

    if let Some(file_ext) = raw_file_ext {
        fp = fp + "." + file_ext; // in cases of file not having a file extension, we can safely handle it like this.
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

#[macro_export]
macro_rules! json_message {
    ($($key:expr => $value:expr),* $(,)?) => {
        axum::extract::ws::Message::Text(serde_json::json!({
            $($key: $value,)*
        }).to_string())
    };
}

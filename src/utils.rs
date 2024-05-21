use axum::{http::StatusCode, Json};
use std::{ffi::OsStr, path::Path};

use crate::models::APIError;

pub fn internal_error<E>(err: E) -> (StatusCode, Json<APIError>)
where
    E: std::error::Error,
{
    tracing::error!("Something went wrong: {:#?}", err);

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

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub parent: Option<&'a str>,
    pub file_name: Option<&'a str>,
    pub ext: Option<&'a str>,
}

impl<'a> FileInfo<'a> {
    pub fn from_str(data: &'a str) -> Self {
        let path = Path::new(data);

        Self {
            parent: path.parent().and_then(|path| path.as_os_str().to_str()),
            file_name: path.file_stem().and_then(OsStr::to_str),
            ext: path.extension().and_then(OsStr::to_str)
        }
    }
}

#[macro_export]
macro_rules! json_message {
    ($($key: expr => $value: expr),* $(,)?) => {
        axum::extract::ws::Message::Text(serde_json::json!({
            $($key: $value,)*
        }).to_string())
    };
}

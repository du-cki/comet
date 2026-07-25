use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{AppendHeaders, IntoResponse, Response},
};

use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use super::AppState;
use crate::{
    models::ErrorResponse,
    utils::{internal_error, parse_filename},
};

pub async fn route(
    Path(raw_media_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let config = sqlx::query!(
        r#"
            SELECT
                enforce_file_extensions
            FROM
                settings WHERE id = 1
        "#
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    let mut ext: Option<&str> = None;
    let mut search_with_ext = false;
    let mut media_id = raw_media_id.clone();

    if config.enforce_file_extensions {
        search_with_ext = true;
        if let (Some(parsed_media_id), Some(parsed_ext)) = parse_filename(&raw_media_id) {
            media_id = parsed_media_id.to_string();
            ext = Some(parsed_ext);
        }
    }

    let res = sqlx::query!(
        r#"
        SELECT file_path, content_type FROM media
        WHERE media_id = ? AND (
            ? = 0 OR file_ext = ?
        )
        "#,
        media_id,
        search_with_ext as i32,
        ext
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    if let Some(query) = res {
        let file = match File::open(&query.file_path).await {
            Ok(file) => file,
            Err(_) => {
                sqlx::query!(
                    r#"
                    DELETE FROM media
                    WHERE file_hash IN (
                        SELECT file_hash FROM media
                        WHERE media_id = ?
                    )
                    "#,
                    media_id
                )
                .execute(&state.db)
                .await
                .map_err(internal_error)?;

                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: "File not found.".to_owned(),
                    }),
                ));
            }
        };

        let headers = AppendHeaders([(header::CONTENT_TYPE, query.content_type)]);

        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);

        return Ok((headers, body).into_response());
    };

    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "File not found.".to_owned(),
        }),
    ))
}

use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::{header, StatusCode},
    response::AppendHeaders,
    Json,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use super::AppState;
use crate::{
    models::APIError,
    utils::{internal_error, parse_filename},
};

pub async fn route(
    Path(raw_media_id): Path<String>,
    State(state): State<AppState>,
) -> Result<
    (
        AppendHeaders<[(header::HeaderName, String); 1]>,
        StreamBody<ReaderStream<File>>,
    ),
    (StatusCode, Json<APIError>),
> {
    let mut ext: Option<&str> = None;
    let mut search_with_ext = false;
    let mut media_id = raw_media_id.clone();

    if state.config.enforce_file_extensions {
        search_with_ext = true;
        if let (Some(parsed_media_id), Some(parsed_ext)) = parse_filename(&raw_media_id) {
            media_id = parsed_media_id.to_string(); // strips off the filename.
            ext = Some(parsed_ext);
        }
    }

    let res = sqlx::query!(
        "
        SELECT file_path, content_type FROM media
            WHERE media_id = ? AND (
                    CASE WHEN ? IS NULL THEN 1 ELSE file_ext = ? END
                )
    ",
        media_id,
        search_with_ext,
        ext
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_error)?;

    if let Some(query) = res {
        let file = match File::open(query.file_path).await {
            Ok(file) => file,
            Err(_) => {
                sqlx::query!(
                    "
                   DELETE FROM media WHERE media_id = ?
                ",
                    media_id
                ) // if the file has been tampered with.
                .execute(&state.pool)
                .await
                .map_err(internal_error)?;

                return Err((
                    StatusCode::NOT_FOUND,
                    Json(APIError {
                        message: "File not found.".to_owned(),
                    }),
                ));
            }
        };

        let headers = AppendHeaders([(header::CONTENT_TYPE, query.content_type)]);

        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        return Ok((headers, body));
    };

    Err((
        StatusCode::NOT_FOUND,
        Json(APIError {
            message: "File not found.".to_owned(),
        }),
    ))
}

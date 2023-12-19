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
    Path(raw_file_name): Path<String>,
    State(state): State<AppState>,
) -> Result<
    (
        AppendHeaders<[(header::HeaderName, String); 1]>,
        StreamBody<ReaderStream<File>>,
    ),
    (StatusCode, Json<APIError>),
> {
    let mut ext: Option<&str> = None;
    let mut file_name = raw_file_name.clone();

    if state.config.enforce_file_extensions {
        if let (Some(parsed_file_name), Some(parsed_ext)) = parse_filename(&raw_file_name) {
            file_name = parsed_file_name.to_string(); // strips off the filename.
            ext = Some(parsed_ext);
        }
    }

    let res = sqlx::query!(
        "
        SELECT file_path, content_type FROM media
            WHERE file_name = ? AND (
                    CASE WHEN ? IS NULL THEN 1 ELSE file_ext = ? END
                )
    ",
        file_name,
        ext, ext
    )
    .fetch_optional(&*state.pool)
    .await
    .map_err(internal_error)?;

    if let Some(query) = res {
        let file = match File::open(query.file_path).await {
            Ok(file) => file,
            Err(_) => {
                sqlx::query!(
                    r#"
                    DELETE FROM media
                        WHERE file_hash IN (
                            SELECT file_hash FROM media
                                WHERE file_name = ?
                        );
                "#,
                    file_name
                ) // if the file has been tampered with.
                .execute(&*state.pool)
                .await
                .map_err(internal_error)?;

                return Err((
                    StatusCode::NOT_FOUND,
                    Json(APIError {
                        message: "file not found".to_owned(),
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
            message: "file not found".to_owned(),
        }),
    ))
}

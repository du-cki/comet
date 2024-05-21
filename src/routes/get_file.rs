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
    utils::{internal_error, FileInfo},
};

pub async fn route(
    Path(abs_file_path): Path<String>,
    State(state): State<AppState>,
) -> Result<
    (AppendHeaders<[(header::HeaderName, String); 1]>,
     StreamBody<ReaderStream<File>>),
    (StatusCode, Json<APIError>),
> {
    let file = FileInfo::from_str(&abs_file_path);

    let res = sqlx::query!("
        SELECT file_path, content_type FROM media
            WHERE file_name = $2 AND (
                CASE WHEN $1 = false THEN 1 ELSE file_ext = $3 END
            ) AND folder_id IS (
                SELECT id FROM folder_paths WHERE path = '/' || $4
            )
    ",
        state.config.enforce_file_extensions,
        file.file_name,
        file.ext,
        file.parent
    )
        .fetch_optional(&*state.pool)
        .await
        .map_err(internal_error)?;

    if let Some(record) = res {
        let Ok(file) = File::open(record.file_path).await else {
            sqlx::query!(r#"
                DELETE FROM media
                    WHERE file_hash IN (
                        SELECT file_hash FROM media
                            WHERE file_name = ?
                    );
                "#,
                file.file_name
            )
                .execute(&*state.pool)
                .await
                .map_err(internal_error)?;

            return Err((
                StatusCode::NOT_FOUND,
                Json(APIError {
                    message: "file not found".to_owned(),
                }),
            ));
        };

        let headers = AppendHeaders([
            (header::CONTENT_TYPE, record.content_type)
        ]);
        let stream = ReaderStream::new(file);

        return Ok((
            headers,
            StreamBody::new(stream)
        ));
    }

    Err((
        StatusCode::NOT_FOUND,
        Json(APIError {
            message: format!("file `/{}` not found", abs_file_path),
        }),
    ))
}

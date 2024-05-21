use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tokio::fs;

use super::AppState;
use crate::{
    models::{APIError, GenericResponse},
    utils::internal_error,
};

pub async fn route(
    Path(file_name): Path<String>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<GenericResponse>), (StatusCode, Json<APIError>)> {
    let query = sqlx::query!(r#"
        SELECT file_path, file_hash,
            (SELECT COUNT(*) FROM media WHERE file_hash = media.file_hash) AS count
        FROM media
            WHERE file_name = $1;
    "#,
        file_name
    )
    .fetch_optional(&*state.pool)
    .await
    .map_err(internal_error)?;

    if let Some(record) = query {
        if record.count == 1 {
            fs::remove_file(record.file_path).await.map_err(internal_error)?;
        }

        sqlx::query!(r#"
            DELETE FROM media
                WHERE file_name = $1
        "#,
            file_name
        )
        .execute(&*state.pool)
        .await
        .map_err(internal_error)?;

        return Ok((
            StatusCode::OK,
            Json(GenericResponse {
                message: "removed".to_string(),
            }),
        ));
    }

    Err((
        StatusCode::NOT_FOUND,
        Json(APIError {
            message: "file not found".to_owned(),
        }),
    ))
}

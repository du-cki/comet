use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use tokio::fs::remove_file;

use super::AppState;
use crate::{
    models::{APIError, GenericResponse},
    utils::{auth, internal_error},
};

pub async fn delete_file(
    Path(media_id): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<GenericResponse>), (StatusCode, Json<APIError>)> {
    auth(&headers, &state.config)?;

    let query = sqlx::query!(
        r#"SELECT file_path FROM media
            WHERE media_id = ?
    "#,
        media_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_error)?;

    if let Some(record) = query {
        if let Ok(_) = remove_file(record.file_path).await {
            sqlx::query!(
                r#"DELETE FROM media
                    WHERE media_id = ?
            "#,
                media_id
            )
            .execute(&state.pool)
            .await
            .map_err(internal_error)?;

            return Ok((
                StatusCode::OK,
                Json(GenericResponse {
                    message: "Removed.".to_string(),
                }),
            ));
        };
    }

    Err((
        StatusCode::NOT_FOUND,
        Json(APIError {
            message: "File not found.".to_owned(),
        }),
    ))
}

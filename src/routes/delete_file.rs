use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tokio::fs::remove_file;

use super::AppState;
use crate::{
    models::{APIError, GenericResponse},
    utils::internal_error,
};

pub async fn route(
    Path(media_id): Path<String>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<GenericResponse>), (StatusCode, Json<APIError>)> {
    let query = sqlx::query!(
        r#"
            SELECT file_path, file_hash,
                (SELECT COUNT(*) FROM media WHERE file_hash = media.file_hash) AS count
            FROM media
                WHERE media_id = ?;
    "#,
        media_id
    )
    .fetch_optional(&*state.pool)
    .await
    .map_err(internal_error)?;

    if let Some(record) = query {
        if record.count == 1 {
            remove_file(record.file_path).await.map_err(internal_error)?;
        }

        sqlx::query!(
                r#"
                DELETE FROM media
                    WHERE media_id = ?
            "#,
            media_id
        )
        .execute(&*state.pool)
        .await
        .map_err(internal_error)?;

        return Ok((
            StatusCode::OK,
            Json(GenericResponse {
                message: "Removed.".to_string(),
            }),
        ));
    }

    Err((
        StatusCode::NOT_FOUND,
        Json(APIError {
            message: "File not found.".to_owned(),
        }),
    ))
}

// if let Ok(_) = remove_file(record.file_path).await {
//     sqlx::query!(
//         r#"DELETE FROM media
//               WHERE media_id = ?
//     "#,
//         media_id
//     )
//     .execute(&*state.pool)
//     .await
//     .map_err(internal_error)?;
// };

use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use tokio::fs::remove_file;

use super::AppState;
use crate::{
    models::{ErrorResponse, UserBroadcast, WsEvent},
    utils::{internal_error, parse_filename},
};

pub async fn route(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
    Path(media_id): Path<String>,
) -> Result<(StatusCode, Json<()>), (StatusCode, Json<ErrorResponse>)> {
    let config =
        sqlx::query!("SELECT enforce_file_extensions, maintenance_mode FROM settings WHERE id = 1")
            .fetch_one(&state.db)
            .await
            .map_err(internal_error)?;

    if config.maintenance_mode {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "This app is in maintenance mode.".to_string(),
            }),
        ));
    }

    let mut media_id = media_id;

    if config.enforce_file_extensions {
        if let (Some(parsed_media_id), _) = parse_filename(&media_id) {
            media_id = parsed_media_id.to_string();
        }
    }

    let query = sqlx::query!(
        r#"
        SELECT 
            file_path, 
            (SELECT COUNT(*) FROM media m2 WHERE m2.file_hash = m1.file_hash) AS "count!: i64"
        FROM media m1
        WHERE m1.media_id = ? AND m1.user_id = ?;
        "#,
        media_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    if let Some(record) = query {
        if record.count == 1 {
            match remove_file(&record.file_path).await {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    tracing::warn!("file {} was already missing from disk.", record.file_path);
                }
                Err(e) => return Err(internal_error(e)),
            }
        }

        sqlx::query!(
            r#"
            DELETE FROM media
            WHERE media_id = ? AND user_id = ?
            "#,
            media_id,
            user_id
        )
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

        let _ = state.tx.send(UserBroadcast {
            user_id,
            event: WsEvent::FileDelete(media_id),
        });

        return Ok((StatusCode::OK, Json(())));
    }

    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "File not found or you do not have permission to delete it.".to_owned(),
        }),
    ))
}

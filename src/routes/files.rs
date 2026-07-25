use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::models::{AppState, ErrorResponse, MediaItem};

#[derive(Serialize)]
pub struct FileResponse {
    items: Vec<MediaItem>,
    next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

pub async fn route(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
    Query(query): Query<QueryParams>,
) -> impl IntoResponse {
    fetch(user_id, query.cursor, query.limit, &state)
        .await
        .map(|r| Json(r))
        .map_err(|error| Json(ErrorResponse { error }))
}

async fn fetch(
    user_id: i64,
    cursor: Option<String>,
    limit: Option<i64>,
    state: &Arc<AppState>,
) -> Result<FileResponse, String> {
    let config = sqlx::query!("SELECT enforce_file_extensions FROM settings WHERE id = 1")
        .fetch_one(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let limit = limit.unwrap_or(20).clamp(1, 50);
    let query_limit = limit + 1;
    let mut cursor_time: Option<i64> = None;

    if let Some(cursor_id) = &cursor {
        cursor_time = sqlx::query_scalar!(
            "SELECT uploaded_at FROM media WHERE media_id = ? AND user_id = ?",
            cursor_id,
            user_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| e.to_string())?;

        if cursor_time.is_none() {
            return Err("invalid cursor provided".to_string());
        }
    }

    let records = sqlx::query!(
        r#"
        SELECT 
            media_id, file_ext, file_size, uploaded_at, content_type, original_file_name
        FROM 
            media
        WHERE 
            user_id = ?
            AND (
                ? IS NULL 
                OR uploaded_at < ? 
                OR (uploaded_at = ? AND media_id < ?)
            )
        ORDER BY 
            uploaded_at DESC, media_id DESC
        LIMIT ?
        "#,
        user_id,
        cursor,
        cursor_time,
        cursor_time,
        cursor,
        query_limit
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    let mut items: Vec<MediaItem> = records
        .into_iter()
        .map(|record| {
            let mut file_url = format!("/view/{}", &record.media_id);
            if config.enforce_file_extensions {
                if let Some(ext) = &record.file_ext {
                    file_url = format!("{}.{}", file_url, ext);
                }
            }
            MediaItem {
                media_id: record.media_id,
                file_url,
                file_size: record.file_size,
                uploaded_at: record.uploaded_at,
                content_type: record.content_type,
                original_file_name: record.original_file_name,
            }
        })
        .collect();

    let mut next_cursor = None;
    if items.len() as i64 > limit {
        items.pop();
        if let Some(last_item) = items.last() {
            next_cursor = Some(last_item.media_id.clone());
        }
    }

    Ok(FileResponse { items, next_cursor })
}

use crate::models::{AppState, WsEvent};
use std::sync::Arc;

pub async fn get_stats(user_id: i64, state: &Arc<AppState>) -> WsEvent {
    let result = sqlx::query!(
        r#"
        SELECT
            COUNT(media_id) as total_files,
            SUM(CASE WHEN uploaded_at >= unixepoch('now', '-1 month') THEN 1 ELSE 0 END) as total_files_trend,

            SUM(file_size) as storage_used_bytes,
            SUM(CASE WHEN uploaded_at >= unixepoch('now', '-1 month') THEN file_size ELSE 0 END) as storage_used_bytes_trend,

            AVG(file_size) as avg_file_size
        FROM media
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => WsEvent::DashboardStats {
            total_files: row.total_files,
            total_files_trend: row.total_files_trend.unwrap_or(0),

            storage_used_bytes: row.storage_used_bytes.unwrap_or(0),
            storage_used_bytes_trend: row.storage_used_bytes_trend.unwrap_or(0),

            average_file_size_bytes: row.avg_file_size.unwrap_or(0),
            views: 0, // TODO
        },
        Err(e) => {
            tracing::error!("failed to fetch stats for user {}: {:?}", user_id, e);

            WsEvent::Error("failed to fetch dashboard stats".into())
        }
    }
}

use std::sync::Arc;

use crate::models::{AppState, FileTypeStat, WsEvent};

pub async fn get_stats(user_id: i64, state: &Arc<AppState>) -> WsEvent {
    let result = sqlx::query!(
        r#"
        SELECT
            COUNT(media_id) as total_files,
            SUM(CASE WHEN uploaded_at >= unixepoch('now', '-1 month') THEN 1 ELSE 0 END) as total_files_trend,

            SUM(file_size) as storage_used_bytes,
            SUM(CASE WHEN uploaded_at >= unixepoch('now', '-1 month') THEN file_size ELSE 0 END) as storage_used_bytes_trend,

            AVG(file_size) as avg_file_size,

            SUM(CASE WHEN content_type LIKE 'image/%' THEN 1 ELSE 0 END) as images_count,
            SUM(CASE WHEN content_type LIKE 'video/%' THEN 1 ELSE 0 END) as videos_count,
            SUM(CASE WHEN content_type LIKE 'audio/%' THEN 1 ELSE 0 END) as audio_count,
            SUM(CASE WHEN 
                content_type LIKE 'text/%' OR 
                content_type LIKE 'application/pdf' OR 
                content_type LIKE 'application/msword' OR 
                content_type LIKE 'application/vnd.%' 
            THEN 1 ELSE 0 END) as documents_count
        FROM media
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let total = row.total_files;

            let images_count = row.images_count.unwrap_or(0);
            let videos_count = row.videos_count.unwrap_or(0);
            let audio_count = row.audio_count.unwrap_or(0);
            let documents_count = row.documents_count.unwrap_or(0);

            let other_count = total - (images_count + videos_count + audio_count + documents_count);

            let file_types = vec![
                FileTypeStat {
                    name: "Images".to_string(),
                    amount: images_count,
                },
                FileTypeStat {
                    name: "Videos".to_string(),
                    amount: videos_count,
                },
                FileTypeStat {
                    name: "Audio".to_string(),
                    amount: audio_count,
                },
                FileTypeStat {
                    name: "Documents".to_string(),
                    amount: documents_count,
                },
                FileTypeStat {
                    name: "Other".to_string(),
                    amount: other_count,
                },
            ];

            WsEvent::DashboardStats {
                total_files: row.total_files,
                total_files_trend: row.total_files_trend.unwrap_or(0),

                storage_used_bytes: row.storage_used_bytes.unwrap_or(0),
                storage_used_bytes_trend: row.storage_used_bytes_trend.unwrap_or(0),

                average_file_size_bytes: row.avg_file_size.unwrap_or(0),
                views: 0, // TODO

                file_types,
            }
        }
        Err(e) => {
            tracing::error!("failed to fetch stats for user {}: {:?}", user_id, e);

            WsEvent::Error("failed to fetch dashboard stats".into())
        }
    }
}

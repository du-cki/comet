use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
};
use lofty::{file::TaggedFileExt, probe::Probe};
use std::{path::Path as StdPath, sync::Arc};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{models::AppState, utils::parse_filename};

pub async fn route(
    State(state): State<Arc<AppState>>,
    Path(media_id): Path<String>,
    req: Request,
) -> Result<impl IntoResponse, StatusCode> {
    let config = sqlx::query!(
        r#"
            SELECT
                enforce_file_extensions
            FROM
                settings WHERE id = 1
        "#
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut media_id = media_id;

    if config.enforce_file_extensions {
        if let (Some(parsed_media_id), _) = parse_filename(&media_id) {
            media_id = parsed_media_id.to_string();
        }
    }

    let query = sqlx::query!(
        r#"
            SELECT
                file_path, file_hash, content_type
            FROM media
                WHERE media_id = ?;
        "#,
        &media_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let original_path = query.file_path;
    let thumb_path = format!("./thumbnails/{}.webp", query.file_hash);

    if !StdPath::new(&thumb_path).exists() {
        if !StdPath::new(&original_path).exists() {
            return Err(StatusCode::NOT_FOUND);
        }

        let thumb = thumb_path.clone();

        tokio::task::spawn_blocking(move || {
            let img = if query.content_type.starts_with("audio/") {
                let tagged_file = Probe::open(&original_path)
                    .and_then(|probe| probe.read())
                    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

                let tag = tagged_file
                    .primary_tag()
                    .or_else(|| tagged_file.first_tag())
                    .ok_or(StatusCode::NOT_FOUND)?;

                let pic = tag.pictures().first().ok_or(StatusCode::NOT_FOUND)?;

                image::load_from_memory(pic.data()).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?
            } else {
                image::open(&original_path).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?
            };

            let thumbnail = img.thumbnail(400, 400);

            thumbnail
                .save(&thumb)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)??;
    }

    match ServeFile::new(&thumb_path).oneshot(req).await {
        Ok(res) => Ok(res),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

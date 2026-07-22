use axum::{
    Json,
    extract::{Extension, Multipart, State},
    http::StatusCode,
};
use std::sync::Arc;

use hex::encode;
use rand::{Rng, distributions::Alphanumeric};
use serde::Serialize;
use sha2::{Digest, Sha256};

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{
    exif::{extract_raw_metadata, parse_metadata},
    models::{ErrorResponse, MediaItem},
    utils::parse_filename,
};
use crate::{
    models::{UserBroadcast, WsEvent},
    utils::{generate_file_path, internal_error},
};

use super::AppState;

#[derive(Serialize, Debug)]
pub struct UploadResponse {
    pub file: String,
    pub file_url: String,
    pub file_size: usize,
}

pub async fn route(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>), (StatusCode, Json<ErrorResponse>)> {
    let config = sqlx::query!(
        "SELECT max_upload_size_mb, enforce_file_extensions, file_name_length, maintenance_mode FROM settings WHERE id = 1"
    )
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

    let max_bytes = config.max_upload_size_mb.map(|mb| mb * 1024 * 1024);

    if let Ok(Some(mut field)) = multipart.next_field().await {
        let org_file_name = field.file_name().unwrap_or("unknown").to_string();
        let file_ext = parse_filename(&org_file_name).1;
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let temp_filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let temp_filepath = format!("{}/tmp_{}", state.config.file_save_path, temp_filename);

        let mut file = File::create(&temp_filepath).await.map_err(internal_error)?;
        let mut hash = Sha256::new();
        let mut total_bytes = 0i64;

        while let Some(chunk) = field.chunk().await.map_err(internal_error)? {
            total_bytes += chunk.len() as i64;

            if let Some(limit) = max_bytes {
                if total_bytes > limit {
                    drop(file);
                    let _ = fs::remove_file(&temp_filepath).await;

                    return Err((
                        StatusCode::PAYLOAD_TOO_LARGE,
                        Json(ErrorResponse {
                            error: "File exceeds maximum allowed size".to_string(),
                        }),
                    ));
                }
            }

            hash.update(&chunk);
            file.write_all(&chunk).await.map_err(internal_error)?;
        }

        drop(file);
        let file_hash = encode(&hash.finalize()[..]);

        let raw_meta = extract_raw_metadata(temp_filepath.clone()).await;
        let parsed_meta = parse_metadata(&raw_meta);

        let lat = parsed_meta.gps.as_ref().map(|g| g.lat);
        let lng = parsed_meta.gps.as_ref().map(|g| g.lng);
        let date_taken = parsed_meta.date_taken.clone();

        let metadata_json =
            serde_json::to_string(&parsed_meta).unwrap_or_else(|_| "{}".to_string());

        let file_exists =
            sqlx::query!("SELECT file_path FROM media WHERE file_hash = ?", file_hash)
                .fetch_optional(&state.db)
                .await
                .map_err(internal_error)?;

        let (final_file_name, final_fp) = generate_file_path(
            config.file_name_length as usize,
            &state.config.file_save_path,
            &file_hash,
            &file_ext,
        );

        let final_path_used = if let Some(record) = file_exists {
            let _ = fs::remove_file(&temp_filepath).await;
            record.file_path
        } else {
            fs::rename(&temp_filepath, &final_fp)
                .await
                .map_err(internal_error)?;
            final_fp
        };

        sqlx::query!(
            r#"
            INSERT INTO media (
                uploaded_at, media_id, file_path, user_id, 
                content_type, file_hash, file_size, file_ext, original_file_name,
                latitude, longitude, date_taken, metadata
            ) 
            VALUES (unixepoch(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            final_file_name,
            final_path_used,
            user_id,
            content_type,
            file_hash,
            total_bytes,
            file_ext,
            org_file_name,
            lat,
            lng,
            date_taken,
            metadata_json
        )
        .execute(&state.db)
        .await
        .map_err(internal_error)?;

        let mut file_url = format!("/view/{}", &final_file_name);
        if config.enforce_file_extensions {
            if let Some(ext) = file_ext {
                file_url = format!("{}.{}", file_url, ext);
            }
        }

        let _ = state.tx.send(UserBroadcast {
            user_id,
            event: WsEvent::FileUpload(MediaItem {
                media_id: final_file_name.clone(),
                content_type,
                file_size: total_bytes,
                file_url: file_url.clone(),
                original_file_name: Some(org_file_name.clone()),
                uploaded_at: 1,
            }),
        });

        return Ok((
            StatusCode::OK,
            Json(UploadResponse {
                file: final_file_name,
                file_size: total_bytes as usize,
                file_url,
            }),
        ));
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "No file field found in request".to_owned(),
        }),
    ))
}

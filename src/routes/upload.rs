use axum::{
    Json,
    extract::{Extension, Multipart, State, multipart::Field},
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
    utils::{TempFileGuard, parse_filename},
};
use crate::{
    models::{UserBroadcast, WsEvent},
    utils::{generate_file_path, internal_error},
};

use super::AppState;

const MAX_FILES_PER_REQUEST: usize = 10;

#[derive(Serialize, Debug)]
pub struct UploadResponse {
    pub file: String,
    pub file_url: String,
    pub file_size: usize,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Upload {
    Success(UploadResponse),
    Failure {
        original_file_name: String,
        error: String,
    },
}

pub async fn route(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Vec<Upload>>), (StatusCode, Json<ErrorResponse>)> {
    let config = sqlx::query!(
        r#"
            SELECT 
                max_upload_size_mb, enforce_file_extensions, 
                file_name_length, maintenance_mode 
            FROM settings 
                WHERE id = 1
        "#
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
    let file_name_length = config.file_name_length;
    let enforce_file_extensions = config.enforce_file_extensions;

    let mut results: Vec<Upload> = Vec::new();

    loop {
        if results.len() >= MAX_FILES_PER_REQUEST {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Too many files in one request (max {MAX_FILES_PER_REQUEST})"),
                }),
            ));
        }

        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => return Err(internal_error(e)),
        };

        if field.file_name().is_none() {
            continue;
        }

        let org_file_name = field.file_name().unwrap_or("unknown").to_string();

        let outcome = match process_field(
            &state,
            max_bytes,
            file_name_length,
            enforce_file_extensions,
            field,
            org_file_name.clone(),
            user_id,
        )
        .await
        {
            Ok(response) => Upload::Success(response),
            Err(error) => Upload::Failure {
                original_file_name: org_file_name,
                error,
            },
        };

        results.push(outcome);
    }

    return Ok((StatusCode::OK, Json(results)));
}

async fn process_field(
    state: &Arc<AppState>,
    max_bytes: Option<i64>,
    file_name_length: i64,
    enforce_file_extensions: bool,
    mut field: Field<'_>,
    org_file_name: String,
    user_id: i64,
) -> Result<UploadResponse, String> {
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

    let temp_fp = format!("{}/tmp_{}", state.config.file_save_path, temp_filename);

    let mut file = File::create(&temp_fp)
        .await
        .map_err(|e| format!("failed to create temp file: {e}"))?;

    let mut guard = TempFileGuard::new(temp_fp.clone());

    let mut hash = Sha256::new();
    let mut total_bytes = 0i64;

    loop {
        let chunk = match field.chunk().await {
            Ok(Some(c)) => c,
            Ok(None) => break,
            Err(e) => return Err(format!("error reading upload stream: {e}")),
        };

        total_bytes += chunk.len() as i64;

        if let Some(limit) = max_bytes {
            if total_bytes > limit {
                return Err("File exceeds maximum allowed size".to_string());
            }
        }

        hash.update(&chunk);
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("failed to write file: {e}"))?;
    }

    drop(file);
    let file_hash = encode(&hash.finalize()[..]);

    let raw_meta = extract_raw_metadata(temp_fp.clone()).await;
    let parsed_meta = parse_metadata(&raw_meta);

    let lat = parsed_meta.gps.as_ref().map(|g| g.lat);
    let lng = parsed_meta.gps.as_ref().map(|g| g.lng);
    let date_taken = parsed_meta.date_taken.clone();

    let metadata_json = serde_json::to_string(&parsed_meta).unwrap_or_else(|_| "{}".to_string());

    let file_exists = sqlx::query!("SELECT file_path FROM media WHERE file_hash = ?", file_hash)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| format!("database error: {e}"))?;

    let (final_file_name, final_fp) = generate_file_path(
        file_name_length as usize,
        &state.config.file_save_path,
        &file_hash,
        &file_ext,
    );

    let (final_path_used, is_dedup) = if let Some(record) = file_exists {
        drop(guard);
        (record.file_path, true)
    } else {
        fs::rename(&temp_fp, &final_fp)
            .await
            .map_err(|e| format!("failed to move file into place: {e}"))?;

        guard.disarm();

        (final_fp, false)
    };

    let res = sqlx::query!(
        r#"
            INSERT INTO media (
                uploaded_at, media_id, file_path, user_id, content_type, 
                file_hash, file_size, file_ext, original_file_name,
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
    .await;

    if let Err(e) = res {
        if !is_dedup {
            let _ = fs::remove_file(&final_path_used).await;
        }

        return Err(format!("database error: {e}"));
    }

    let mut file_url = format!("/view/{}", &final_file_name);
    if enforce_file_extensions {
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

    return Ok(UploadResponse {
        file: final_file_name,
        file_size: total_bytes as usize,
        file_url,
    });
}

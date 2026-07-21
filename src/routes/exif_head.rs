use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
};
use std::sync::Arc;

use super::AppState;
use crate::{
    exif::ParsedMetadata,
    models::ErrorResponse,
    utils::{internal_error, parse_filename},
};

pub async fn route(
    State(state): State<Arc<AppState>>,
    Path(media_id): Path<String>,
) -> Result<(StatusCode, HeaderMap), (StatusCode, Json<ErrorResponse>)> {
    let mut media_id = media_id;

    if state.config.enforce_file_extensions {
        if let (Some(parsed_media_id), _) = parse_filename(&media_id) {
            media_id = parsed_media_id.to_string();
        }
    }

    let record = sqlx::query!(
        r#"
        SELECT 
            file_path, content_type, file_size, original_file_name,
            latitude, longitude, date_taken, metadata
        FROM media 
        WHERE media_id = ?
        "#,
        media_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    let record = match record {
        Some(r) => r,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "File not found.".to_owned(),
                }),
            ));
        }
    };

    let mut headers = HeaderMap::new();
    if let Ok(val) = HeaderValue::from_str(&record.content_type) {
        headers.insert(axum::http::header::CONTENT_TYPE, val);
    }

    if let Ok(val) = HeaderValue::from_str(&record.file_size.to_string()) {
        headers.insert(axum::http::header::CONTENT_LENGTH, val);
    }

    let mut add_header = |key: &'static str, val: Option<String>| {
        if let Some(v) = val {
            let safe_val = v.replace('\n', " ").replace('\r', "");
            if let Ok(header_val) = HeaderValue::from_str(&safe_val) {
                headers.insert(HeaderName::from_static(key), header_val);
            }
        }
    };

    add_header(
        axum::http::header::CONTENT_DISPOSITION.as_str(),
        record
            .original_file_name
            .map(|s| format!("inline; filename=\"{}\"", s)),
    );

    add_header("x-exif-datetaken", record.date_taken);
    add_header("x-exif-gpslatitude", record.latitude.map(|s| s.to_string()));
    add_header(
        "x-exif-gpslongitude",
        record.longitude.map(|s| s.to_string()),
    );

    let metadata_str = record.metadata.unwrap_or_else(|| "{}".to_string());
    if let Ok(meta) = serde_json::from_str::<ParsedMetadata>(&metadata_str) {
        add_header("x-exif-camera", meta.camera);
        add_header("x-exif-resolution", meta.resolution);
        add_header("x-exif-aperture", meta.aperture);
        add_header("x-exif-shutterspeed", meta.shutter_speed);
        add_header("x-exif-iso", meta.iso);
        add_header("x-exif-focallength", meta.focal_length);
        add_header("x-exif-flash", meta.flash);
        add_header("x-exif-whitebalance", meta.white_balance);

        add_header("x-audio-title", meta.title);
        add_header("x-audio-artist", meta.artist);
        add_header("x-audio-album", meta.album);
    }

    Ok((StatusCode::OK, headers))
}

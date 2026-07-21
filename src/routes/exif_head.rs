use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
};
use lofty::{file::TaggedFileExt, probe::Probe, tag::Accessor};
use std::sync::Arc;

use super::AppState;
use crate::{
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

    let record = sqlx::query!("SELECT file_path FROM media WHERE media_id = ?", media_id)
        .fetch_optional(&state.db)
        .await
        .map_err(internal_error)?;

    let file_path = match record {
        Some(r) => r.file_path,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "File not found.".to_owned(),
                }),
            ));
        }
    };

    let headers = tokio::task::spawn_blocking(move || {
        let mut map = HeaderMap::new();

        let file = match std::fs::File::open(&file_path) {
            Ok(f) => f,
            Err(_) => return map,
        };

        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();

        if let Ok(exif_data) = exifreader.read_from_container(&mut bufreader) {
            let mut found_exif = false;
            for field in exif_data.fields() {
                found_exif = true;
                let header_key = format!("x-exif-{}", field.tag).to_lowercase();
                if let Ok(header_name) = HeaderName::from_bytes(header_key.as_bytes()) {
                    let val_str = field.display_value().with_unit(&exif_data).to_string();
                    let safe_val = val_str.replace('\n', " ").replace('\r', "");
                    if let Ok(header_value) = HeaderValue::from_str(&safe_val) {
                        map.insert(header_name, header_value);
                    }
                }
            }

            if found_exif {
                return map;
            }
        }

        if let Ok(tagged_file) = Probe::open(&file_path).and_then(|probe| probe.read()) {
            if let Some(tag) = tagged_file.primary_tag() {
                let mut add_audio_header = |key: &str, value: Option<&str>| {
                    if let Some(val) = value {
                        let header_key = format!("x-audio-{}", key);
                        if let Ok(name) = HeaderName::from_bytes(header_key.as_bytes()) {
                            let safe_val = val.replace('\n', " ").replace('\r', "");
                            if let Ok(h_val) = HeaderValue::from_str(&safe_val) {
                                map.insert(name, h_val);
                            }
                        }
                    }
                };

                add_audio_header("artist", tag.artist().as_deref());
                add_audio_header("title", tag.title().as_deref());
                add_audio_header("album", tag.album().as_deref());
                add_audio_header("genre", tag.genre().as_deref());
            }
        }
        map
    })
    .await
    .map_err(internal_error)?;

    Ok((StatusCode::OK, headers))
}

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::AppState;
use crate::{
    models::{APIError, UploadResponse},
    utils::{generate_file_path, internal_error, parse_filename},
};

pub async fn route(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>),
            (StatusCode, Json<APIError>)> {

    while let Ok(Some(field)) = multipart.next_field().await {
        let org_file_name = field.file_name().unwrap_or_else(|| "unknown").to_string();

        let file_ext = parse_filename(&org_file_name).1;

        let content_type = field
            .content_type()
            .unwrap_or_else(|| &state.config.fallback_content_type)
            .to_string();

        let (file_name, fp) = generate_file_path(
            state.config.file_name_length,
            state.config.file_save_path,
            &file_ext,
        );

        let data = field.bytes().await.map_err(internal_error)?;

        let mut file = File::create(&fp).await.map_err(internal_error)?;
        file.write_all(&data).await.map_err(internal_error)?;

        sqlx::query!(
            "INSERT INTO media VALUES (unixepoch(), ?, ?, ?, ?, ?)",
            file_name,
            fp,
            content_type,
            file_ext,
            org_file_name
        )
        .execute(&*state.pool)
        .await
        .map_err(internal_error)?;

        // formats the url properly so I can return it with the response.
        let mut file_url = format!("{}{}", &state.config.endpoints.get_file, &file_name);
        if state.config.enforce_file_extensions {
            if let Some(ext) = file_ext {
                file_url += &format!(".{}", ext);
            }
        }

        return Ok((
            StatusCode::OK,
            Json(UploadResponse {
                file: file_name,
                file_size: data.len(),
                file_url,
            }),
        ));
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(APIError {
            message: "Bad Request".to_owned(),
        }),
    ))
}

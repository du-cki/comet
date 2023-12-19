use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    Json,
};
use hex::encode;
use sha2::{Digest, Sha256};

use tokio::{fs::File, io::AsyncWriteExt};

use super::AppState;
use crate::{
    models::{APIError, FileRecord as FileT, UploadQuery, UploadResponse},
    utils::{generate_file_path, internal_error, parse_filename},
};

pub async fn route(
    State(state): State<AppState>,
    query_param: Query<UploadQuery>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>), (StatusCode, Json<APIError>)> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let org_file_name = field.file_name().unwrap_or_else(|| "unknown").to_string();

        let file_ext = parse_filename(&org_file_name).1;

        let content_type = field
            .content_type()
            .unwrap_or_else(|| &state.config.fallback_content_type)
            .to_string();

        let data = field.bytes().await.map_err(internal_error)?;

        let mut hash = Sha256::new();
        hash.update(&data);
        let file_hash = encode(&hash.finalize()[..]);

        let (file_name, mut fp) = generate_file_path(
            state.config.file_name_length,
            state.config.file_save_path,
            &file_hash,
            &file_ext,
        );

        let is_public: u16 = query_param.public.unwrap_or(state.config.default_public) as u16;

        let file_exists = sqlx::query!(
            r#"
            SELECT file_path FROM media
                WHERE ? = file_hash
        "#,
            file_hash
        )
        .fetch_optional(&*state.pool)
        .await
        .map_err(internal_error)?;

        if let Some(record) = file_exists {
            fp = record.file_path;
        } else {
            let mut file = File::create(&fp).await.map_err(internal_error)?;

            file.write_all(&data).await.map_err(internal_error)?;
        }

        let file = sqlx::query_as!(
            FileT,
            r#"
               INSERT INTO media (
                 uploaded_at, is_public, file_name,
                 file_path, content_type, file_hash,
                 file_ext, original_file_name
               ) VALUES (
                   unixepoch(), ?, ?,
                   ?, ?, ?,
                   ?, ?
               ) RETURNING
                 original_file_name, file_name, file_ext,
                 folder_id, file_id, uploaded_at AS last_updated
            "#,
            is_public,
            file_name,
            fp,
            content_type,
            file_hash,
            file_ext,
            org_file_name,
        )
        .fetch_one(&*state.pool)
        .await
        .map_err(internal_error)?;

        if state.sx.receiver_count() > 0 {
            tracing::info!("broadcasting: {:?}", file);

            if let Err(err) = state.sx.send(file) {
                tracing::error!("could not broadcast file upload, due to: {:#?}", err);
            };
        }

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

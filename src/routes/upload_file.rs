use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    Json,
};
use hex::encode;
use nanoid::nanoid;

use sha2::{Digest, Sha256};

use tokio::{fs::File, io::AsyncWriteExt};

use super::AppState;
use crate::{
    models::{APIError, FileRecord, UploadQuery, UploadResponse},
    utils::{internal_error, FileInfo},
};

pub async fn route(
    State(state): State<AppState>,
    query_param: Query<UploadQuery>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadResponse>), (StatusCode, Json<APIError>)> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let original_file_name = field.file_name().unwrap_or_else(|| "unknown").to_string();

        let file_ext = FileInfo::from_str(&original_file_name).ext;

        let content_type = field
            .content_type()
            .unwrap_or_else(|| &state.config.fallback_content_type)
            .to_string();

        let data = field.bytes().await.map_err(internal_error)?;

        let file_hash = {
            let mut hash = Sha256::new();
            hash.update(&data);

            encode(&hash.finalize()[..])
        };

        let len = state.config.file_name_length;

        let file_name = state
            .config
            .retain_uploaded_file_name
            .then_some(original_file_name.clone())
            .or(nanoid!(len).into());

        let is_public = query_param.public.unwrap_or(state.config.default_public) as u16;

        let fp = {
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
                record.file_path
            } else {
                let file_path = format!(
                    "{:}{:}{:}",
                    state.config.file_save_path,
                    file_hash,
                    file_ext.map_or("".to_string(), |e| format!(".{e:}"))
                );

                let mut file = File::create(&file_path).await.map_err(internal_error)?;
                file.write_all(&data).await.map_err(internal_error)?;

                file_path
            }
        };

        let file = sqlx::query_as!(
            FileRecord,
            r#"
               INSERT INTO media (
                 uploaded_at, is_public, file_name,
                 file_path, content_type, file_hash,
                 file_ext, original_file_name, folder_id
               ) VALUES (
                   unixepoch(), $1, $2,
                   $3, $4, $5,
                   $6, $7, (
                        SELECT EXISTS(
                            SELECT id FROM folder_paths WHERE path = $8
                        )
                    )
               ) RETURNING
                 original_file_name, file_id, file_ext,
                 folder_id, file_name, uploaded_at AS last_updated,
                 (
                    SELECT path FROM folder_paths WHERE id = media.folder_id
                 ) || '/' || media.file_name || COALESCE("." || media.file_ext, "") AS "file_url!"
            "#,
            is_public,
            file_name,
            fp,
            content_type,
            file_hash,
            file_ext,
            original_file_name,
            query_param.folder
        )
        .fetch_one(&*state.pool)
        .await
        .map_err(|_| {
            // TODO: fs::remove_file(fp).await else;

            // should be the only error that could possible be raised by this query
            // not sure how I can filter the error and get the proper error anyway.
            (
                StatusCode::NOT_FOUND,
                Json(APIError {
                    message: format!("folder `{}` not found", query_param.folder.clone().unwrap()),
                }),
            )
        })?;

        if state.sx.receiver_count() > 0 {
            if let Err(err) = state.sx.send(file.clone()) {
                tracing::error!("could not broadcast file upload, due to: {:#?}", err);
            };
        }

        return Ok((
            StatusCode::OK,
            Json(UploadResponse {
                file: file.file_name.unwrap(),
                file_size: data.len(),
                file_url: file.file_url,
                is_public,
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

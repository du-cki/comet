use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        ConnectInfo, State
    },
    response::IntoResponse
};
use tokio::sync::broadcast::Receiver;
use std::net::SocketAddr;

use crate::{models::{AppState, File, Folder, GenericRequest, FileRecord, Events}, json_message};

struct FileUpdates<'a> {
    sock: &'a mut WebSocket,
    rx: Receiver<FileRecord>,
    state: AppState
}

impl<'a> FileUpdates<'a> {
    fn from_data(
        sock: &'a mut WebSocket,
        rx: Receiver<FileRecord>,
        state: AppState
    ) -> Self {
        Self { sock, rx, state }
    }

    async fn file_upload_events(&mut self) {
        while let Ok(file) = self.rx.recv().await {
            let _ = self.sock.send(json_message!(
                "event" => Events::FileUpload.to_string(),
                "file" => file
            )).await;
        }
    }

    async fn respond_to_messages(&mut self) {
        while let Some(message) = self.sock.recv().await {
            if let Ok(msg) = message {
                if let Ok(data) = serde_json::from_str::<GenericRequest>(
                    msg.to_text().unwrap()
                ) {
                    use crate::models::Events as E;

                    let msg = match data.event {
                        E::QueryFolder => query_folder(
                            &self.state,
                            data
                        ).await,
                        other => { tracing::error!("unimplemented event `{:?}` received, ignoring...", other); continue; },
                    };

                    let _ = self.sock.send(msg).await;
                };
            }
        };
    }

    async fn start(&mut self) {
        // self.file_upload_events().await;
        self.respond_to_messages().await;
    }
}

async fn query_folder(
    state: &AppState,
    request: GenericRequest,
) -> Message {
    tracing::info!("Querying directory: {:#?}", request.data);

    let folder_query = sqlx::query_as!(Folder, r#"
        WITH RECURSIVE tmp(id, path) AS (
           SELECT NULL, '/'
           UNION ALL

           SELECT folder_id, '/' || folder_name
           FROM folders
           WHERE parent_folder_id IS NULL

           UNION ALL

           SELECT
               folder_id,
               tmp.path || '/' || folders.folder_name AS name
           FROM folders
               JOIN tmp ON folders.parent_folder_id = tmp.id
        ) SELECT * FROM tmp WHERE tmp.path = $1;
    "#,
        request.data
    )
        .fetch_optional(&*state.pool).await.unwrap();

    if let Some(folder) = folder_query {
        let file_query = sqlx::query_as!(File, r#"
            SELECT file_name, file_ext, file_id,
                COALESCE(last_updated_at, uploaded_at) AS last_updated
            FROM media
                WHERE is_public = 1
                    AND folder_id IS $1;
        "#,
            folder.id
        )
            .fetch_all(&*state.pool)
            .await;

        if let Ok(files) = file_query {
            return json_message!(
                "event" => Events::QueryFolder.to_string(),
                "files" => files,
                "request_id" => request.request_id
            )
        }
    }

    json_message!(
        "event" => Events::QueryFolder.to_string(),
        "files" => Vec::<File>::new(),
        "request_id" => request.request_id
    )
}


pub async fn route(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| pineapple(
        socket,
        state,
        addr,
    ))
}


async fn pineapple(
    mut socket: WebSocket,
    state: AppState,
    _: SocketAddr,
) {
    FileUpdates::from_data(
        &mut socket,
        state.sx.subscribe(),
        state
    ).start().await
}


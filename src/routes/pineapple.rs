use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::IntoResponse,
};

use std::net::SocketAddr;
use tokio::sync::broadcast::Receiver;

use crate::{
    json_message,
    models::{AppState, Events, File, FileRecord, Folder, GenericRequest},
};

struct WSHandler {
    socket: WebSocket,
    rx: Receiver<FileRecord>,
    state: AppState,
}

unsafe impl std::marker::Send for WSHandler {}
unsafe impl std::marker::Sync for WSHandler {}

impl WSHandler {
    async fn from(socket: WebSocket, rx: Receiver<FileRecord>, state: AppState) {
        Self { socket, rx, state }.handle().await;
    }

    async fn handle(&mut self) {
        tracing::info!("Received a new connection, starting loop...");

        loop {
            tokio::select! {
                Some(Ok(message)) = self.socket.recv() => {
                    tracing::info!("received message: {:?}", message);

                    match message {
                        Message::Text(msg) => {
                            if let Some(response) = self.poll_response(msg).await {
                                let _ = self.socket
                                    .send(response)
                                    .await;
                            }
                        }
                        Message::Close(_) => {
                            tracing::info!("client disconnected");
                            return;
                        }
                        _ => {}
                    }
                }
                Ok(file) = self.rx.recv() => {
                    tracing::info!("received file: {:?}", file);

                    let _ = self.socket
                        .send(json_message!(
                            "event" => Events::FileUpload.to_string(),
                            "file" => file
                        ))
                        .await;
                }
            }
        }
    }

    async fn poll_response(&self, msg: String) -> Option<Message> {
        if let Ok(data) = serde_json::from_str::<GenericRequest>(&msg) {
            let message: Message = match data.event {
                Events::QueryFolder => self.query_folder(data).await,
                other => {
                    tracing::error!("unimplemented event `{:?}` received, ignoring...", other);

                    return None;
                }
            };

            return Some(message);
        }

        None
    }

    async fn query_folder(&self, request: GenericRequest) -> Message {
        let folder_query = sqlx::query_as!(
            Folder,
            r#"
            SELECT
                id, path as "path!: String"
            FROM folder_paths
                WHERE path = $1;
        "#,
            request.data
        )
        .fetch_optional(&*self.state.pool)
        .await
        .unwrap();

        if let Some(folder) = folder_query {
            let file_query = sqlx::query_as!(
                File,
                r#"
                SELECT file_id as "id!",
                    file_name || COALESCE('.' || file_ext, '') as "name!",
                    1 AS "file_type!",
                    COALESCE(last_updated_at, uploaded_at) AS "last_updated!"
                FROM media
                    WHERE is_public = 1
                        AND folder_id is $1

                UNION

                SELECT folder_id AS "id!",
                    folder_name AS "name!",
                     2 AS "file_type!",
                    (SELECT MAX(COALESCE(last_updated_at, uploaded_at))
                        FROM media) AS "last_updated!"
                FROM folders
                    WHERE is_public = 1
                        AND parent_folder_id IS $1;
            "#,
                folder.id
            )
            .fetch_all(&*self.state.pool)
            .await;

            if let Ok(files) = file_query {
                return json_message!(
                    "event" => Events::QueryFolder,
                    "files" => files,
                    "request_id" => request.request_id
                );
            }
        }

        json_message!(
            "event" => Events::QueryFolder,
            "files" => Vec::<File>::new(),
            "request_id" => request.request_id
        )
    }
}

pub async fn route(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| pineapple(socket, state, addr))
}

async fn pineapple(
    socket: WebSocket,
    state: AppState,
    _: SocketAddr, // TODO
) {
    WSHandler::from(socket, state.sx.subscribe(), state).await
}

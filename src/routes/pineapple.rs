use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    response::IntoResponse,
};
use futures::StreamExt;
use tokio::{
    sync::{broadcast::Receiver, Mutex},
    time::{self, Duration},
};

use std::{net::SocketAddr, sync::Arc};

use crate::{
    json_message,
    models::{AppState, Events, File, FileRecord, Folder, GenericRequest},
};

struct WSHandler {
    socket: WebSocket,
    rx: Receiver<FileRecord>,
    state: AppState,
}

impl WSHandler {
    async fn from(socket: WebSocket, rx: Receiver<FileRecord>, state: AppState) {
        Self { socket, rx, state }.handle().await;
    }

    async fn handle(&mut self) {
        loop {
            if let Ok(file) = self.rx.recv().await {
                let _ = self
                    .socket
                    .send(json_message!(
                        "event" => Events::FileUpload.to_string(),
                        "file" => file
                    ))
                    .await;
            }

            let message = self.socket.next().await;

            if let Some(Ok(message)) = message {
                if let Ok(msg) = message.to_text() {
                    if let Ok(data) = serde_json::from_str::<GenericRequest>(msg) {
                        let msg: Message = match data.event {
                            Events::QueryFolder => self.query_folder(data).await,
                            other => {
                                tracing::error!(
                                    "unimplemented event `{:?}` received, ignoring...",
                                    other
                                );
                                continue;
                            }
                        };

                        let _ = self.socket.send(msg).await;
                    }
                }
            } else if let None = message {
                tracing::info!("client disconnected, exiting loop...");
                break;
            }

            time::sleep(Duration::from_secs(1)).await;
            tracing::info!("Done iteration, sleeping...")
        }
    }

    // async fn respond_to_messages(&self) {
    //     while let Some(Ok(message)) = self.receiver.lock().await.next().await {
    //         if let Ok(msg) = message.to_text() {
    //             if let Ok(data) = serde_json::from_str::<GenericRequest>(msg) {
    //                 let msg: Message = match data.event {
    //                     Events::QueryFolder => self.query_folder(data).await,
    //                     other => {
    //                         tracing::error!(
    //                             "unimplemented event `{:?}` received, ignoring...",
    //                             other
    //                         );
    //                         continue;
    //                     }
    //                 };

    //                 let mut sender = self.sender.lock().await;
    //                 let _ = sender.send(msg).await;
    //             }
    //         }
    //     }
    // }

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

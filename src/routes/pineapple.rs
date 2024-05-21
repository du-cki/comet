use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        ConnectInfo, State
    },
    response::IntoResponse
};
use futures::{StreamExt, stream::{SplitStream, SplitSink}, SinkExt};
use tokio::sync::{broadcast::Receiver, Mutex};

use std::{net::SocketAddr, sync::Arc};

use crate::{
    models::{AppState, File, Folder, GenericRequest, FileRecord, Events},
    json_message
};

struct FileUpdates {
    receiver: Arc<Mutex<SplitStream<WebSocket>>>,
    sender:   Arc<Mutex<SplitSink<WebSocket, Message>>>,
    rx:       Arc<Mutex<Receiver<FileRecord>>>,
    state:    AppState
}

impl FileUpdates {
    async fn from(
        sock: WebSocket, rx: Receiver<FileRecord>, state: AppState
    ) {
        let (sender, receiver) = sock.split();

        let s = Self {
            receiver: Arc::new(Mutex::new(receiver)),
            sender:   Arc::new(Mutex::new(sender)),
            rx:       Arc::new(Mutex::new(rx)),
            state
        };

        s.respond_to_messages().await;
        // tokio::join!(
        //     tokio::spawn(s.file_upload_events())
        // );
    }

    async fn file_upload_events(&self) {
        while let Ok(file) = self.rx.lock().await.recv().await {
            let mut writer = self.sender.lock().await;

            let _ = writer.send(json_message!(
                "event" => Events::FileUpload.to_string(),
                "file" => file
            )).await;
        }
    }

    async fn respond_to_messages(&self) {
        while let Some(Ok(message)) = self.receiver.lock().await.next().await {
            if let Ok(msg) = message.to_text() {
                if let Ok(data) = serde_json::from_str::<GenericRequest>(msg) {
                    let msg: Message = match data.event {
                        Events::QueryFolder => self.query_folder(data).await,
                        other => {
                            tracing::error!("unimplemented event `{:?}` received, ignoring...", other);
                            continue;
                        },
                    };

                    let mut sender = self.sender.lock().await;
                    let _ = sender.send(msg).await;
                }
            }
        }
    }

    async fn query_folder(
        &self,
        request: GenericRequest,
    ) -> Message {
        let folder_query = sqlx::query_as!(Folder, r#"
            SELECT
                id, path as "path!: String"
            FROM folder_paths
                WHERE path = $1;
        "#,
            request.data
        )
            .fetch_optional(&*self.state.pool).await.unwrap();

        if let Some(folder) = folder_query {
            let file_query = sqlx::query_as!(File, r#"
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
                )
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
    ws.on_upgrade(move |socket| pineapple(
        socket,
        state,
        addr,
    ))
}


async fn pineapple(
    socket: WebSocket,
    state: AppState,
    _: SocketAddr, // TODO
) {
    FileUpdates::from(
        socket,
        state.sx.subscribe(),
        state
    ).await
}


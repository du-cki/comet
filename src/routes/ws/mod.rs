use std::sync::Arc;

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use dispatch::dispatch;
use futures_util::SinkExt;
use serde::Deserialize;
use tokio::sync::broadcast;

use crate::{
    jwt::validate_jwt,
    models::{AppState, WSClientCommand, WsEvent},
};

mod dispatch;
mod handlers;

#[derive(Deserialize)]
struct WsAuthPayload {
    token: String,
}

pub async fn route(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn send_ws_event(socket: &mut WebSocket, event: WsEvent) -> Result<(), ()> {
    let json = serde_json::to_string(&event).map_err(|_| ())?;
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|_| ())
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let user_id = match authenticate_socket(&mut socket, &state).await {
        Ok(id) => id,
        Err(_) => {
            let _ = socket.close().await;
            return;
        }
    };

    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            broadcast_result = rx.recv() => {
                let broadcast_msg = match broadcast_result {
                    Ok(m) => m,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                };

                if broadcast_msg.user_id == user_id
                    && send_ws_event(&mut socket, broadcast_msg.event).await.is_err()
                {
                    break;
                }
            }
            msg = socket.recv() => {
                let msg = match msg {
                    Some(Ok(m)) => m,
                    Some(Err(_)) | None => break,
                };

                let Message::Text(text) = msg else { continue };

                let Ok(cmd) = serde_json::from_str::<WSClientCommand>(&text) else {
                    let _ = send_ws_event(&mut socket, WsEvent::Error("unrecognized command".into())).await;
                    continue;
                };

                let event = dispatch(cmd, user_id, &state).await;
                if send_ws_event(&mut socket, event).await.is_err() {
                    break;
                }
            }
        }
    }
}

async fn authenticate_socket(socket: &mut WebSocket, state: &Arc<AppState>) -> Result<i64, ()> {
    let msg = match socket.recv().await {
        Some(Ok(msg)) => msg,
        _ => return Err(()),
    };

    let text = match msg {
        Message::Text(text) => text,
        _ => return Err(()),
    };

    let payload: WsAuthPayload = serde_json::from_str(&text).map_err(|_| ())?;
    let token_data = validate_jwt(&payload.token, &state.jwt_secret).map_err(|_| ())?;

    Ok(token_data.claims.sub)
}

use super::handlers::{stats, uploads};
use crate::models::{AppState, WSClientCommand, WsEvent};
use std::sync::Arc;

pub async fn dispatch(cmd: WSClientCommand, user_id: i64, state: &Arc<AppState>) -> WsEvent {
    match cmd {
        WSClientCommand::GetStats => stats::get_stats(user_id, state).await,
        WSClientCommand::GetUploads { cursor, limit } => {
            uploads::get_uploads(user_id, cursor, limit, state).await
        }
    }
}

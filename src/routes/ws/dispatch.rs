use super::handlers::{stats, uploads};
use crate::models::{AppState, DbUser, WSClientCommand, WsEvent};
use std::sync::Arc;

pub async fn dispatch(cmd: WSClientCommand, user: &DbUser, state: &Arc<AppState>) -> WsEvent {
    match cmd {
        WSClientCommand::GetStats => stats::get_stats(user, state).await,
        WSClientCommand::GetAdminStats => stats::get_stats_admin(user, state).await,
        WSClientCommand::GetUploads { cursor, limit } => {
            uploads::get_uploads(user, cursor, limit, state).await
        }
    }
}

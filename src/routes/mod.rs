use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{delete as del, get, head, patch, post},
};

use crate::{jwt::validate_jwt, models::AppState};

mod config;
mod delete;
mod does_any_user_exist;
mod exif_head;
mod login;
mod profile;
mod register;
mod settings;
mod thumbnail;
mod upload;
mod view;
mod ws;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let user_id: i64;

    if auth_header.starts_with("Bearer ") {
        let token = &auth_header[7..];
        let token_data =
            validate_jwt(token, &state.jwt_secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

        user_id = token_data.claims.sub;
    } else {
        let record = sqlx::query!("SELECT id FROM users WHERE api_key = ?", auth_header)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match record {
            Some(user) => {
                user_id = user.id;
            }
            None => return Err(StatusCode::UNAUTHORIZED),
        }
    }

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}

pub fn with_state(state: Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/ping", get(|| async {}))
        .route("/config", get(config::route))
        .route("/does-any-user-exist", get(does_any_user_exist::route))
        .route("/login", post(login::route))
        .route("/pineapple", get(ws::route))
        .route("/register", post(register::route))
        .route("/view/{media_id}", head(exif_head::route))
        .route("/thumb/{media_id}", get(thumbnail::route))
        .route("/view/{media_id}", get(view::route));

    let auth = Router::new()
        .route("/upload", post(upload::route))
        .route("/profile/reset-password", post(profile::reset_password))
        .route("/profile/reset-key", post(profile::reset_api_key))
        .route("/view/{media_id}", del(delete::route))
        .route("/settings", get(settings::get_settings))
        .route("/settings", patch(settings::patch_settings))
        .layer(DefaultBodyLimit::disable())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new().merge(app).merge(auth).with_state(state)
}

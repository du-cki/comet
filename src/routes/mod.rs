use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{delete as del, get, head, post},
};

use crate::{jwt::validate_jwt, models::AppState};

mod delete;
mod does_any_user_exist;
mod exif_head;
mod login;
mod register;
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

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    let token_data =
        validate_jwt(token, &state.jwt_secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(token_data.claims.sub);

    Ok(next.run(req).await)
}

pub fn with_state(state: Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/ping", get(|| async {}))
        .route("/does-any-user-exist", get(does_any_user_exist::route))
        .route("/login", post(login::route))
        .route("/pineapple", get(ws::route))
        .route("/register", post(register::route))
        .route("/view/{media_id}", head(exif_head::route))
        .route("/view/{media_id}", get(view::route));

    let auth = Router::new()
        .route("/upload", post(upload::route))
        .route("/view/{media_id}", del(delete::route))
        .layer(DefaultBodyLimit::disable())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new().merge(app).merge(auth).with_state(state)
}

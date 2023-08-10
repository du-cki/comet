use axum::response::Json;

use crate::models::GenericResponse;

pub async fn route() -> Json<GenericResponse> {
    Json(GenericResponse {
        message: "pong!".to_owned(),
    })
}

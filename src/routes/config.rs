use axum::{
    Json,
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ToolQuery {
    pub tool: String,
    pub api_key: String,
    pub domain: String,
}

pub async fn route(Query(query): Query<ToolQuery>) -> Result<impl IntoResponse, StatusCode> {
    match query.tool.as_str() {
        "sharex" => Ok(sharex(query.api_key, query.domain)),
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

fn sharex(api_key: String, domain: String) -> impl IntoResponse {
    let sxcu_payload = json!({
        "Version": "15.0.0",
        "Name": "Comet",
        "DestinationType": "ImageUploader, TextUploader, FileUploader",
        "RequestMethod": "POST",
        "RequestURL": format!("{domain}/upload"),
        "Headers": {
            "Authorization": api_key
        },
        "Body": "MultipartFormData",
        "FileFormName": "file",
        "URL": format!("{domain}{{json:$.[0].file_url}}"),
        "ErrorMessage": "{json:$.[0].error}"
    });

    let headers = [
        (header::CONTENT_TYPE, "application/json; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"comet.sxcu\"",
        ),
    ];

    (headers, Json(sxcu_payload))
}

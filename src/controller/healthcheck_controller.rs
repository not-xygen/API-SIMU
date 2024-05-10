use axum::{response::IntoResponse, Json};

use crate::service::healthcheck_service::healthcheck_service;

pub async fn healthcheck_controller() -> impl IntoResponse {
    let message = healthcheck_service().await;

    let json_response = serde_json::json!({
        "status": "ok",
        "message": message
    });

    Json(json_response)
}
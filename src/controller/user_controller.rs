use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    model::schema::{CreateUpdateUserSchema, FilterOptions},
    service::user_service::{
        create_user_service, delete_user_by_id_service, get_all_user_service,
        get_user_by_id_service, update_user_service,
    },
    AppState,
};

pub async fn get_all_user_controller(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let Query(_opts) = opts.unwrap_or_default();
    let res = get_all_user_service(State(data)).await;

    match res {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "ok",
                "message": "Users fetched successfully",
                "data": res.ok()
            });

            Json(json_response)
        }
        Err(e) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": e
            });

            Json(json_response)
        }
    }
}

pub async fn get_user_by_id_controller(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let res = get_user_by_id_service(State(data), id).await;

    match res {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "ok",
                "message": "User fetched successfully",
                "data": res.ok()
            });

            Json(json_response)
        }
        Err(e) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": e
            });

            Json(json_response)
        }
    }
}

pub async fn create_user_controller(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUpdateUserSchema>,
) -> impl IntoResponse {
    let res = create_user_service(State(data), axum::Json(body)).await;

    match res {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "ok",
                "message": "User created successfully"
            });

            Json(json_response)
        }
        Err(e) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": e
            });

            Json(json_response)
        }
    }
}

pub async fn update_user_controller(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<CreateUpdateUserSchema>,
) -> impl IntoResponse {
    let res = update_user_service(State(data), id, axum::Json(body)).await;

    match res {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "ok",
                "message": "User updated successfully"
            });

            Json(json_response)
        }
        Err(e) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": e
            });

            Json(json_response)
        }
    }
}

pub async fn delete_user_by_id_controller(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {

    let res = delete_user_by_id_service(State(data), id).await;
    match res {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "ok",
                "message": "User Deleted successfully"
            });

            Json(json_response)
        }
        Err(e) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": e
            });

            Json(json_response)
        }
    }
}

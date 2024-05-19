use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    model::schema::{CreateUpdateUserSchema, FilterOptions},
    service::user::user_service::{
        create_user_service, delete_user_by_id_service, get_all_user_service,
        get_user_by_id_service, update_user_service,
    },
    utils::singleton::AppState,
};

pub async fn get_all_user_controller(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let Query(_opts) = opts.unwrap_or_default();
    
    let data = State(data);
    data.observable.notify_crud("GET", "All Users");

    let res = get_all_user_service(data).await;
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
    let data = State(data);
    data.observable.notify_crud("GET", "User");

    let res = get_user_by_id_service(data, id).await;
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
    let data = State(data);
    data.observable.notify_crud("POST", "User");

    let res = create_user_service(data, axum::Json(body)).await;
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
    let data = State(data);
    data.observable.notify_crud("PUT", "User");

    let res = update_user_service(data, id, axum::Json(body)).await;
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
    let data = State(data);
    data.observable.notify_crud("DEL", "User");

    let res = delete_user_by_id_service(data, id).await;
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

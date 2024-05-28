use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::{
    model::schema::{CreateUpdateUserSchema, FilterOptions},
    service::user::user_service::{
        create_user_service, delete_user_by_id_service, get_all_user_service,
        get_user_by_id_service, update_user_service,
    },
    utils::singleton::{init_app_state, AppState},
};

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub async fn get_all_user_controller(opts: Option<Query<FilterOptions>>) -> impl IntoResponse {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);

    let Query(_opts) = opts.unwrap_or_default();

    data.observable.notify_crud("GET", "All Users");

    let res = get_all_user_service().await;
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

pub async fn get_user_by_id_controller(Path(id): Path<i32>) -> impl IntoResponse {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    data.observable.notify_crud("GET", "User");

    let res = get_user_by_id_service(id).await;
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

pub async fn create_user_controller(Json(body): Json<CreateUpdateUserSchema>) -> impl IntoResponse {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    data.observable.notify_crud("POST", "User");

    let res = create_user_service(axum::Json(body)).await;
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
    Path(id): Path<i32>,
    Json(body): Json<CreateUpdateUserSchema>,
) -> impl IntoResponse {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    data.observable.notify_crud("PUT", "User");

    let res = update_user_service(id, axum::Json(body)).await;
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

pub async fn delete_user_by_id_controller(Path(id): Path<i32>) -> impl IntoResponse {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    let data: State<Arc<AppState>> = State(app_state);
    data.observable.notify_crud("DEL", "User");

    let res = delete_user_by_id_service(id).await;
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

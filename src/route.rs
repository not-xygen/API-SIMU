use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::OnceCell;

use crate::{
    controller::{
        healthcheck_controller::healthcheck_controller,
        user_controller::{
            create_user_controller, delete_user_by_id_controller, get_all_user_controller,
            get_user_by_id_controller, update_user_controller,
        },
    },
    utils::singleton::{init_app_state, AppState},
};

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

pub async fn create_router() -> Router {
    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();
    Router::new()
        .route("/api/healthcheck", get(healthcheck_controller))
        .route(
            "/api/user",
            get(get_all_user_controller).post(create_user_controller),
        )
        .route(
            "/api/user/:id",
            get(get_user_by_id_controller)
                .put(update_user_controller)
                .delete(delete_user_by_id_controller),
        )
        .with_state(app_state)
}

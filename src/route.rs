use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    controller::{
        healthcheck_controller::healthcheck_controller,
        user_controller::{
            create_user_controller, get_all_user_controller, get_user_by_id_controller,
            update_user_controller,delete_user_by_id_controller
        },
    },
    utils::singleton::AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthcheck", get(healthcheck_controller))
        .route(
            "/api/user",
            get(get_all_user_controller).post(create_user_controller),
        )
        .route(
            "/api/user/:id",
            get(get_user_by_id_controller).put(update_user_controller).delete(delete_user_by_id_controller),
        )
        .with_state(app_state)
}

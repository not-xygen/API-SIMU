mod controller;
mod model;
mod route;
mod service;
mod utils;

use crate::utils::singleton::{init_app_state, AppState};
use axum::http::{header::CONTENT_TYPE, Method};
use route::create_router;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tower_http::cors::{Any, CorsLayer};

static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    println!("🌟 REST API Service 🌟");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app_state: Arc<AppState> = APP_STATE.get_or_init(init_app_state).await.clone();

    let app = create_router(app_state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3245").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

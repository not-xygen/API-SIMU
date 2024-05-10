mod route;
mod model;
mod controller;
mod service;
mod utils;

use std::sync::Arc;
use sqlx::mysql::MySqlPool;
use tower_http::cors::{Any, CorsLayer};
use axum::http::{header::CONTENT_TYPE, Method};
use route::create_router;
use utils::singleton::get_database_pool;

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    println!("🌟 REST API Service 🌟");

    let pool = get_database_pool().await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5555").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
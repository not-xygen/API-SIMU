mod route;
mod model;
mod controller;
mod service;
mod utils;

use tower_http::cors::{Any, CorsLayer};
use axum::http::{header::CONTENT_TYPE, Method};
use crate::utils::singleton::get_app_state;
use route::create_router;

#[tokio::main]
async fn main() {
    println!("🌟 REST API Service 🌟");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app_state = get_app_state().await.unwrap();

    let app = create_router(app_state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3245").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
mod controller;
mod model;
mod route;
mod service;
mod utils;

use axum::http::{header::CONTENT_TYPE, Method};
use route::create_router;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    println!("🌟 REST API Service 🌟");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router().await.layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3245").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

mod config;
mod error;
mod state;
mod handlers;
mod models;
mod routes;
mod services;

use crate::handlers::health::health_check;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

mod config;
mod error;
mod state;
mod handlers;
mod models;
mod routes;
mod services;

use crate::handlers::health::health_check;
use sqlx::PgPool;

use axum::{
    routing::get,
    Router,
};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgres database pool");

    let state = AppState {
        pool
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

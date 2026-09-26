mod config;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

use crate::handlers::health::health_check;
use crate::routes::hiragana::hiragana_routes;
use crate::state::AppState;
use sqlx::PgPool;
use tower_http::services::ServeDir;

use crate::handlers::hiragana::get_practice_question_handler;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let static_file = ServeDir::new("static");

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgres database pool");

    let state = AppState { pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/hiragana", hiragana_routes())
        .route("/api/practice/question", get(get_practice_question_handler))
        .fallback_service(static_file)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

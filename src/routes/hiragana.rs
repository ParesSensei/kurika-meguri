use axum::{Router, routing::get};

use crate::{
    handlers::hiragana::{get_all_hiragana_handler, get_random_hiragana_handler},
    state::AppState,
};

pub fn hiragana_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_hiragana_handler))
        .route("/random", get(get_random_hiragana_handler))
}

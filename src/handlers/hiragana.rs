use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use tracing::{event, info, Level};
use crate::{
    models::hiragana::Hiragana,
    state::AppState,
};

pub async fn get_all_hiragana_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Hiragana>>, (StatusCode, String)> {
    info!("get all hiragana route called");
    let hiragana = sqlx::query_as::<_, Hiragana>(
        "SELECT id, character, romaji, row_group
         FROM hiragana
         ORDER BY id ASC"
    )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(hiragana))
}

pub async fn get_random_hiragana_handler(
    State(state): State<AppState>,
) -> Result<Json<Hiragana>, (StatusCode, String)> {
    event!(Level::INFO, "getting random hiragana route called");
    let ran_id = rand::random_range(1..=46);

    let hiragana = sqlx::query_as!(
        Hiragana,
        "SELECT id, character, romaji, row_group from hiragana WHERE id = $1",
        ran_id
    )
        .fetch_one(&state.pool)
        .await;

    let result = match hiragana {
        Ok(hiragana) => Ok(Json(hiragana)),
        Err(eror) => Err((StatusCode::INTERNAL_SERVER_ERROR, eror.to_string()))
    };
    result
}
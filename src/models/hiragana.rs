use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use rand::rng;
use serde::{Serialize};
use sqlx::FromRow;
use crate::state::AppState;

#[derive(Serialize, FromRow, Debug)]
pub struct Hiragana {
    pub id: i32,
    pub character: String,
    pub romaji: String,
    pub row_group: String,
}

#[axum::debug_handler]
pub async fn get_all_hiragana_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Hiragana>>, (StatusCode, String)> {
    let hiragana = sqlx::query_as::<_, Hiragana>(
        "SELECT id, character, romaji, row_group FROM hiragana ORDER BY id ASC"
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


// Contoh menggunakan PostgreSQL ($1) dan makro query_as!
// let target_id = 42;
//
// let user = sqlx::query_as!(
//     User,
//     "SELECT id, name FROM users WHERE id = $1",
//     target_id
// )
// .fetch_one(&pool)
// .await?;

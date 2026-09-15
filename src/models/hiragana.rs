use serde::{Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Hiragana {
    pub id: i32,
    pub character: String,
    pub romaji: String,
    pub row_group: String,
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

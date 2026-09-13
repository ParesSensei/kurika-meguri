use sqlx::PgPool;

pub struct AppState {
    pub state: PgPool,
}
use serde::Deserialize;
use sqlx::pool::Pool;
use sqlx::Postgres;

pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub content: String,
}

use sqlx::pool::Pool;
use sqlx::Postgres;

pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

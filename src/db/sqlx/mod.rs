use connection::default_connection_pool;
use income_repository::SqlxIncomeRepository;
use sqlx::SqlitePool;

use super::IncomeRepository;

mod connection;
mod criteria;
mod income_repository;
mod record;

pub async fn default_income_repository() -> impl IncomeRepository {
    let pool = default_connection_pool().await.unwrap();
    SqlxIncomeRepository::new(pool.clone())
}

pub async fn income_repository(pool: SqlitePool) -> impl IncomeRepository {
    SqlxIncomeRepository::new(pool)
}

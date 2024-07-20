use connection::default_connection_pool;
use income_repository::SqlxIncomeRepository;

use super::IncomeRepository;

mod connection;
mod criteria;
mod income_repository;
mod record;

pub async fn income_repository() -> impl IncomeRepository {
    let pool = default_connection_pool().await.unwrap();
    SqlxIncomeRepository::new(pool.clone())
}

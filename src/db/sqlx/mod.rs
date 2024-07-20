use connection::default_connection_pool;
use income_repository::SqlxIncomeRepository;
use payment_repository::SqlxPaymentRepository;
use sqlx::SqlitePool;

use crate::config::load_config;

use super::{IncomeRepository, PaymentRepository};

mod connection;
mod criteria;
mod income_repository;
mod payment_repository;
mod record;

pub async fn default_income_repository() -> impl IncomeRepository {
    let pool = default_connection_pool().await.unwrap();
    income_repository(pool).await
}

pub async fn income_repository(pool: SqlitePool) -> impl IncomeRepository {
    SqlxIncomeRepository::new(pool)
}

pub async fn default_payment_repository() -> impl PaymentRepository {
    let pool = default_connection_pool().await.unwrap();
    payment_repository(pool).await
}

pub async fn payment_repository(pool: SqlitePool) -> impl PaymentRepository {
    let config = load_config().unwrap();
    SqlxPaymentRepository::new(pool, config)
}

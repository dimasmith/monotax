use income_repository::SqlxIncomeRepository;
use payment_repository::SqlxPaymentRepository;
use sqlx::SqlitePool;

use crate::db::sqlx::tax_payment_repository::SqlxTaxPaymentRepository;
use crate::domain::repository::{IncomeRepository, PaymentRepository, TaxPaymentRepository};

pub mod connection;
mod criteria;
mod income_repository;
pub mod init;
mod payment_repository;
// TODO: hide behind the trait
mod record;
mod tax_payment_repository;

pub async fn income_repository(pool: SqlitePool) -> impl IncomeRepository {
    SqlxIncomeRepository::new(pool)
}

pub async fn payment_repository(pool: SqlitePool, tax_rate: f64) -> impl PaymentRepository {
    SqlxPaymentRepository::new(pool, tax_rate)
}

pub async fn payment_tax_repository(pool: SqlitePool) -> impl TaxPaymentRepository {
    SqlxTaxPaymentRepository::new(pool)
}

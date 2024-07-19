use connect::connection_pool;
use payment::RusqlitePaymentRepository;

use crate::{config::load_config, db::rusqlite::income::RusqliteIncomeRepository};

pub mod connect;
pub mod income;
pub mod payment;

pub fn create_income_repo() -> anyhow::Result<RusqliteIncomeRepository> {
    let pool = connection_pool()?;
    let income_repo = RusqliteIncomeRepository::new(pool);
    Ok(income_repo)
}

pub fn create_payment_repo() -> anyhow::Result<RusqlitePaymentRepository> {
    let pool = connection_pool()?;
    let config = load_config()?;
    let payment_repo = RusqlitePaymentRepository::new(pool, config);
    Ok(payment_repo)
}

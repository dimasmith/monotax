use connect::connection_pool;

use crate::db::rusqlite::income::RusqliteIncomeRepository;

pub mod connect;
pub mod income;

pub fn create_income_repo() -> anyhow::Result<RusqliteIncomeRepository> {
    let pool = connection_pool()?;
    let income_repo = RusqliteIncomeRepository::new(pool);
    Ok(income_repo)
}

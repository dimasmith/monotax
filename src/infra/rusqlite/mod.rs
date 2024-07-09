use crate::infra::rusqlite::income::RusqliteIncomeRepository;
use monotax::db::config::connect;

pub mod income;

pub fn create_income_repo() -> anyhow::Result<RusqliteIncomeRepository> {
    let conn = connect()?;
    let income_repo = RusqliteIncomeRepository::new(conn);
    Ok(income_repo)
}

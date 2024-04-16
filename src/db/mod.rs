//! Database support.
//! The database keeps incomes to query them later.
//! It can be used as a data source instead of reading bank CSV every time.

use crate::income::Income;

use self::config::connect;

mod config;
pub mod init;
pub mod repository;

pub fn save_all(incomes: &[Income]) -> anyhow::Result<()> {
    let mut conn = connect()?;
    repository::save_incomes(&mut conn, incomes)
}

pub fn find_all() -> anyhow::Result<Vec<Income>> {
    let mut conn = connect()?;
    repository::load_all_incomes(&mut conn)
}

//! Database support.
//! The database keeps incomes to query them later.
//! It can be used as a data source instead of reading bank CSV every time.

use crate::config::load_config;
use crate::{income::Income, payment::Payment};

use self::config::connect;
use self::criteria::SqlCriteria;

mod config;
pub mod criteria;
mod date;
pub mod init;
pub mod repository;

pub fn save_all(incomes: &[Income]) -> anyhow::Result<usize> {
    let mut conn = connect()?;
    repository::save_incomes(&mut conn, incomes)
}

pub fn find_all() -> anyhow::Result<Vec<Income>> {
    let mut conn = connect()?;
    repository::load_all_incomes(&mut conn)
}

pub fn find_by_criteria(criteria: impl SqlCriteria) -> anyhow::Result<Vec<Income>> {
    let mut conn = connect()?;
    repository::find_incomes(&mut conn, criteria)
}

pub fn find_payments_by_criteria(criteria: impl SqlCriteria) -> anyhow::Result<Vec<Payment>> {
    let config = load_config()?;
    let tax_rate = config.tax().tax_rate();
    let mut conn = connect()?;
    let records = repository::find_records_by(&mut conn, criteria)?;

    let payments = records
        .into_iter()
        .map(|r| {
            let paid = r.tax_paid();
            let income = r.income();
            Payment::tax_rate(income, tax_rate, paid)
        })
        .collect();
    Ok(payments)
}

pub fn mark_paid(payment_no: i64) -> anyhow::Result<()> {
    let conn = connect()?;
    repository::save_tax_paid(&conn, payment_no, true)
}

pub fn mark_unpaid(payment_no: i64) -> anyhow::Result<()> {
    let conn = connect()?;
    repository::save_tax_paid(&conn, payment_no, false)
}

//! Database support.
//! The database keeps incomes to query them later.
//! It can be used as a data source instead of reading bank CSV every time.

use async_trait::async_trait;

use self::rusqlite::connect::connect;
use crate::config::load_config;
use crate::domain::income::Income;
use crate::domain::payment::Payment;
use crate::income::criteria::IncomeCriteria;

pub mod config;
mod criteria;
mod date;
pub mod init;
pub mod repository;
pub mod rusqlite;

#[async_trait]
pub trait IncomeRepository {
    async fn save_all(&mut self, incomes: &[Income]) -> anyhow::Result<usize>;

    async fn find_all(&mut self) -> anyhow::Result<Vec<Income>>;

    async fn find_by(&mut self, criteria: IncomeCriteria) -> anyhow::Result<Vec<Income>>;
}

pub async fn find_payments_by_criteria(criteria: &IncomeCriteria) -> anyhow::Result<Vec<Payment>> {
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

pub async fn mark_paid(payment_no: i64) -> anyhow::Result<()> {
    let conn = connect()?;
    repository::save_tax_paid(&conn, payment_no, true)
}

pub async fn mark_unpaid(payment_no: i64) -> anyhow::Result<()> {
    let conn = connect()?;
    repository::save_tax_paid(&conn, payment_no, false)
}

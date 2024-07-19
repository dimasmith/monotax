use async_trait::async_trait;
use chrono::{Datelike, NaiveDateTime};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, Row, ToSql};
use tokio::task;

use crate::{
    config::Config,
    db::PaymentRepository,
    domain::{income::Income, payment::Payment},
    income::criteria::IncomeCriteria,
    time::Quarter,
};

use super::criteria::SqlCriteria;

pub struct RusqlitePaymentRepository {
    pool: Pool<SqliteConnectionManager>,
    config: Config,
}

impl RusqlitePaymentRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>, config: Config) -> Self {
        Self { pool, config }
    }
}

#[async_trait]
impl PaymentRepository for RusqlitePaymentRepository {
    async fn find_by(&mut self, criteria: IncomeCriteria) -> anyhow::Result<Vec<Payment>> {
        let tax_rate = self.config.tax().tax_rate();

        let pool = self.pool.clone();
        let records = find_records_by(pool, criteria).await?;

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

    async fn mark_paid(&mut self, payment_no: i64) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        save_tax_paid(pool, payment_no, true).await
    }

    async fn mark_unpaid(&mut self, payment_no: i64) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        save_tax_paid(pool, payment_no, false).await
    }
}

async fn save_tax_paid(
    pool: Pool<SqliteConnectionManager>,
    payment_no: i64,
    tax_paid: bool,
) -> anyhow::Result<()> {
    let pool = pool.clone();
    task::spawn_blocking(move || {
        let conn = pool.get().unwrap();
        let marked = conn.execute(
            "UPDATE income SET tax_paid = :tax_paid WHERE payment_no = :payment_no",
            named_params! {
                ":tax_paid": tax_paid,
                ":payment_no": payment_no
            },
        )?;
        anyhow::ensure!(marked == 1, "payment {} does not exist", payment_no);
        Ok(())
    })
    .await?
}

async fn find_records_by(
    pool: Pool<SqliteConnectionManager>,
    criteria: IncomeCriteria,
) -> anyhow::Result<Vec<IncomeRecord>> {
    let pool = pool.clone();
    task::spawn_blocking(move || {
        let where_clause = if criteria.where_clause().is_empty() {
            String::default()
        } else {
            format!("WHERE {}", criteria.where_clause())
        };
        let query =format!(
                "SELECT date, amount, payment_no, description, tax_paid, year, quarter FROM income {} ORDER BY date ASC",
                where_clause
            );

        let params = criteria.params();
        let params: Vec<(&str, &dyn ToSql)> = params
            .iter()
            .map(|(name, value)| (*name, value as &dyn ToSql))
            .collect();

        let conn = pool.get().unwrap();
        let mut stmt = conn.prepare(&query).unwrap();
        let incomes_records = stmt.query_map(params.as_slice(), map_income_records)
            .unwrap();
        let incomes = incomes_records.map(|r| r.unwrap()).collect::<Vec<_>>();

        Ok(incomes)
    })
    .await?
}

fn map_income_records(row: &Row) -> Result<IncomeRecord, rusqlite::Error> {
    let date = row.get("date")?;
    let amount: f64 = row.get("amount")?;
    let payment_no = row.get("payment_no")?;
    let description: String = row.get("description")?;
    let tax_paid = row.get("tax_paid")?;
    Ok(IncomeRecord::new(
        date,
        amount,
        description,
        tax_paid,
        payment_no,
    ))
}

#[derive(Debug, Clone)]
struct IncomeRecord {
    date: NaiveDateTime,
    amount: f64,
    payment_no: i64,
    description: String,
    year: i32,
    quarter: u32,
    tax_paid: bool,
}

impl IncomeRecord {
    fn new(
        date: NaiveDateTime,
        amount: f64,
        description: String,
        tax_paid: bool,
        payment_no: i64,
    ) -> Self {
        let year = date.year();
        let quarter = Quarter::from(&date).index() as u32;
        Self {
            date,
            amount,
            payment_no,
            description,
            year,
            quarter,
            tax_paid,
        }
    }

    fn tax_paid(&self) -> bool {
        self.tax_paid
    }

    fn income(&self) -> Income {
        Income::new(self.date, self.amount).with_no(self.payment_no)
    }
}

impl From<&Income> for IncomeRecord {
    fn from(income: &Income) -> Self {
        Self::new(
            income.datetime(),
            income.amount(),
            income.comment().unwrap_or_default().to_string(),
            false,
            income.income_no(),
        )
    }
}

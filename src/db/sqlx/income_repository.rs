use async_trait::async_trait;
use sqlx::{pool, SqlitePool};

use super::record::IncomeRecord;
use crate::db::IncomeRepository;
use crate::domain::income::Income;
use crate::income::criteria::IncomeCriteria;

pub struct SqlxIncomeRepository {
    pool: SqlitePool,
}

impl SqlxIncomeRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IncomeRepository for SqlxIncomeRepository {
    async fn save_all(&mut self, incomes: &[Income]) -> anyhow::Result<usize> {
        todo!()
    }

    async fn find_all(&mut self) -> anyhow::Result<Vec<Income>> {
        let pool = &self.pool;
        let records = sqlx::query_as!(
            IncomeRecord,
            r#"
            SELECT date, amount, payment_no, description, year as "year: u16", quarter as "quarter: u8", tax_paid
            FROM income
            "#
        )
        .fetch_all(pool)
        .await?;
        let incomes = records.into_iter().map(|record| record.into()).collect();
        Ok(incomes)
    }

    async fn find_by(&mut self, criteria: IncomeCriteria) -> anyhow::Result<Vec<Income>> {
        todo!()
    }
}

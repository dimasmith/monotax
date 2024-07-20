use async_trait::async_trait;
use sqlx::SqlitePool;

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
        let income_records = incomes.iter().map(IncomeRecord::from);
        let mut updated = 0;
        let mut tx = self.pool.begin().await?;
        let max_payment_no: i64 = sqlx::query_scalar!(r#"SELECT MAX(payment_no) FROM income"#)
            .fetch_one(&mut *tx)
            .await
            .expect("failed to fetch max payment no")
            .unwrap_or_default();
        for record in income_records {
            let payment_no = max_payment_no + updated as i64 + 1;
            sqlx::query!(
                r#"
                INSERT OR IGNORE INTO income (date, amount, payment_no, description, year, quarter, tax_paid)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
                record.date,
                record.amount,
                payment_no,
                record.description,
                record.year,
                record.quarter,
                record.tax_paid
            )
            .execute(&mut *tx)
            .await?;
            updated += 1;
        }
        tx.commit().await?;
        Ok(updated)
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

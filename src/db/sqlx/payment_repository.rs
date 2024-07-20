use async_trait::async_trait;
use sqlx::{query, QueryBuilder, Sqlite, SqlitePool};

use crate::config::Config;
use crate::db::sqlx::criteria::SqlxCriterion;
use crate::db::sqlx::record::IncomeRecord;
use crate::db::PaymentRepository;
use crate::domain::income::Income;
use crate::domain::payment::Payment;
use crate::income::criteria::{IncomeCriteria, IncomeCriterion};

pub struct SqlxPaymentRepository {
    pool: SqlitePool,
    config: Config,
}

impl SqlxPaymentRepository {
    pub fn new(pool: SqlitePool, config: Config) -> Self {
        Self { pool, config }
    }
}

#[async_trait]
impl PaymentRepository for SqlxPaymentRepository {
    async fn find_by(&mut self, criteria: IncomeCriteria) -> anyhow::Result<Vec<Payment>> {
        let pool = &self.pool;
        let tax_rate = self.config.tax().tax_rate();

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT date, amount, payment_no, description, year, quarter, tax_paid
            FROM income
            "#,
        );
        if !criteria.is_empty() {
            query_builder.push("WHERE 1=1 ");
            for criterion in criteria.criteria().iter() {
                match criterion {
                    IncomeCriterion::Quarter(filter) => {
                        if let Some(params) = filter.bind_param() {
                            query_builder.push(" AND ");
                            query_builder.push(params.0);
                            query_builder.push_bind(params.1);
                        }
                    }
                    IncomeCriterion::Year(filter) => {
                        if let Some(params) = filter.bind_param() {
                            query_builder.push(" AND ");
                            query_builder.push(params.0);
                            query_builder.push_bind(params.1);
                        }
                    }
                }
            }
        }

        let query = query_builder.build_query_as();
        let records: Vec<IncomeRecord> = query.fetch_all(pool).await?;
        let payments: Vec<Payment> = records
            .into_iter()
            .map(|i| {
                let tax_paid = i.tax_paid;
                let income = Income::from(i);
                Payment::tax_rate(income, tax_rate, tax_paid)
            })
            .collect();
        Ok(payments)
    }

    async fn mark_paid(&mut self, payment_no: i64) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;
        let result = query!(
            r#"
            UPDATE income
            SET tax_paid = 1
            WHERE payment_no = ?
            "#,
            payment_no
        )
        .execute(&mut *conn)
        .await?;
        anyhow::ensure!(
            result.rows_affected() == 1,
            "payment {} does not exist",
            payment_no
        );
        Ok(())
    }

    async fn mark_unpaid(&mut self, payment_no: i64) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await?;
        let result = query!(
            r#"
            UPDATE income
            SET tax_paid = 0
            WHERE payment_no = ?
            "#,
            payment_no
        )
        .execute(&mut *conn)
        .await?;
        anyhow::ensure!(
            result.rows_affected() == 1,
            "payment {} does not exist",
            payment_no
        );
        Ok(())
    }
}

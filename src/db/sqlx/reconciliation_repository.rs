use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{types::Uuid, SqlitePool};

use crate::{
    db::sqlx::record::IncomeRecord,
    domain::{
        income::Income,
        reconciliation::{Completeness, Reconciliation, ReconciliationID},
        repository::reconciliation::ReconciliationRepository,
    },
};

pub(super) struct SqlxReconciliationRepository {
    pool: SqlitePool,
}

impl SqlxReconciliationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReconciliationRepository for SqlxReconciliationRepository {
    async fn add(&self, reconciliation: Reconciliation) -> anyhow::Result<ReconciliationID> {
        let id = reconciliation.id();
        let income_id = reconciliation.income_id();
        let payment_id = reconciliation.payment_id();
        let reconciled_amount = reconciliation.reconciled_amount();
        let reconciled_on = reconciliation.reconciled_on();
        let completeness = reconciliation.completeness().as_str();
        sqlx::query!(
            r#"
            INSERT INTO reconciliation (id, income_id, payment_id, amount, reconciliation_date, reconciled)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id, income_id, payment_id, reconciled_amount, reconciled_on, completeness
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn find_unreconciled_incomes(&self) -> anyhow::Result<Vec<Income>> {
        let income_records = sqlx::query_as!(
            IncomeRecord,
            r#"
            SELECT income.payment_no, income.date, income.amount, income.description, income.year as "year: u16", income.quarter as "quarter: u8", income.tax_paid
            FROM income
            LEFT JOIN reconciliation ON reconciliation.income_id = income.payment_no
            WHERE income.payment_no NOT IN
            (SELECT income_id FROM reconciliation WHERE reconciled = 'full')
            ORDER BY income.date
            "#
        ).fetch_all(&self.pool)
        .await?;

        let incomes = income_records.into_iter().map(Income::from).collect();

        Ok(incomes)
    }

    async fn find_unreconciled(&self) -> anyhow::Result<Vec<Reconciliation>> {
        let records = sqlx::query_as!(
            ReconciliationRecord,
            r#"
            SELECT r.id as "id: Uuid", r.income_id, r.payment_id, r.amount, r.reconciliation_date, r.reconciled
            FROM income
            LEFT JOIN reconciliation r ON r.income_id = income.payment_no
            WHERE income.payment_no NOT IN
            (SELECT income_id FROM reconciliation WHERE reconciled = 'full')
            ORDER BY income.date
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let reconciliations = records.into_iter().map(Reconciliation::from).collect();
        Ok(reconciliations)
    }
}

struct ReconciliationRecord {
    id: Uuid,
    income_id: i64,
    payment_id: i64,
    amount: f64,
    reconciliation_date: NaiveDateTime,
    reconciled: String,
}

impl From<ReconciliationRecord> for Reconciliation {
    fn from(record: ReconciliationRecord) -> Self {
        let completeness = Completeness::try_from(record.reconciled).unwrap();
        Reconciliation::hydrate(
            record.id,
            record.income_id,
            record.payment_id,
            record.amount,
            record.reconciliation_date,
            completeness,
        )
    }
}

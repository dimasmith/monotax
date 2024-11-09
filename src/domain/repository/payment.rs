//! Database support.
//! The database keeps incomes to query them later.
//! It can be used as a data source instead of reading bank CSV every time.

use async_trait::async_trait;

use crate::domain::{filter::income::IncomeCriteria, Payment};

#[async_trait]
pub trait PaymentRepository {
    async fn find_by(&mut self, criteria: IncomeCriteria) -> anyhow::Result<Vec<Payment>>;

    async fn mark_paid(&mut self, payment_no: i64) -> anyhow::Result<()>;

    async fn mark_unpaid(&mut self, payment_no: i64) -> anyhow::Result<()>;
}

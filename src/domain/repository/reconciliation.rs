use async_trait::async_trait;

use crate::domain::{
    income::Income,
    reconciliation::{Reconciliation, ReconciliationID},
};

#[async_trait]
pub trait ReconciliationRepository {
    async fn add(&self, reconciliation: Reconciliation) -> anyhow::Result<ReconciliationID>;

    async fn find_unreconciled_incomes(&self) -> anyhow::Result<Vec<Income>>;

    async fn find_unreconciled(&self) -> anyhow::Result<Vec<Reconciliation>>;
}

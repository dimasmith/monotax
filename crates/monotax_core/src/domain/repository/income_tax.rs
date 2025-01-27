use async_trait::async_trait;

use crate::domain::model::income_tax::IncomeTax;

#[async_trait]
pub trait IncomeTaxRepository {
    /// Lists all income taxes.
    async fn find_all(&self) -> anyhow::Result<Vec<IncomeTax>>;
}

//! Database support.
//! The database keeps incomes to query them later.
//! It can be used as a data source instead of reading bank CSV every time.

use async_trait::async_trait;

use crate::domain::{NewTaxPayment, TaxPayment, TaxPaymentID};

#[async_trait]
pub trait TaxPaymentRepository {
    async fn insert_payment(&mut self, new_payment: NewTaxPayment) -> anyhow::Result<TaxPaymentID>;

    async fn find_by_year(&mut self, year: i32) -> anyhow::Result<Vec<TaxPayment>>;
}

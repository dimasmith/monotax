//! Integration with Taxer software.
//!
//! The [taxer](https://taxer.ua/) is a Ukrainian software for accounting and tax reporting.
//! This module provides a way to export income data to a CSV file that can be imported into Taxer.

use std::io::Write;

use csv::Writer;
use serde::{Deserialize, Serialize};

use crate::domain::Income;

pub struct TaxerIncome<'a> {
    income: &'a Income,
    tax_number: &'a str,
    comment: &'a str,
}

/// Configuration for the Taxer import.
///
/// - The `id` is person's national tax identifier.
/// - The `account_name` is the name of the account in the Taxer.
/// - The `default_comment` is a comment that will be used if the income has no comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxerImportConfig {
    id: String,
    account_name: String,
    default_comment: String,
}

/// Export incomes to a CSV file that can be imported into Taxer.
pub fn export_csv<W>(
    income: impl IntoIterator<Item = Income>,
    config: &TaxerImportConfig,
    writer: W,
) -> anyhow::Result<()>
where
    W: Write,
{
    let incomes = income.into_iter().collect::<Vec<_>>();
    let taxer_records: Vec<TaxerIncome> = incomes
        .iter()
        .map(|income| {
            let tax_number = config.id();
            let comment = income.comment().unwrap_or(config.default_comment());
            TaxerIncome::new(income, tax_number, comment)
        })
        .collect();
    let mut csv_writer = csv::WriterBuilder::new().from_writer(writer);
    for record in taxer_records {
        record.write(&mut csv_writer)?;
    }
    Ok(())
}

impl<'a> TaxerIncome<'a> {
    pub fn new(income: &'a Income, tax_number: &'a str, comment: &'a str) -> Self {
        Self {
            income,
            tax_number,
            comment,
        }
    }

    pub fn write<W>(&self, writer: &mut Writer<W>) -> anyhow::Result<()>
    where
        W: Write,
    {
        let date = self.income.date().format("%d.%m.%Y").to_string();
        let amount = format!("{:.2}", self.income.amount());
        writer.write_record([self.tax_number, date.as_str(), &amount, self.comment])?;
        Ok(())
    }
}

impl TaxerImportConfig {
    /// Provide the national tax identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Provide the name of the account in the Taxer.
    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    /// Provide the default comment that will be used if the income has no comment.
    pub fn default_comment(&self) -> &str {
        &self.default_comment
    }
}

impl Default for TaxerImportConfig {
    fn default() -> Self {
        Self {
            id: "1234567890".to_string(),
            account_name: Default::default(),
            default_comment: Default::default(),
        }
    }
}

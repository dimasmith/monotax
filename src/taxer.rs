//! Integration with Taxer software.
//!
//! The [taxer](https://taxer.ua/) is a Ukrainian software for accounting and tax reporting.
//! This module provides a way to export income data to a CSV file that can be imported into Taxer.

use std::io::Write;

use csv::Writer;

use crate::config::TaxerImportConfig;
use crate::domain::Income;

pub struct TaxerIncome<'a> {
    income: &'a Income,
    tax_number: &'a str,
    comment: &'a str,
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

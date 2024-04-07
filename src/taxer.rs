use std::io::Write;

use csv::Writer;

use crate::{config::TaxerImportConfig, income::Income};

pub struct TaxerIncome {
    income: Income,
    tax_number: String,
    comment: String,
}

pub fn export_csv<W>(income: &[Income], config: &TaxerImportConfig, writer: W) -> anyhow::Result<()>
where
    W: Write,
{
    let taxer_records: Vec<TaxerIncome> = income
        .iter()
        .map(|income| {
            let tax_number = &config.id;
            let comment = income.comment().unwrap_or(config.default_comment.to_string());
            TaxerIncome::new(income.clone(), tax_number, comment)
        })
        .collect();
    let mut csv_writer = csv::WriterBuilder::new().from_writer(writer);
    for record in taxer_records {
        record.write(&mut csv_writer)?;
    }
    Ok(())
}

impl TaxerIncome {
    pub fn new(income: Income, tax_number: impl Into<String>, comment: impl Into<String>) -> Self {
        Self {
            income,
            tax_number: tax_number.into(),
            comment: comment.into(),
        }
    }

    pub fn write<W>(&self, writer: &mut Writer<W>) -> anyhow::Result<()>
    where
        W: Write,
    {
        let date = self.income.date().format("%d.%m.%Y").to_string();
        let amount = format!("{:.2}", self.income.amount());
        writer.write_record([&self.tax_number, &date, &amount, &self.comment])?;
        Ok(())
    }
}

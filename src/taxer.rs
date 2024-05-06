use std::io::Write;

use csv::Writer;

use crate::{config::TaxerImportConfig, income::Income};

pub struct TaxerIncome<'a> {
    income: &'a Income,
    tax_number: &'a str,
    comment: &'a str,
}

pub fn export_csv<W>(income: &[Income], config: &TaxerImportConfig, writer: W) -> anyhow::Result<()>
where
    W: Write,
{
    let taxer_records: Vec<TaxerIncome> = income
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

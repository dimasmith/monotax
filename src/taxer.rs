use std::io::Write;

use csv::Writer;

use crate::income::{DescribedIncome, Income};

pub struct TaxerIncome {
    income: Income,
    tax_number: String,
    comment: String,
}

pub fn export_csv<W, I>(income: &[I], writer: W) -> anyhow::Result<()>
where
    W: Write,
    I: DescribedIncome,
{
    let taxer_records: Vec<TaxerIncome> = income.iter().map(TaxerIncome::from_income).collect();
    let mut csv_writer = csv::WriterBuilder::new().from_writer(writer);
    for record in taxer_records {
        record.write(&mut csv_writer)?;
    }
    Ok(())
}

impl TaxerIncome {
    pub fn new(income: Income, tax_number: String, comment: String) -> Self {
        Self {
            income,
            tax_number,
            comment,
        }
    }

    pub fn from_income(income: &impl DescribedIncome) -> Self {
        TaxerIncome::new(income.income(), income.tax_number(), income.comment())
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

impl DescribedIncome for TaxerIncome {
    fn income(&self) -> Income {
        self.income
    }

    fn tax_number(&self) -> String {
        self.tax_number.clone()
    }

    fn comment(&self) -> String {
        self.comment.clone()
    }
}

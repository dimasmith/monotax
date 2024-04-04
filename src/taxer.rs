use chrono::NaiveDate;
use csv::Writer;

use crate::income::Income;

pub struct TaxerIncome {
    tax_number: String,
    date: NaiveDate,
    amount: f64,
    comment: String,
}

impl TaxerIncome {
    pub fn new(tax_number: String, date: NaiveDate, amount: f64, comment: String) -> Self {
        Self {
            tax_number,
            date,
            amount,
            comment,
        }
    }

    pub fn from_income(income: &impl Income) -> Self {
        Self {
            tax_number: income.tax_number(),
            date: income.date(),
            amount: income.amount(),
            comment: income.comment(),
        }
    }

    pub fn write<W>(&self, writer: &mut Writer<W>) -> anyhow::Result<()>
    where
        W: std::io::Write,
    {
        let date = self.date.format("%d.%m.%Y").to_string();
        let amount = format!("{:.2}", self.amount);
        writer.write_record(&[&self.tax_number, &date, &amount, &self.comment])?;
        Ok(())
    }
}

impl Income for TaxerIncome {
    fn tax_number(&self) -> String {
        self.tax_number.clone()
    }

    fn date(&self) -> NaiveDate {
        self.date
    }

    fn amount(&self) -> f64 {
        self.amount
    }

    fn comment(&self) -> String {
        self.comment.clone()
    }
}

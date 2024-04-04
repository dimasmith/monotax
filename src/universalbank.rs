use anyhow::Context;
use chrono::NaiveDate;
use csv::StringRecord;

use crate::income::Income;

#[derive(Debug, Clone)]
pub struct UnivesralBankIncome {
    tax_number: String,
    date: NaiveDate,
    amount: f64,
    comment: String,
}

impl UnivesralBankIncome {
    pub fn new(tax_number: String, date: NaiveDate, amount: f64, comment: String) -> Self {
        Self {
            tax_number,
            date,
            amount,
            comment,
        }
    }
}

const DATE_COLUMN: usize = 12;
const AMOUNT_COLUMN: usize = 14;
const REGISTRATION_NO_COLUMN: usize = 0;
const DESCRIPTION_COLUMN: usize = 15;

impl TryFrom<StringRecord> for UnivesralBankIncome {
    type Error = anyhow::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let date = value
            .get(DATE_COLUMN)
            .ok_or_else(|| anyhow::anyhow!("date not found"))?;
        let amount = value
            .get(AMOUNT_COLUMN)
            .ok_or_else(|| anyhow::anyhow!("amount not found"))?;
        let registration_no = value
            .get(REGISTRATION_NO_COLUMN)
            .ok_or_else(|| anyhow::anyhow!("registration_no not found"))?;
        let description = value.get(DESCRIPTION_COLUMN).unwrap_or_default();

        let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").context("failed to parse date")?;
        let amount = amount.parse().context("failed to parse amount")?;

        Ok(Self {
            tax_number: registration_no.to_owned(),
            date,
            amount,
            comment: description.to_owned(),
        })
    }
}

impl Income for UnivesralBankIncome {
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

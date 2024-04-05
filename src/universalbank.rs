use std::io::{BufReader, Read};

use anyhow::Context;
use chrono::NaiveDate;
use csv::StringRecord;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;

use crate::income::{DescribedIncome, Income};

#[derive(Debug, Clone)]
pub struct UnivesralBankIncome {
    income: Income,
    tax_number: String,
    comment: String,
}

impl UnivesralBankIncome {
    pub fn new(income: Income, tax_number: String, comment: String) -> Self {
        Self {
            income,
            tax_number,
            comment,
        }
    }
}

const DATE_COLUMN: usize = 12;
const AMOUNT_COLUMN: usize = 14;
const REGISTRATION_NO_COLUMN: usize = 0;
const DESCRIPTION_COLUMN: usize = 15;

pub fn read_incomes<R>(reader: R) -> anyhow::Result<Vec<impl DescribedIncome>>
where
    R: Read,
{
    let mut reader = DecodingReader::new(BufReader::new(reader), WINDOWS_1251.new_decoder());
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(&mut reader);
    let mut incomes = Vec::new();
    for result in csv_reader.records() {
        let record = result.context("failed to read record")?;
        let income = UnivesralBankIncome::try_from(record)?;
        incomes.push(income);
    }
    Ok(incomes)
}

impl TryFrom<StringRecord> for UnivesralBankIncome {
    type Error = anyhow::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let date = value
            .get(DATE_COLUMN)
            .ok_or_else(|| anyhow::anyhow!("date not found"))?;
        let amount = value
            .get(AMOUNT_COLUMN)
            .ok_or_else(|| anyhow::anyhow!("amount not found"))?;
        let tax_number = value
            .get(REGISTRATION_NO_COLUMN)
            .ok_or_else(|| anyhow::anyhow!("registration_no not found"))?;
        let comment = value.get(DESCRIPTION_COLUMN).unwrap_or_default();

        let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").context("failed to parse date")?;
        let amount = amount.parse().context("failed to parse amount")?;

        let income = Income::new(date, amount);

        Ok(UnivesralBankIncome::new(
            income,
            tax_number.to_owned(),
            comment.to_owned(),
        ))
    }
}

impl DescribedIncome for UnivesralBankIncome {
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

impl DescribedIncome for &UnivesralBankIncome {
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

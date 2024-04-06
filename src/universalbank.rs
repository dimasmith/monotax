use std::io::{BufReader, Read};

use anyhow::Context;
use chrono::NaiveDate;
use csv::StringRecord;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;

use crate::{filter::IncomeFilter, income::Income};

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
const DESCRIPTION_COLUMN: usize = 15;

pub fn read_incomes<R>(reader: R, filter: &IncomeFilter) -> anyhow::Result<Vec<Income>>
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
        let income = income_from_csv(&record)?;
        if filter.test(&income) {
            incomes.push(income);
        }
    }
    Ok(incomes)
}

fn income_from_csv(record: &StringRecord) -> anyhow::Result<Income> {
    let date = record
        .get(DATE_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("date not found"))?;
    let amount = record
        .get(AMOUNT_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("amount not found"))?;
    let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").context("failed to parse date")?;
    let amount = amount.parse().context("failed to parse amount")?;
    Ok(Income::new(date, amount))
}

//! Import format for DBOsoft banking export
//!
//! The format is used by the popular online bank.
use std::io::{BufReader, Read};

use anyhow::Context;
use chrono::NaiveDateTime;
use csv::StringRecord;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;

use crate::domain::Income;
use crate::filter::IncomePredicate;

const DATE_COLUMN: usize = 4;
const AMOUNT_COLUMN: usize = 14;
const DESCRIPTION_COLUMN: usize = 15;

pub fn read_incomes<R>(reader: R, filter: impl IncomePredicate) -> anyhow::Result<Vec<Income>>
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
    incomes.sort();
    Ok(incomes)
}

fn income_from_csv(record: &StringRecord) -> anyhow::Result<Income> {
    let date = record
        .get(DATE_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("date not found"))?;
    let amount = record
        .get(AMOUNT_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("amount not found"))?;
    let comment = record
        .get(DESCRIPTION_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("comment not found"))?;
    let date =
        NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S").context("failed to parse date")?;
    let amount = amount.parse().context("failed to parse amount")?;
    Ok(Income::new(date, amount).with_comment(comment.to_string()))
}

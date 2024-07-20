use chrono::{Datelike, NaiveDateTime};

use crate::{domain::income::Income, time::Quarter};

pub struct IncomeRecord {
    pub date: NaiveDateTime,
    pub amount: f64,
    pub payment_no: i64,
    pub description: Option<String>,
    pub year: u16,
    pub quarter: u8,
    pub tax_paid: bool,
}

impl From<IncomeRecord> for Income {
    fn from(record: IncomeRecord) -> Self {
        Income::new(record.date, record.amount).with_no(record.payment_no)
    }
}

impl From<&Income> for IncomeRecord {
    fn from(value: &Income) -> Self {
        let quarter = Quarter::from(&value.datetime()).index();
        Self {
            date: value.datetime(),
            amount: value.amount(),
            payment_no: value.income_no(),
            description: value.comment().map(|s| s.to_string()),
            year: value.datetime().year() as u16,
            quarter: quarter as u8,
            tax_paid: false,
        }
    }
}

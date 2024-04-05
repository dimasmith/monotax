use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Income {
    date: NaiveDate,
    amount: f64,
}

pub trait DescribedIncome {
    fn income(&self) -> Income;
    fn tax_number(&self) -> String;
    fn comment(&self) -> String;
    fn quarter(&self) -> u32 {
        let date = self.income().date();
        let month = date.month();
        match month {
            1..=3 => 1,
            4..=6 => 2,
            7..=9 => 3,
            10..=12 => 4,
            _ => unreachable!(),
        }
    }
}

impl Income {
    pub fn new(date: NaiveDate, amount: f64) -> Self {
        Self { date, amount }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl AsRef<NaiveDate> for Income {
    fn as_ref(&self) -> &NaiveDate {
        &self.date
    }
}

use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Income {
    date: NaiveDate,
    amount: f64,
}

pub trait DescribedIncome {
    fn income(&self) -> Income;
    fn tax_number(&self) -> String;
    fn comment(&self) -> String;
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

use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Income {
    date: NaiveDate,
    amount: f64,
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

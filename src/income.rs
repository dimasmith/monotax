use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct Income {
    date: NaiveDate,
    amount: f64,
    comment: Option<String>,
}

impl Income {
    pub fn new(date: NaiveDate, amount: f64) -> Self {
        Self { date, amount, comment: None }
    }

    pub fn with_comment(self, comment: String) -> Self {
        Income {
            date: self.date,
            amount: self.amount,
            comment: Some(comment),
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }
}

impl AsRef<NaiveDate> for Income {
    fn as_ref(&self) -> &NaiveDate {
        &self.date
    }
}
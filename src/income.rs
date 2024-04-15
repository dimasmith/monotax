use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Income {
    date: NaiveDate,
    amount: f64,
    comment: Option<String>,
}

impl Income {
    pub fn new(date: NaiveDate, amount: f64) -> Self {
        Self {
            date,
            amount,
            comment: None,
        }
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

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
}

impl AsRef<NaiveDate> for Income {
    fn as_ref(&self) -> &NaiveDate {
        &self.date
    }
}

use std::cmp::Ordering;

impl Ord for Income {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Income {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Income {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.amount == other.amount
    }
}

impl Eq for Income {}

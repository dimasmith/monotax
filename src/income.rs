use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Income {
    income_no: i64,
    date: NaiveDateTime,
    amount: f64,
    comment: Option<String>,
}

impl Income {
    pub fn new(date: NaiveDateTime, amount: f64) -> Self {
        Self {
            income_no: 0,
            date,
            amount,
            comment: None,
        }
    }

    pub fn from_date(date: NaiveDate, amount: f64) -> Self {
        Self {
            income_no: 0,
            date: date.and_hms_opt(0, 0, 0).unwrap(),
            amount,
            comment: None,
        }
    }

    pub fn with_comment(self, comment: String) -> Self {
        Income {
            income_no: 0,
            date: self.date,
            amount: self.amount,
            comment: Some(comment),
        }
    }

    pub fn with_no(self, income_no: i64) -> Self {
        Income {
            income_no,
            date: self.date,
            amount: self.amount,
            comment: self.comment,
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date.date()
    }

    pub fn datetime(&self) -> NaiveDateTime {
        self.date
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn income_no(&self) -> i64 {
        self.income_no
    }
}

impl AsRef<NaiveDateTime> for Income {
    fn as_ref(&self) -> &NaiveDateTime {
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

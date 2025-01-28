use chrono::{NaiveDate, NaiveDateTime};
use std::{cmp::Ordering, fmt::Display, iter::Sum, ops::Add};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Income {
    income_no: i64,
    date: NaiveDateTime,
    amount: Amount,
    comment: Option<String>,
}

const MAX_AMOUNT: f64 = 1000000000.0;

/// Monetary amount.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Amount(f64);

#[derive(Debug, Clone, Error)]
#[error("invalid amount {invalid_amount}")]
pub struct AmountError {
    pub invalid_amount: f64,
}

impl Income {
    pub fn new(date: NaiveDateTime, amount: Amount) -> Self {
        Self {
            income_no: 0,
            date,
            amount,
            comment: None,
        }
    }

    pub fn from_date(date: NaiveDate, amount: Amount) -> Self {
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

    pub fn amount(&self) -> Amount {
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

impl Amount {
    pub const ZERO: Amount = Amount(0.0);
    pub fn new(raw: f64) -> Result<Amount, AmountError> {
        let acceptable_amounts = 0.0..MAX_AMOUNT;
        if !acceptable_amounts.contains(&raw) {
            return Err(AmountError {
                invalid_amount: raw,
            });
        }
        Ok(Self(raw))
    }

    pub fn amount(&self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Amount {
    type Error = AmountError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Amount::new(value)
    }
}

impl Eq for Amount {}

impl PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Amount {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(ordering) => ordering,
            None => unreachable!(),
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Amount::ZERO
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount::new(self.amount() + rhs.amount()).unwrap()
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, v| acc + v).unwrap_or_default()
    }
}

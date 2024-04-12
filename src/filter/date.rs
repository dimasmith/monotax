//! Filters items by quarters and years

use chrono::{Datelike, Local, NaiveDate};

use crate::income::Income;
use crate::time::Quarter;

use super::IncomePredicate;

/// Predicate that filters incomes by year.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum YearFilter {
    /// Filter by a specific year.
    One(i32),
    /// Accept income from any year.
    Any,
    /// Filter by the current year.
    #[default]
    Current,
}

/// Predicate that filters incomes by quarter.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum QuarterFilter {
    /// Filter by a specific quarter.
    Only(Quarter),
    /// Filter by the year-to-date quarter.
    Ytd(Quarter),
    /// Accept income from any quarter.
    Any,
    /// Filter by the current quarter.
    #[default]
    Current,
    /// Filter by the current quarter and all previous quarters.
    CurrentToDate,
}

impl QuarterFilter {
    pub fn filter(&self, date: &NaiveDate) -> bool {
        match self {
            QuarterFilter::Only(quarter) => *quarter == Quarter::from(date),
            QuarterFilter::Ytd(quarter) => *quarter >= Quarter::from(date),
            QuarterFilter::Any => true,
            QuarterFilter::Current => Quarter::current() == Quarter::from(date),
            QuarterFilter::CurrentToDate => Quarter::current() >= Quarter::from(date),
        }
    }

    pub fn filter_income(&self, income: &Income) -> bool {
        self.filter(&income.date())
    }
}

impl YearFilter {
    pub fn filter(&self, date: &NaiveDate) -> bool {
        match self {
            YearFilter::One(year) => *year == date.year(),
            YearFilter::Any => true,
            YearFilter::Current => Local::now().naive_local().year() == date.year(),
        }
    }

    pub fn filter_income(&self, income: &Income) -> bool {
        self.filter(&income.date())
    }
}

impl IncomePredicate for QuarterFilter {
    fn test(&self, income: &Income) -> bool {
        self.filter_income(income)
    }
}

impl IncomePredicate for YearFilter {
    fn test(&self, income: &Income) -> bool {
        self.filter_income(income)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_exact_quarter() {
        let q1_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let q2_date = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let q3_date = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        let q4_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let dates = vec![q1_date, q2_date, q3_date, q4_date];

        let filtered = dates
            .iter()
            .filter(|d| QuarterFilter::Only(Quarter::Q1).filter(d))
            .collect::<Vec<_>>();

        assert_eq!(&filtered, &[&q1_date]);

        let filtered = dates
            .iter()
            .filter(|d| QuarterFilter::Only(Quarter::Q3).filter(d))
            .collect::<Vec<_>>();

        assert_eq!(&filtered, &[&q3_date]);
    }

    #[test]
    fn filter_by_ytd_quarter() {
        let q1_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let q2_date = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let q3_date = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        let q4_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let dates = vec![q1_date, q2_date, q3_date, q4_date];

        let filtered = dates
            .iter()
            .filter(|d| QuarterFilter::Ytd(Quarter::Q3).filter(d))
            .collect::<Vec<_>>();

        assert_eq!(&filtered, &[&q1_date, &q2_date, &q3_date]);
    }
}

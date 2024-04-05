//! Filters items by quarters and years

use chrono::{Datelike, Local, NaiveDate};

use crate::income::DescribedIncome;
use crate::time::Quarter;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum YearFilter {
    OneYear(i32),
    AnyYear,
    #[default]
    CurrentYear,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum QuarterFilter {
    OneQuarter(Quarter),
    Ytd(Quarter),
    AllQuarters,
    #[default]
    CurrentQuarter,
    CurrentQuarterYtd,
}

impl QuarterFilter {
    pub fn filter(&self, date: &NaiveDate) -> bool {
        match self {
            QuarterFilter::OneQuarter(quarter) => *quarter == Quarter::from(date),
            QuarterFilter::Ytd(quarter) => *quarter >= Quarter::from(date),
            QuarterFilter::AllQuarters => true,
            QuarterFilter::CurrentQuarter => Quarter::current() == Quarter::from(date),
            QuarterFilter::CurrentQuarterYtd => Quarter::current() >= Quarter::from(date),
        }
    }

    pub fn filter_income<T: DescribedIncome>(&self, income: &T) -> bool {
        self.filter(&income.income().date())
    }
}

impl YearFilter {
    pub fn filter(&self, date: &NaiveDate) -> bool {
        match self {
            YearFilter::OneYear(year) => *year == date.year(),
            YearFilter::AnyYear => true,
            YearFilter::CurrentYear => Local::now().naive_local().year() == date.year(),
        }
    }

    pub fn filter_income<T: DescribedIncome>(&self, income: &T) -> bool {
        self.filter(&income.income().date())
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
            .filter(|d| QuarterFilter::OneQuarter(Quarter::Q1).filter(d))
            .collect::<Vec<_>>();

        assert_eq!(&filtered, &[&q1_date]);

        let filtered = dates
            .iter()
            .filter(|d| QuarterFilter::OneQuarter(Quarter::Q3).filter(d))
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

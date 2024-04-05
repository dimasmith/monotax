//! Filters items by quarters and years

use chrono::{Datelike, Local, NaiveDate};

use crate::income::DescribedIncome;

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl QuarterFilter {
    pub fn filter(&self, date: &NaiveDate) -> bool {
        match self {
            QuarterFilter::OneQuarter(quarter) => *quarter == Quarter::of_date(date),
            QuarterFilter::Ytd(quarter) => *quarter >= Quarter::of_date(date),
            QuarterFilter::AllQuarters => true,
            QuarterFilter::CurrentQuarter => Quarter::current() == Quarter::of_date(date),
            QuarterFilter::CurrentQuarterYtd => Quarter::current() >= Quarter::of_date(date),
            _ => true,
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
            _ => true,
        }
    }

    pub fn filter_income<T: DescribedIncome>(&self, income: &T) -> bool {
        self.filter(&income.income().date())
    }
}

impl Quarter {
    fn of_date(date: &NaiveDate) -> Self {
        let month = date.month();
        match month {
            1..=3 => Self::Q1,
            4..=6 => Self::Q2,
            7..=9 => Self::Q3,
            10..=12 => Self::Q4,
            _ => unreachable!(),
        }
    }

    fn current() -> Self {
        let date = Local::now().naive_local().date();
        Self::of_date(&date)
    }

    fn as_int(&self) -> usize {
        match self {
            Self::Q1 => 1,
            Self::Q2 => 2,
            Self::Q3 => 3,
            Self::Q4 => 4,
        }
    }
}

impl Ord for Quarter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_int().cmp(&other.as_int())
    }
}

impl TryFrom<u32> for Quarter {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Q1),
            2 => Ok(Self::Q2),
            3 => Ok(Self::Q3),
            4 => Ok(Self::Q4),
            _ => anyhow::bail!("Invalid quarter number"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn detect_quarter() {
        assert_eq!(
            Quarter::of_date(&NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()),
            Quarter::Q1
        );
        assert_eq!(
            Quarter::of_date(&NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()),
            Quarter::Q2
        );
        assert_eq!(
            Quarter::of_date(&NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()),
            Quarter::Q3
        );
        assert_eq!(
            Quarter::of_date(&NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
            Quarter::Q4
        );
    }

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

    #[test]
    fn compare_quarters() {
        assert!(Quarter::Q1 < Quarter::Q2);
        assert!(Quarter::Q2 < Quarter::Q3);
        assert!(Quarter::Q3 < Quarter::Q4);
        assert!(Quarter::Q2 == Quarter::Q2);
    }
}

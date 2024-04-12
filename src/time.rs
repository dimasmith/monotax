//! Time-related items.

use chrono::{Datelike, Local, NaiveDate};
use clap::ValueEnum;
use std::fmt::Display;

/// A quarter of the year.
///
/// The reporting and taxation are usually based on quarters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Quarter {
    pub fn current() -> Self {
        let date = Local::now().naive_local().date();
        Quarter::from(&date)
    }

    pub fn of(date_ref: impl AsRef<NaiveDate>) -> Self {
        Quarter::from(date_ref.as_ref())
    }

    fn index(&self) -> usize {
        match self {
            Self::Q1 => 1,
            Self::Q2 => 2,
            Self::Q3 => 3,
            Self::Q4 => 4,
        }
    }
}

impl PartialOrd for Quarter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Quarter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index().cmp(&other.index())
    }
}

impl Display for Quarter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Q{}", self.index())
    }
}

impl From<&NaiveDate> for Quarter {
    fn from(date: &NaiveDate) -> Self {
        let month = date.month();
        match month {
            1..=3 => Self::Q1,
            4..=6 => Self::Q2,
            7..=9 => Self::Q3,
            10..=12 => Self::Q4,
            _ => unreachable!(),
        }
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

    use crate::income::Income;

    use super::*;

    #[test]
    fn quarter_from_date() {
        assert_eq!(
            Quarter::from(&NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()),
            Quarter::Q1
        );
        assert_eq!(
            Quarter::from(&NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()),
            Quarter::Q2
        );
        assert_eq!(
            Quarter::from(&NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()),
            Quarter::Q3
        );
        assert_eq!(
            Quarter::from(&NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
            Quarter::Q4
        );
    }

    #[test]
    fn quarter_from_number() {
        assert!(matches!(Quarter::try_from(1), Ok(Quarter::Q1)));
        assert!(matches!(Quarter::try_from(2), Ok(Quarter::Q2)));
        assert!(matches!(Quarter::try_from(3), Ok(Quarter::Q3)));
        assert!(matches!(Quarter::try_from(4), Ok(Quarter::Q4)));
        assert!(matches!(Quarter::try_from(0), Err(_)));
        assert!(matches!(Quarter::try_from(5), Err(_)));
    }

    #[test]
    fn compare_quarters() {
        assert!(Quarter::Q1 < Quarter::Q2);
        assert!(Quarter::Q2 < Quarter::Q3);
        assert!(Quarter::Q3 < Quarter::Q4);
        assert!(Quarter::Q2 == Quarter::Q2);
    }

    #[test]
    fn income_quarter() {
        let income = Income::new(NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(), 1000.0);
        let quarter = Quarter::of(&income);

        assert_eq!(quarter, Quarter::Q1);
    }
}

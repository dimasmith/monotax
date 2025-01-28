//! Taxes that are dependent on incomes.

use std::ops::Mul;

use anyhow::Result;
use chrono::NaiveDate;
use thiserror::Error;
use uuid::Uuid;

use super::income::Amount;

pub type TaxID = Uuid;

#[derive(Debug)]
pub struct IncomeTax {
    id: TaxID,
    name: String,
    rates: Vec<IncomeTaxRate>,
}

#[derive(Debug)]
pub struct IncomeTaxRate {
    start: NaiveDate,
    end: NaiveDate,
    rate: TaxRate,
}

#[derive(Debug, Clone, Error)]
#[error("the end date {end_date} is before the start date {start_date}")]
pub struct IcorrectTaxRateDatesError {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TaxRate(f64);

#[derive(Debug, Clone, Error)]
#[error("rate {invalid_rate} is outside of allowed values")]
pub struct TaxRateError {
    pub invalid_rate: f64,
}

impl IncomeTax {
    pub fn new(id: TaxID, name: String, rates: Vec<IncomeTaxRate>) -> Self {
        Self { id, name, rates }
    }

    pub fn new_unchecked(id: String, name: String, rates: Vec<IncomeTaxRate>) -> Self {
        let id = Uuid::parse_str(&id).unwrap();
        Self { id, name, rates }
    }

    pub fn calculate_obligation(&self, income_amount: Amount, income_date: NaiveDate) -> Amount {
        self.rates
            .iter()
            .rfind(|rate| rate.is_applicable(income_date))
            .map(|rate| rate.calculate_obligation(income_amount, income_date))
            .unwrap_or(Amount::ZERO)
    }

    pub fn add_rate_unchecked(&mut self, start_date: NaiveDate, end_date: NaiveDate, rate: f64) {
        let period_rate = IncomeTaxRate::new(start_date, end_date, TaxRate(rate)).unwrap();
        self.rates.push(period_rate);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &TaxID {
        &self.id
    }
}

impl IncomeTaxRate {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        rate: TaxRate,
    ) -> Result<Self, IcorrectTaxRateDatesError> {
        if end_date <= start_date {
            return Err(IcorrectTaxRateDatesError {
                start_date,
                end_date,
            });
        }
        Ok(Self {
            start: start_date,
            end: end_date,
            rate,
        })
    }

    fn is_applicable(&self, date: NaiveDate) -> bool {
        date >= self.start && date < self.end
    }

    fn calculate_obligation(&self, income_amount: Amount, income_date: NaiveDate) -> Amount {
        if self.is_applicable(income_date) {
            income_amount * self.rate
        } else {
            Amount::ZERO
        }
    }
}

impl TaxRate {
    pub fn new(raw: f64) -> Result<Self, TaxRateError> {
        let acceptable_values = 0.0..1.0;
        if !acceptable_values.contains(&raw) {
            return Err(TaxRateError { invalid_rate: raw });
        }
        Ok(Self(raw))
    }

    pub fn rate(&self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for TaxRate {
    type Error = TaxRateError;

    fn try_from(raw: f64) -> std::result::Result<Self, Self::Error> {
        TaxRate::new(raw)
    }
}

impl Eq for TaxRate {}

impl PartialOrd for TaxRate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TaxRate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(ordering) => ordering,
            None => unreachable!(),
        }
    }
}

impl AsRef<f64> for TaxRate {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Mul<TaxRate> for Amount {
    type Output = Amount;

    fn mul(self, rhs: TaxRate) -> Self::Output {
        let raw = self.amount() * rhs.rate();
        // it's safe to unwrap because tax rate is not larger than 1.0,
        // so the result won't ever be higher than the original amount.
        Amount::new(raw).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tax_rate {
        use super::{Amount, TaxRate};

        #[test]
        fn accept_valid_tax_rates() {
            assert_eq!(TaxRate::new(0.05).unwrap().rate(), 0.05);
            assert_eq!(TaxRate::try_from(0.1).unwrap().rate(), 0.1);
            // boundary condition. zero tax can be used for periods when taxes were temporarily
            // disabled.
            assert_eq!(TaxRate::try_from(0.0).unwrap().rate(), 0.0);
        }

        #[test]
        fn reject_invalid_tax_rates() {
            assert!(
                TaxRate::new(-0.01).is_err(),
                "tax rates can't be less than zero"
            );
            assert!(TaxRate::new(1.01).is_err(), "tax rates can't go above 100%");
        }

        #[test]
        fn compare_tax_rates() {
            let low_rate = TaxRate::new(0.05).unwrap();
            let mid_rate = TaxRate::new(0.2).unwrap();
            let another_mid_rate = TaxRate::new(0.2).unwrap();

            assert!(low_rate < mid_rate);
            assert!(mid_rate > low_rate);
            assert!(mid_rate == another_mid_rate);
        }

        #[test]
        fn calculate_tax_amount() {
            let income_amount = Amount::new(250.0).unwrap();
            let tax_rate = TaxRate::new(0.05).unwrap();

            let tax_amount = income_amount * tax_rate;

            assert_eq!(tax_amount, Amount::new(12.5).unwrap());
        }
    }

    mod income_tax_rate {
        use chrono::NaiveDate;

        use crate::domain::model::income_tax::{IncomeTaxRate, TaxRate};

        fn date(year: i32, month: u32, day: u32) -> NaiveDate {
            NaiveDate::from_ymd_opt(year, month, day).unwrap()
        }

        fn valid_rate() -> TaxRate {
            TaxRate::new(0.05).unwrap()
        }

        #[test]
        fn accept_valid_income_tax_rates() {
            let start = date(2008, 1, 1);
            let next_day = date(2008, 1, 2);
            let end = date(2024, 5, 12);

            assert!(IncomeTaxRate::new(start, end, valid_rate()).is_ok());
            assert!(IncomeTaxRate::new(start, next_day, valid_rate()).is_ok());
        }

        #[test]
        fn reject_invalid_tax_rates() {
            let early = date(2008, 1, 1);
            let late = date(2024, 5, 12);
            assert!(
                IncomeTaxRate::new(late, early, valid_rate()).is_err(),
                "the end date can't be before the start date"
            );
            assert!(
                IncomeTaxRate::new(late, late, valid_rate()).is_err(),
                "the end date can't be the same as a start date"
            );
        }
    }
}

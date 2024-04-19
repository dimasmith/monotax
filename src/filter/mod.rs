//! Filters for incomes.

use crate::income::Income;

pub mod date;

/// A predicate that determines whether the income passes the filter.
/// Returns true if income fits.
///
/// You can combine multiple predicates using `IncomeFilter`.
///
/// # Example
/// ```rust
/// # use monotax::income::Income;
/// # use monotax::filter::IncomePredicate;
/// # use chrono::NaiveDateTime;
/// struct LowerAmountPredicate {
///    max_amount: f64,
/// }
///
/// impl IncomePredicate for LowerAmountPredicate {
///    fn test(&self, income: &Income) -> bool {
///       income.amount() <= self.max_amount
///   }
/// }
///
/// let high_income = Income::new(NaiveDate::from_ymd_opt(2024, 04, 04).unwrap(), 1000.0);
/// let low_income = Income::new(NaiveDate::from_ymd_opt(2024, 04, 04).unwrap(), 200.0);
/// let lower_than_500 = LowerAmountPredicate { max_amount: 500.0 };
///
/// assert!(!lower_than_500.test(&high_income), "High income should not pass the filter");
/// assert!(lower_than_500.test(&low_income), "Low income should pass the filter");
///
/// ```
pub trait IncomePredicate {
    /// Determines whether the income passes the filter.
    /// Returns true if income fits.
    fn test(&self, income: &Income) -> bool;

    /// Helper method to convert the predicate to a boxed trait object.
    /// Useful for combining multiple predicates.
    fn boxed(self) -> Box<dyn IncomePredicate>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub struct IncomeFilter {
    predicates: Vec<Box<dyn IncomePredicate>>,
}

impl IncomeFilter {
    pub fn new(predicates: Vec<Box<dyn IncomePredicate>>) -> Self {
        Self { predicates }
    }
}

impl IncomePredicate for IncomeFilter {
    fn test(&self, income: &Income) -> bool {
        for predicate in &self.predicates {
            if !predicate.test(income) {
                return false;
            }
        }
        true
    }
}

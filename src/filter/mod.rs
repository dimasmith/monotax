//! Filters for incomes.

use crate::income::criteria::{IncomeCriteria, IncomeCriterion};
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
/// # use chrono::{NaiveDateTime, NaiveDate};
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
/// let high_income = Income::from_date(NaiveDate::from_ymd_opt(2024, 04, 04).unwrap(), 1000.0);
/// let low_income = Income::from_date(NaiveDate::from_ymd_opt(2024, 04, 04).unwrap(), 200.0);
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
}

impl IncomePredicate for IncomeCriteria {
    fn test(&self, income: &Income) -> bool {
        for c in self.criteria() {
            if !c.test(income) {
                return false;
            }
        }
        true
    }
}

impl IncomePredicate for IncomeCriterion {
    fn test(&self, income: &Income) -> bool {
        match self {
            IncomeCriterion::Quarter(filter) => filter.filter_income(income),
            IncomeCriterion::Year(filter) => filter.filter_income(income),
        }
    }
}

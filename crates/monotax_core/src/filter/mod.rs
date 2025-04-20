//! Filters for incomes.

use crate::domain::{
    filter::income::{IncomeCriteria, IncomeCriterion},
    Income,
};

pub mod date;

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

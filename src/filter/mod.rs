//! Filters for incomes.

use crate::income::Income;

pub mod date;

pub trait IncomePredicate {
    
    /// Determines whether the income passes the filter.
    /// Returns true if income fits.
    fn test(&self, income: &Income) -> bool;
}

pub struct IncomeFilter {
    predicates: Vec<Box<dyn IncomePredicate>>,
}

impl IncomeFilter {
    pub fn new(predicates: Vec<Box<dyn IncomePredicate>>) -> Self {
        Self {
            predicates
        }
    }

    pub fn test(&self, income: &Income) -> bool {
        for predicate in &self.predicates {
            if !predicate.test(income) {
                return false;
            }
        }
        true
    }
}
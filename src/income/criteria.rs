//! Universal criteria filters.

use crate::time::Quarter;

/// A set of filtering criteria to pick necessary incomes.
#[derive(Debug, Clone)]
pub struct IncomeCriteria {
    criteria: Vec<IncomeCriterion>,
}

/// A single filtering element for income.
#[derive(Debug, Clone, Copy)]
pub enum IncomeCriterion {
    Quarter(QuarterFilter),
    Year(YearFilter),
}

impl IncomeCriteria {
    pub fn new(criteria: &[IncomeCriterion]) -> Self {
        Self {
            criteria: criteria.to_vec(),
        }
    }

    pub fn criteria(&self) -> &[IncomeCriterion] {
        &self.criteria
    }
}

impl From<QuarterFilter> for IncomeCriterion {
    fn from(value: QuarterFilter) -> Self {
        IncomeCriterion::Quarter(value)
    }
}

impl From<YearFilter> for IncomeCriterion {
    fn from(value: YearFilter) -> Self {
        IncomeCriterion::Year(value)
    }
}

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

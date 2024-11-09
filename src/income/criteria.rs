//! Filtering criteria for incomes.
//!
//! Criteria allows picking only necessary incomes from the list.
//! Usually, it's used to filter incomes by year or quarter.
//!
//! Main components of filtering API:
//!
//! - [`IncomeCriterion`] - a single predicate that income must conform.
//! - [`IncomeCriteria`] - a combination of all predicates.
//!
//! ## Example
//!
//! The user wants to see incomes of the Q2 of 2021.
//! The first criterion is to filter by the year 2021.
//! The second criterion is to filter by the quarter Q2.
//! All criteria are combined into a single [`IncomeCriteria`] object.
//!
//! ```rust
//! use monotax::income::criteria::{IncomeCriteria, IncomeCriterion, YearFilter, QuarterFilter};
//! use monotax::domain::Quarter;
//!
//! let criteria = IncomeCriteria::new(&[
//!     IncomeCriterion::Year(YearFilter::One(2021)),
//!     IncomeCriterion::Quarter(QuarterFilter::Only(Quarter::Q2)),
//! ]);
//! ```
//!
//! ## Implementations
//!
//! Criteria are implemented for directly filtering iterables of incomes,
//! and also to translate to SQL queries.
//! This way the user can use the same criteria to filter incomes in memory and in the database.

use crate::domain::Quarter;

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

    pub fn is_empty(&self) -> bool {
        self.criteria.is_empty()
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

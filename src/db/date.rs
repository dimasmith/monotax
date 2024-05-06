//! Implement criteria for date-based filters

use chrono::Datelike;

use crate::{
    income::criteria::{IncomeCriterion, QuarterFilter, YearFilter},
    time::Quarter,
};

use super::criteria::SqlCriterion;

impl SqlCriterion for IncomeCriterion {
    fn where_clause(&self) -> Option<String> {
        match self {
            IncomeCriterion::Quarter(f) => f.where_clause(),
            IncomeCriterion::Year(f) => f.where_clause(),
        }
    }

    fn params(&self) -> Option<(&str, rusqlite::types::Value)> {
        match self {
            IncomeCriterion::Quarter(f) => f.params(),
            IncomeCriterion::Year(f) => f.params(),
        }
    }
}

impl SqlCriterion for YearFilter {
    fn where_clause(&self) -> Option<String> {
        let col = "year";
        match self {
            YearFilter::One(_) | YearFilter::Current => Some(format!("{col} = :{col}")),
            YearFilter::Any => None,
        }
    }

    fn params(&self) -> Option<(&str, rusqlite::types::Value)> {
        let param = ":year";
        match self {
            YearFilter::One(year) => Some((param, rusqlite::types::Value::Integer(*year as i64))),
            YearFilter::Current => {
                let year = chrono::Local::now().year();
                Some((param, rusqlite::types::Value::Integer(year as i64)))
            }
            YearFilter::Any => None,
        }
    }
}

impl SqlCriterion for QuarterFilter {
    fn where_clause(&self) -> Option<String> {
        match self {
            QuarterFilter::Any => None,
            QuarterFilter::Only(_) | QuarterFilter::Current => {
                Some("quarter = :quarter".to_string())
            }
            QuarterFilter::Ytd(_) | QuarterFilter::CurrentToDate => {
                Some("quarter <= :quarter".to_string())
            }
        }
    }

    fn params(&self) -> Option<(&str, rusqlite::types::Value)> {
        match self {
            QuarterFilter::Any => None,
            QuarterFilter::Only(quarter) | QuarterFilter::Ytd(quarter) => {
                let quarter_index = quarter.index() as i64;
                Some((":quarter", rusqlite::types::Value::Integer(quarter_index)))
            }
            QuarterFilter::Current | QuarterFilter::CurrentToDate => {
                let now = chrono::Local::now().naive_local();
                let quarter = Quarter::from(&now).index() as i64;
                Some((":quarter", rusqlite::types::Value::Integer(quarter)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Quarter;

    use super::*;

    #[test]
    fn where_clause_for_one_year() {
        let filter = YearFilter::One(2021);
        assert_eq!(filter.where_clause(), Some("year = :year".to_string()));
        assert_eq!(
            filter.params(),
            Some((":year", rusqlite::types::Value::Integer(2021)))
        );
    }

    #[test]
    fn where_clause_for_any_yera() {
        let filter = YearFilter::Any;
        assert_eq!(filter.where_clause(), None);
        assert_eq!(filter.params(), None);
    }

    #[test]
    fn where_clause_for_one_quarter() {
        let filter = QuarterFilter::Only(Quarter::Q2);
        assert_eq!(
            filter.where_clause(),
            Some("quarter = :quarter".to_string())
        );
        assert_eq!(
            filter.params(),
            Some((":quarter", rusqlite::types::Value::Integer(2)))
        );
    }

    #[test]
    fn where_clause_for_ytd_quarter() {
        let filter = QuarterFilter::Ytd(Quarter::Q3);
        assert_eq!(
            filter.where_clause(),
            Some("quarter <= :quarter".to_string())
        );
        assert_eq!(
            filter.params(),
            Some((":quarter", rusqlite::types::Value::Integer(3)))
        );
    }

    #[test]
    fn where_clause_for_any() {
        let filter = QuarterFilter::Any;
        assert_eq!(filter.where_clause(), None);
        assert_eq!(filter.params(), None);
    }
}

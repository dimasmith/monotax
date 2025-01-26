use chrono::Datelike;

use monotax_core::domain::filter::income::{QuarterFilter, YearFilter};
use monotax_core::domain::Quarter;

pub trait SqlxCriterion<T> {
    fn bind_param(&self) -> Option<(&str, T)>;
}

impl SqlxCriterion<i32> for YearFilter {
    fn bind_param(&self) -> Option<(&str, i32)> {
        match self {
            YearFilter::One(year) => Some(("year = ", *year)),
            YearFilter::Any => None,
            YearFilter::Current => {
                let current_year = chrono::Utc::now().year();
                Some(("year = ", current_year))
            }
        }
    }
}

impl SqlxCriterion<u8> for QuarterFilter {
    fn bind_param(&self) -> Option<(&str, u8)> {
        match self {
            QuarterFilter::Any => None,
            QuarterFilter::Ytd(qarter) => Some(("quarter <= ", qarter.index() as u8)),
            QuarterFilter::Only(quarter) => Some(("quarter = ", quarter.index() as u8)),
            QuarterFilter::Current => {
                let current_quarter = Quarter::current().index();
                Some(("quarter = ", current_quarter as u8))
            }
            QuarterFilter::CurrentToDate => {
                let current_quarter = Quarter::current().index();
                Some(("quarter <= ", current_quarter as u8))
            }
        }
    }
}

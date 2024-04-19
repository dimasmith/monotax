use monotax::filter::date::{QuarterFilter, YearFilter};

use crate::cli::{IncludeQuarters, IncludeYears};

use super::FilterArgs;

pub fn build_quarter_filter(cli: &FilterArgs) -> QuarterFilter {
    QuarterFilter::from(cli)
}

pub fn build_year_filter(cli: &FilterArgs) -> YearFilter {
    YearFilter::from(cli)
}

impl From<&FilterArgs> for QuarterFilter {
    fn from(cli: &FilterArgs) -> Self {
        match (cli.include_quarters, cli.quarter) {
            (IncludeQuarters::Any, None) => QuarterFilter::Any,
            (IncludeQuarters::Any, Some(q)) => QuarterFilter::Only(q),
            (IncludeQuarters::One, None) => QuarterFilter::Current,
            (IncludeQuarters::One, Some(q)) => QuarterFilter::Only(q),
            (IncludeQuarters::Ytd, None) => QuarterFilter::CurrentToDate,
            (IncludeQuarters::Ytd, Some(q)) => QuarterFilter::Ytd(q),
        }
    }
}

impl From<&FilterArgs> for YearFilter {
    fn from(cli: &FilterArgs) -> Self {
        match (cli.include_years, cli.year) {
            (IncludeYears::All, None) => YearFilter::Any,
            (IncludeYears::All, Some(y)) => YearFilter::One(y),
            (IncludeYears::One, None) => YearFilter::Current,
            (IncludeYears::One, Some(y)) => YearFilter::One(y),
            (IncludeYears::Current, None) => YearFilter::Current,
            (IncludeYears::Current, Some(y)) => YearFilter::One(y),
        }
    }
}

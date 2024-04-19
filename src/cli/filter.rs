use monotax::filter::date::{QuarterFilter, YearFilter};

use crate::cli::{IncludeQuarters, IncludeYears};

use super::Cli;

pub fn build_quarter_filter(cli: &Cli) -> anyhow::Result<QuarterFilter> {
    let include_quarters = cli.include_quarters;
    let quarter = cli.quarter;

    let quarter_filter = match (include_quarters, quarter) {
        (IncludeQuarters::Any, None) => QuarterFilter::Any,
        (IncludeQuarters::Any, Some(q)) => QuarterFilter::Only(q),
        (IncludeQuarters::One, None) => QuarterFilter::Current,
        (IncludeQuarters::One, Some(q)) => QuarterFilter::Only(q),
        (IncludeQuarters::Ytd, None) => QuarterFilter::CurrentToDate,
        (IncludeQuarters::Ytd, Some(q)) => QuarterFilter::Ytd(q),
    };
    Ok(quarter_filter)
}

pub fn build_year_filter(cli: &Cli) -> anyhow::Result<YearFilter> {
    let year_filter = match (cli.include_years, cli.year) {
        (IncludeYears::All, None) => YearFilter::Any,
        (IncludeYears::All, Some(y)) => YearFilter::One(y),
        (IncludeYears::One, None) => YearFilter::Current,
        (IncludeYears::One, Some(y)) => YearFilter::One(y),
        (IncludeYears::Current, None) => YearFilter::Current,
        (IncludeYears::Current, Some(y)) => YearFilter::One(y),
    };
    Ok(year_filter)
}

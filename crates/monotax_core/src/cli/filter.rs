use crate::domain::filter::income::{IncomeCriteria, IncomeCriterion, QuarterFilter, YearFilter};
use crate::domain::Quarter;
use clap::Args;

use crate::cli::{IncludeQuarters, IncludeYears};

#[derive(Debug, Args, Clone)]
pub struct FilterArgs {
    /// A quarter to filter incomes. Optional.
    #[clap(short, long)]
    #[arg(value_enum)]
    pub quarter: Option<Quarter>,
    #[clap(long)]
    #[arg(value_enum, default_value_t)]
    pub include_quarters: IncludeQuarters,

    /// What years to include in the report.
    #[clap(long)]
    #[arg(value_enum, default_value_t)]
    pub include_years: IncludeYears,

    /// A specific year to filter incomes. Optional.
    #[clap(short, long)]
    #[arg(value_enum)]
    pub year: Option<i32>,
}

impl FilterArgs {
    pub fn criteria(&self) -> IncomeCriteria {
        let quarter_filter = build_quarter_filter(self);
        let year_filter = build_year_filter(self);
        IncomeCriteria::new(&[
            IncomeCriterion::Quarter(quarter_filter),
            IncomeCriterion::Year(year_filter),
        ])
    }
}

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

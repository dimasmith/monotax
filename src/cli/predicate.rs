use monotax::filter::IncomePredicate;

use super::{
    filter::{build_quarter_filter, build_year_filter},
    Cli,
};

pub fn build_predicates(cli: &Cli) -> anyhow::Result<Vec<Box<dyn IncomePredicate>>> {
    let filter_args = &cli.filter;
    let quarter_filter = build_quarter_filter(filter_args);
    let year_filter = build_year_filter(filter_args);
    Ok(vec![quarter_filter.boxed(), year_filter.boxed()])
}

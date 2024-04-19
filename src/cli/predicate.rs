use monotax::filter::IncomePredicate;

use super::filter::{build_quarter_filter, build_year_filter, FilterArgs};

pub fn build_predicates(filter_args: &FilterArgs) -> anyhow::Result<Vec<Box<dyn IncomePredicate>>> {
    let quarter_filter = build_quarter_filter(filter_args);
    let year_filter = build_year_filter(filter_args);
    Ok(vec![quarter_filter.boxed(), year_filter.boxed()])
}

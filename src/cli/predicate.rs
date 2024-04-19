use monotax::filter::IncomePredicate;

use super::{
    filter::{build_quarter_filter, build_year_filter},
    Cli,
};

pub fn build_predicates(cli: &Cli) -> anyhow::Result<Vec<Box<dyn IncomePredicate>>> {
    let quarter_filter = build_quarter_filter(cli)?;
    let year_filter = build_year_filter(cli)?;
    Ok(vec![quarter_filter.boxed(), year_filter.boxed()])
}

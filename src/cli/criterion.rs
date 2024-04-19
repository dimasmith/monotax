use monotax::db::criteria::Criterion;

use super::{
    filter::{build_quarter_filter, build_year_filter},
    Cli,
};

pub fn build_criteria(cli: &Cli) -> anyhow::Result<Vec<Box<dyn Criterion>>> {
    let filter_args = &cli.filter;
    let quarter_filter = build_quarter_filter(filter_args);
    let year_filter = build_year_filter(filter_args);
    Ok(vec![Box::new(quarter_filter), Box::new(year_filter)])
}

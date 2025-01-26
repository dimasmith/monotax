//! Handlers for cli app requests.

use std::{fs::File, path::Path};

use anyhow::Context;
use monotax_core::app::income::read_incomes;
use monotax_dbo::dbo;
use tokio::task::block_in_place;

use monotax_core::domain::repository::IncomeRepository;
use monotax_core::domain::Income;
use monotax_core::infra::io::writer;
use monotax_core::integration::taxer;

use crate::config;

use super::filter::FilterArgs;

pub async fn generate_taxer_report(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let config = config::load_config()?;
    let incomes = read_incomes_from_file_or_db(income_repo, input, filter).await?;
    let writer = writer(output)?;
    taxer::export_csv(incomes, config.taxer(), writer)?;
    Ok(())
}

async fn read_incomes_from_file_or_db(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<Vec<Income>> {
    let incomes = match input {
        Some(statement) => incomes_from_dbo_file(statement, filter).await?,
        None => read_incomes(filter.criteria(), income_repo).await?,
    };
    Ok(incomes)
}

async fn incomes_from_dbo_file(input: &Path, filter: &FilterArgs) -> anyhow::Result<Vec<Income>> {
    let incomes = block_in_place(move || {
        let file = File::open(input).context("opening input file")?;
        dbo::read_incomes(file, filter.criteria())
    })?;
    Ok(incomes)
}

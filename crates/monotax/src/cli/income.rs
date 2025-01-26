use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Subcommand;
use monotax_core::{
    app::income::import_incomes,
    domain::{repository::IncomeRepository, Income},
};
use monotax_dbo::dbo;
use tokio::task::block_in_place;

use super::filter::FilterArgs;

#[derive(Debug, Subcommand)]
pub enum IncomeCommands {
    /// Import incomes from DBOSoft banking statement file.
    ImportDbo {
        statement_file: PathBuf,
        #[command(flatten)]
        filter: FilterArgs,
    },
}

pub async fn process_incomes(
    command: &IncomeCommands,
    income_repository: &mut impl IncomeRepository,
) -> anyhow::Result<()> {
    match command {
        IncomeCommands::ImportDbo {
            statement_file,
            filter,
        } => import_incomes_from_dbo_csv(income_repository, statement_file, filter).await,
    }
}

async fn import_incomes_from_dbo_csv(
    income_repo: &mut impl IncomeRepository,
    statement: &Path,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let incomes = incomes_from_dbo_file(statement, filter).await?;
    let _ = import_incomes(incomes, income_repo).await?;
    Ok(())
}

async fn incomes_from_dbo_file(input: &Path, filter: &FilterArgs) -> anyhow::Result<Vec<Income>> {
    let incomes = block_in_place(move || {
        let file = File::open(input).context("opening input file")?;
        dbo::read_incomes(file, filter.criteria())
    })?;
    Ok(incomes)
}

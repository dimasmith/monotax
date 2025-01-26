//! Invoke cli application with necessary environment and a command.

use monotax_sqlite::income_repository::income_repository;
use sqlx::SqlitePool;

use crate::config::Configuration;
use crate::init::init;

use super::handler::*;
use super::opts::Command;
use super::Cli;

/// Runs a CLI command.
pub async fn handle_command(
    cli: &Cli,
    _config: &Configuration,
    db_pool: SqlitePool,
) -> anyhow::Result<()> {
    let mut income_repo = income_repository(db_pool.clone());

    match &cli.command {
        Command::Init { force } => init(&db_pool, *force).await?,
        Command::Incomes { command } => {
            super::income::process_incomes(command, &mut income_repo).await?
        }

        Command::Taxer {
            input,
            output,
            filter,
        } => {
            generate_taxer_report(
                &mut income_repo,
                input.as_deref(),
                output.as_deref(),
                filter,
            )
            .await?;
        }
    }

    Ok(())
}

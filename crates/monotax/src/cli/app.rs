//! Invoke cli application with necessary environment and a command.

use monotax_sqlite::income_repository::income_repository;
use sqlx::SqlitePool;

use crate::config::Configuration;
use crate::init::init;

use super::handler::*;
use super::Cli;
use super::Command;

/// Runs a CLI command.
pub async fn run_cli_command(
    cli: &Cli,
    _config: &Configuration,
    db_pool: SqlitePool,
) -> anyhow::Result<()> {
    let mut income_repo = income_repository(db_pool.clone());

    match &cli.command {
        Command::Init { force } => init(&db_pool, *force).await?,
        Command::Import { statement, filter } => {
            import_incomes_from_dbo_csv(&mut income_repo, statement, filter).await?;
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

//! Invoke cli application with necessary environment and a command.

use monotax_sqlite::income_repository::income_repository;
use monotax_sqlite::payment_repository::payment_repository;
use monotax_sqlite::tax_payment_repository::payment_tax_repository;
use sqlx::SqlitePool;

use crate::config::Configuration;
use crate::init::init;

use super::handler::*;
use super::payment::PaymentCommands;
use super::Cli;
use super::Command;

/// Runs a CLI command.
pub async fn run_cli_command(
    cli: &Cli,
    config: &Configuration,
    db_pool: SqlitePool,
) -> anyhow::Result<()> {
    let mut income_repo = income_repository(db_pool.clone());
    let mut payment_repo = payment_repository(db_pool.clone(), config.tax().tax_rate());
    let mut tax_payment_repo = payment_tax_repository(db_pool.clone());

    match &cli.command {
        Command::Init { force } => init(&db_pool, *force).await?,
        Command::Import { statement, filter } => {
            import_incomes(&mut income_repo, statement, filter).await?;
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
        Command::Report {
            input,
            format,
            output,
            filter,
        } => {
            generate_incomes_report(
                &mut income_repo,
                input.as_deref(),
                format,
                output.as_deref(),
                filter,
            )
            .await?;
        }
        Command::Payments { command } => match command {
            PaymentCommands::Report { output, filter } => {
                report_payments(&mut payment_repo, output.as_deref(), filter).await?;
            }
            PaymentCommands::Pay { payment_no } => {
                pay_tax(
                    &mut payment_repo,
                    &mut income_repo,
                    &mut tax_payment_repo,
                    *payment_no,
                )
                .await?;
            }
            PaymentCommands::Unpay { payment_no } => {
                cancel_tax_payment(&mut payment_repo, payment_no).await?;
            }
        },
    }

    Ok(())
}

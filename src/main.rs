use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};
use std::path::Path;

use anyhow::Context;

use clap::Parser;
use cli::filter::FilterArgs;
use cli::payment::PaymentCommands;
use cli::ReportFormat;
use cli::{Cli, Command};
use env_logger::{Builder, Env};
use monotax::db::sqlx::{default_income_repository, default_payment_repository};
use monotax::db::{IncomeRepository, PaymentRepository};
use monotax::domain::income::Income;
use monotax::payment::report::plaintext::plaintext_report;
use monotax::payment::report::PaymentReport;
use monotax::report::QuarterlyReport;
use monotax::{config, init, report, taxer, universalbank};
use tokio::task::block_in_place;

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read local .env file
    let _ = dotenvy::dotenv();
    // Initialize logging
    let env = Env::default().filter_or("RUST_LOG", "monotax=info");
    Builder::from_env(env).init();

    let cli = Cli::parse();

    match &cli.command {
        Command::Init { force } => init::init(*force).await?,
        Command::Import { statement, filter } => {
            let mut income_repo = default_income_repository().await;
            import_incomes(&mut income_repo, statement, filter).await?;
        }

        Command::Taxer {
            input,
            output,
            filter,
        } => {
            let mut income_repo = default_income_repository().await;
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
            let mut income_repo = default_income_repository().await;
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
                let mut payment_repo = default_payment_repository().await;
                report_payments(&mut payment_repo, output.as_deref(), filter).await?;
            }
            PaymentCommands::Pay { payment_no } => {
                let mut payment_repo = default_payment_repository().await;
                pay_tax(&mut payment_repo, payment_no).await?;
            }
            PaymentCommands::Unpay { payment_no } => {
                let mut payment_repo = default_payment_repository().await;
                cancel_tax_payment(&mut payment_repo, payment_no).await?;
            }
        },
    }

    Ok(())
}

async fn cancel_tax_payment(
    payments_repo: &mut impl PaymentRepository,
    payment_no: &i64,
) -> anyhow::Result<()> {
    payments_repo.mark_unpaid(*payment_no).await?;
    Ok(())
}

async fn pay_tax(
    payments_repo: &mut impl PaymentRepository,
    payment_no: &i64,
) -> anyhow::Result<()> {
    payments_repo.mark_paid(*payment_no).await?;
    Ok(())
}

async fn report_payments(
    payments_repo: &mut impl PaymentRepository,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let criteria = filter.criteria();
    let payments = payments_repo.find_by(criteria).await?;
    let report = PaymentReport::from_payments(payments);
    let writer = writer(output)?;
    plaintext_report(&report, writer)?;
    Ok(())
}

async fn generate_incomes_report(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    format: &ReportFormat,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let config = config::load_config()?;
    let incomes = read_incomes(income_repo, input, filter).await?;
    let report = QuarterlyReport::build_report(incomes, config.tax());
    let writer = writer(output)?;
    match format {
        ReportFormat::Console => report::console::pretty_print(&report, writer)?,
        ReportFormat::Csv => report::csv::render_csv(&report, writer)?,
    };
    Ok(())
}

async fn generate_taxer_report(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let config = config::load_config()?;
    let incomes = read_incomes(income_repo, input, filter).await?;
    let writer = writer(output)?;
    taxer::export_csv(incomes, config.taxer(), writer)?;
    Ok(())
}

async fn import_incomes(
    income_repo: &mut impl IncomeRepository,
    statement: &Path,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let incomes = read_incomes(income_repo, Some(statement), filter).await?;
    let imported = income_repo
        .save_all(&incomes.into_iter().collect::<Vec<_>>())
        .await?;
    log::info!("Imported {} incomes", imported);
    Ok(())
}

async fn read_incomes(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<impl IntoIterator<Item = Income>> {
    let incomes = match input {
        Some(stmt) => block_in_place(move || {
            let file = File::open(stmt).context("opening input file")?;
            universalbank::read_incomes(file, filter.criteria())
        })?,
        None => income_repo.find_all().await?,
    };
    Ok(incomes)
}

fn writer(output: Option<&Path>) -> anyhow::Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(stdout())),
    };
    Ok(writer)
}

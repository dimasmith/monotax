use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};
use std::path::PathBuf;

use anyhow::Context;

use clap::Parser;
use cli::filter::FilterArgs;
use cli::payment::PaymentCommands;
use cli::ReportFormat;
use cli::{Cli, Command};
use env_logger::{Builder, Env};
use monotax::db::{find_payments_by_criteria, mark_paid, mark_unpaid, IncomeRepository};
use monotax::domain::income::Income;
use monotax::payment::report::plaintext::plaintext_report;
use monotax::payment::report::PaymentReport;
use monotax::report::QuarterlyReport;
use monotax::{config, init, report, taxer, universalbank};
use tokio::task::block_in_place;

mod cli;
mod infra;

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
            let mut income_repo = infra::rusqlite::create_income_repo()?;
            import_incomes(&mut income_repo, statement, filter).await?;
        }

        Command::Taxer {
            input,
            output,
            filter,
        } => {
            let mut income_repo = infra::rusqlite::create_income_repo()?;
            generate_taxer_report(&mut income_repo, input, output, filter).await?;
        }
        Command::Report {
            input,
            format,
            output,
            filter,
        } => {
            let mut income_repo = infra::rusqlite::create_income_repo()?;
            generate_incomes_report(&mut income_repo, input, format, output, filter).await?;
        }
        Command::Payments { command } => match command {
            PaymentCommands::Report { output, filter } => {
                report_payments(output, filter)?;
            }
            PaymentCommands::Pay { payment_no } => {
                pay_tax(payment_no)?;
            }
            PaymentCommands::Unpay { payment_no } => {
                cancel_tax_payment(payment_no)?;
            }
        },
    }

    Ok(())
}

fn cancel_tax_payment(payment_no: &i64) -> anyhow::Result<()> {
    mark_unpaid(*payment_no)?;
    Ok(())
}

fn pay_tax(payment_no: &i64) -> anyhow::Result<()> {
    mark_paid(*payment_no)?;
    Ok(())
}

fn report_payments(output: &Option<PathBuf>, filter: &FilterArgs) -> anyhow::Result<()> {
    let payments = find_payments_by_criteria(filter.criteria())?;
    let report = PaymentReport::from_payments(payments);
    let writer = writer(output)?;
    plaintext_report(&report, writer)?;
    Ok(())
}

async fn generate_incomes_report(
    income_repo: &mut impl IncomeRepository,
    input: &Option<PathBuf>,
    format: &ReportFormat,
    output: &Option<PathBuf>,
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
    input: &Option<PathBuf>,
    output: &Option<PathBuf>,
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
    statement: &PathBuf,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let incomes = read_incomes(income_repo, &Some(statement.clone()), filter).await?;
    let imported = income_repo
        .save_all(&incomes.into_iter().collect::<Vec<_>>())
        .await?;
    log::info!("Imported {} incomes", imported);
    Ok(())
}

fn writer(output: &Option<PathBuf>) -> anyhow::Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(stdout())),
    };
    Ok(writer)
}

async fn read_incomes(
    income_repo: &mut impl IncomeRepository,
    input: &Option<PathBuf>,
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

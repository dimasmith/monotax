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
use monotax::db::{self, find_payments_by_criteria, mark_paid, mark_unpaid};
use monotax::income::Income;
use monotax::payment::report::plaintext::plaintext_report;
use monotax::payment::report::PaymentReport;
use monotax::report::QuarterlyReport;
use monotax::{config, init, report, taxer, universalbank};

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read local .env file
    dotenvy::dotenv()?;
    // Initialize logging
    let env = Env::default().filter_or("RUST_LOG", "monotax=info");
    Builder::from_env(env).init();

    let cli = Cli::parse();

    match &cli.command {
        Command::Init { force } => init::init(*force).await?,
        Command::Import { statement, filter } => {
            let incomes = incomes(&Some(statement.clone()), filter)?;
            let imported = db::save_all(&incomes.into_iter().collect::<Vec<_>>())?;
            log::info!("Imported {} incomes", imported);
        }

        Command::Taxer {
            input,
            output,
            filter,
        } => {
            let config = config::load_config()?;
            let incomes = incomes(input, filter)?;
            let writer = writer(output)?;
            taxer::export_csv(incomes, config.taxer(), writer)?;
        }
        Command::Report {
            input,
            format,
            output,
            filter,
        } => {
            let config = config::load_config()?;
            let incomes = incomes(input, filter)?;
            let report = QuarterlyReport::build_report(incomes, config.tax());
            let writer = writer(output)?;
            match format {
                ReportFormat::Console => report::console::pretty_print(&report, writer)?,
                ReportFormat::Csv => report::csv::render_csv(&report, writer)?,
            };
        }
        Command::Payments { command } => match command {
            PaymentCommands::Report { output, filter } => {
                let payments = find_payments_by_criteria(filter.criteria())?;
                let report = PaymentReport::from_payments(payments);
                let writer = writer(output)?;
                plaintext_report(&report, writer)?;
            }
            PaymentCommands::Pay { payment_no } => {
                mark_paid(*payment_no)?;
            }
            PaymentCommands::Unpay { payment_no } => {
                mark_unpaid(*payment_no)?;
            }
        },
    }

    Ok(())
}

fn writer(output: &Option<PathBuf>) -> anyhow::Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(stdout())),
    };
    Ok(writer)
}

fn incomes(
    input: &Option<PathBuf>,
    filter: &FilterArgs,
) -> anyhow::Result<impl IntoIterator<Item = Income>> {
    let incomes = match input {
        Some(stmt) => {
            let file = File::open(stmt).context("opening input file")?;
            universalbank::read_incomes(file, filter.criteria())?
        }
        None => db::find_by_criteria(filter.criteria())?,
    };
    Ok(incomes)
}
